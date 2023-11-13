using System.Net;
using System.Net.Sockets;
using System.Text;
using LibreHardwareMonitorWrapper.Hardware;

namespace LibreHardwareMonitorWrapper;

public class Server
{
    private const string Address = "127.0.0.1";
    private readonly Socket _client;
    private readonly Socket _listener;
    private int _port = 55555;
    private readonly byte[] _buffer = new byte[1024];

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
        var stream = new NetworkStream(_client);
        var bytes = Encoding.UTF8.GetBytes(jsonText);
        Console.WriteLine("Sending hardware" + jsonText);
        stream.Write(bytes);
        Console.WriteLine("Hardware send");
    }

    public void WaitForCommand()
    {
        while (true)
        {
            Console.WriteLine("waiting for commands");
            if (!block_read()) return;
            var command = (Command)BitConverter.ToInt32(_buffer, 0);
            Console.WriteLine("Receive command: " + command);
            
            int value;
            int index;
            switch (command)
            {
                case Command.SetAuto:
                    if (!block_read()) return;
                    index = BitConverter.ToInt32(_buffer, 0);
                    State.Controls[index].SetAuto();
                    break;
                case Command.SetValue:
                    if (!block_read()) return;
                    index = BitConverter.ToInt32(_buffer, 0);
                    if (!block_read()) return;
                    value = BitConverter.ToInt32(_buffer, 0);
                    State.Controls[index].SetSpeed(value);
                    break;
                case Command.GetValue:
                    if (!block_read()) return;
                    index = BitConverter.ToInt32(_buffer, 0);
                    if (!block_read()) return;
                    var type = (HardwareType)BitConverter.ToInt32(_buffer, 0);

                    value = type switch
                    {
                        HardwareType.Fan => State.Fans[index].Value(),
                        HardwareType.Temp => State.Temps[index].Value(),
                        _ => throw new ArgumentOutOfRangeException()
                    };

                    var bytes = BitConverter.GetBytes(value);
                    if (!block_send(bytes)) return;
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

    private bool block_read()
    {
        try
        {
            var bytesRead = _client.Receive(_buffer);
            return bytesRead != 0;
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            return false;
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