


using LibreHardwareMonitorWrapper;

var server = new Server();

server.WaitForCommand();

server.JustWait();

server.Shutdown();