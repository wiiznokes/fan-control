


using LibreHardwareMonitorWrapper;
using LibreHardwareMonitorWrapper.Hardware;




var server = new Server();

server.WaitForCommand();

server.JustWait();

server.Shutdown();