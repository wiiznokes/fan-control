using System.Net;
using System.Net.Sockets;
using System.Text;

namespace LibreHardwareMonitorWrapper;

public class Server
{
    private const string Address = "127.0.0.1";
    private readonly byte[] _buffer = new byte[4];
    private readonly Socket _client;
    private readonly Socket _listener;
    private int _port = 55555;

    public Server()
    {
        _listener = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
        BindPort();
        _listener.Listen(1);
        Console.WriteLine("Server Started");
        _client = _listener.Accept();
        Console.WriteLine("Client Connected");
    }

    public void SendHardware(string jsonText)
    {
        var stringBuilder = new StringBuilder(jsonText);
        stringBuilder.Append('\n');

        var jsonTextWithLineDelimiter = stringBuilder.ToString();

        var stream = new NetworkStream(_client);
        var bytes = Encoding.UTF8.GetBytes(jsonTextWithLineDelimiter);
        Console.WriteLine("Sending hardware");
        stream.Write(bytes);
        stream.Close();
        Console.WriteLine("Hardware send");
    }

    public void WaitForCommand()
    {
        while (true)
        {
            Console.WriteLine("waiting for commands");
            var res = block_read();
            if (res < 0) return;

            var command = (Command)res;
            Console.WriteLine("Receive command: " + command);

            int value;
            int index;
            switch (command)
            {
                case Command.SetAuto:
                    index = block_read();
                    if (index < 0) return;
                    HardwareManager.SetAuto(index);
                    break;
                case Command.SetValue:
                    index = block_read();
                    if (index < 0) return;
                    value = block_read();
                    if (value < 0) return;
                    HardwareManager.SetValue(index, value);
                    break;
                case Command.GetValue:
                    index = block_read();
                    if (index < 0) return;
                    Console.WriteLine("Receive index: " + index);
                    value = HardwareManager.GetValue(index);
                    var bytes = BitConverter.GetBytes(value);
                    Console.WriteLine("sending value: " + value);
                    if (!block_send(bytes)) return;
                    Console.WriteLine("value send");
                    break;
                case Command.Shutdown:
                    return;
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
            Console.WriteLine("byte read = " + bytesRead);
            return -1;
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            return -1;
        }
    }

    private void BindPort()
    {
        var p = _port;
        for (; p <= 65535; p++)
        {
            try
            {
                _listener.Bind(new IPEndPoint(IPAddress.Parse(Address), p));
            }
            catch (SocketException e)
            {
                Console.WriteLine("SelectPort: port " + p + " invalid, " + e.Message);
                continue;
            }
            catch (ObjectDisposedException e)
            {
                Console.WriteLine("SelectPort: listener has been disposed " + e.Message);
                break;
            }

            Console.WriteLine("SelectPort: valid port " + p);
            break;
        }

        if (p > 65535)
            throw new ArgumentException("No valid port can be found for " + Address);
        _port = p;
    }


    public void Shutdown()
    {
        _client.Dispose();
        _client.Close();
        _listener.Dispose();
        _listener.Close();
        Console.WriteLine("Shutdown");
    }
}