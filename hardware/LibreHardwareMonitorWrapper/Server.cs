using System.Net;
using System.Net.Sockets;
using System.Text;

namespace LibreHardwareMonitorWrapper;

public enum Command
{
    SetAuto = 1,
    SetValue = 2,
    GetValue = 3,
    Shutdown = 4,
    Update = 5
}

public class Server
{
    private const string Address = "127.0.0.1";
    private const int DefaultPort = 55555;
    private const string Check = "fan-control-check";
    private const string CheckResponse = "fan-control-ok";
    private readonly Socket _client;
    private readonly Socket _listener;
    private readonly byte[] _buffer = new byte[4];
    
    public Server()
    {
        _listener = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
        
        StartServer();
        _client = AcceptClient();
        
        //listener.Dispose();
        //listener.Close();
    }
    
    

    public void SendHardware(string jsonText)
    {
        
        var bytes = Encoding.UTF8.GetBytes(jsonText);
        Logger.Debug("Sending hardware");
        block_send(bytes);
        Logger.Debug("Hardware send");
    }

    public void WaitForCommand(HardwareManager hardwareManager)
    {
        while (true)
        {
            var res = block_read();

            var command = (Command)res;

            int value;
            int index;
            switch (command)
            {
                case Command.SetAuto:
                    index = block_read();
                    hardwareManager.SetAuto(index);
                    break;
                case Command.SetValue:
                    index = block_read();
                    value = block_read();
                    hardwareManager.SetValue(index, value);
                    break;
                case Command.GetValue:
                    index = block_read();
                    value = hardwareManager.GetValue(index);
                    var bytes = BitConverter.GetBytes(value);
                    block_send(bytes);
                    break;
                case Command.Shutdown:
                    return;
                case Command.Update:
                    hardwareManager.Update();
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(command), command, "Unknown command");
            }
        }
    }

    

    private void StartServer()
    {
        var p = DefaultPort;
        for (; p <= 65535; p++)
        {
            try
            {
                _listener.Bind(new IPEndPoint(IPAddress.Parse(Address), p));
                _listener.Listen(1);
            }
            catch (SocketException e)
            {
                Logger.Error("SelectPort: port " + p + " invalid, " + e);
                continue;
            }
            catch (ObjectDisposedException e)
            {
                Logger.Error("SelectPort: listener has been disposed " + e.Message);
                break;
            }

            Logger.Info("Server Started on " + Address + ":" + p);
            return;
        }

        throw new ArgumentException("No valid port can be found for " + Address);

    }


    // return client
    private Socket AcceptClient()
    {
        var client = _listener.Accept();
        var checkBytes = Encoding.UTF8.GetBytes(Check);
        var readBuf = new byte[checkBytes.Length];
        var res = client.Receive(readBuf);

        var str = Encoding.UTF8.GetString(readBuf);
        if (str != Check)
        {
            throw new Exception("Invalid client. Check : " + str + "byte received: " + res);
        }

        client.Send(Encoding.UTF8.GetBytes(CheckResponse));

        Logger.Info("Client accepted.");
        return client;
    }
    
    private void block_send(byte[] bytes)
    {
        var bytesSend = _client.Send(bytes);
        if (bytesSend != bytes.Length)
            throw new InvalidDataException("byte send " + bytesSend + " != byte to send " + bytes.Length);
    }

    private int block_read()
    {
        var bytesRead = _client.Receive(_buffer);
        if (bytesRead != _buffer.Length)
            throw new InvalidDataException("byte read " + bytesRead + " != " + _buffer.Length);
        
        return BitConverter.ToInt32(_buffer, 0);
    }
    
    public void Shutdown()
    {
        _client.Dispose();
        _client.Close();
        _listener.Dispose();
        _listener.Close();
        
        Logger.Info("Shutdown server.");
    }
}