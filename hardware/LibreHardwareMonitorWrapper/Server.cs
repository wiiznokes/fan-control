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
    private readonly int _port;
    private readonly byte[] _buffer = new byte[4];

    public Server()
    {
        _listener = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
        _port = StartServer();
        _client = AcceptClient();
    }

    public void SendHardware(string jsonText)
    {

        var stream = new NetworkStream(_client);
        var bytes = Encoding.UTF8.GetBytes(jsonText);
        Log.LogD("Sending hardware");
        stream.Write(bytes);
        stream.Close();
        Log.LogD("Hardware send");
    }

    public void WaitForCommand(HardwareManager hardwareManager)
    {
        while (true)
        {
            var res = block_read();
            if (res < 0) return;

            var command = (Command)res;

            int value;
            int index;
            switch (command)
            {
                case Command.SetAuto:
                    index = block_read();
                    if (index < 0) return;
                    hardwareManager.SetAuto(index);
                    break;
                case Command.SetValue:
                    index = block_read();
                    if (index < 0) return;
                    value = block_read();
                    if (value < 0) return;
                    hardwareManager.SetValue(index, value);
                    break;
                case Command.GetValue:
                    index = block_read();
                    if (index < 0) return;
                    value = hardwareManager.GetValue(index);
                    var bytes = BitConverter.GetBytes(value);
                    if (!block_send(bytes)) return;
                    break;
                case Command.Shutdown:
                    return;
                case Command.Update:
                    hardwareManager.Update();
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }
    }

    private bool block_send(byte[] bytes)
    {
        try
        {
            var bytesSend = _client.Send(bytes);
            return bytesSend != 0;
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            return false;
        }
    }

    // return -1 if error
    private int block_read()
    {
        try
        {
            var bytesRead = _client.Receive(_buffer);
            if (bytesRead == 4) return BitConverter.ToInt32(_buffer, 0);
            Log.LogE("byte read = " + bytesRead);
            return -1;
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            return -1;
        }
    }

    // return port
    private int StartServer()
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
                Log.LogE("SelectPort: port " + p + " invalid, " + e.Message);
                continue;
            }
            catch (ObjectDisposedException e)
            {
                Log.LogE("SelectPort: listener has been disposed " + e.Message);
                break;
            }

            Log.LogD("Server Started on " + Address + ":" + p);
            return p;
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
            throw new Exception("invalid client. Check : " + str + "byte received: " + res);
        }

        client.Send(Encoding.UTF8.GetBytes(CheckResponse));

        Log.LogD("Client accepted!");
        return client;
    }

    public void Shutdown()
    {
        _client.Dispose();
        _client.Close();
        _listener.Dispose();
        _listener.Close();
        Log.LogD("Shutdown server");
    }
}