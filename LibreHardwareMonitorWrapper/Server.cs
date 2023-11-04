using System.Net;
using System.Net.Sockets;
using System.Text;

namespace LibreHardwareMonitorWrapper
{
    public class Server
    {
        private readonly Socket _listener;
        private readonly Socket _client;
        private const string Address = "127.0.0.1";
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


        public void WaitForCommand()
        {
            var buffer = new byte[1024];
            var bytesRead = _client.Receive(buffer);
            
            var message = Encoding.ASCII.GetString(buffer, 0, bytesRead);
            Console.WriteLine("Received: " + message);

            // var responseBuffer = Encoding.ASCII.GetBytes("Hello from C#");
            var responseBuffer = "Hello from C#"u8.ToArray();
            _client.Send(responseBuffer);
        }
        
        public void JustWait()
        {
            var buffer = new byte[1024];
            var bytesRead = _client.Receive(buffer);
            
            var message = Encoding.ASCII.GetString(buffer, 0, bytesRead);
            Console.WriteLine("Received: " + message);

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
}