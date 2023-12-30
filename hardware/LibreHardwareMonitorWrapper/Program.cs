using LibreHardwareMonitorWrapper;
using Microsoft.Win32;


Logger.LogToFile = true;

if (args.Contains("--log=info"))
{
    Logger.LogLevel = LogLevel.Info;
}
else if (args.Contains("--log=debug"))
{
    Logger.LogLevel = LogLevel.Debug;
}

var connectTask = Task.Run(() => new Server());

var hardwareManager = new HardwareManager();
var jsonText = hardwareManager.ToJson();

var server = await connectTask;

Console.CancelKeyPress += (sender, e) =>
{
    Logger.Info("On canceled process");
    server.Shutdown();
    hardwareManager.Stop();
};

SystemEvents.SessionEnding += (sender, e) =>
{
    Logger.Info("On disconnected session");
    server.Shutdown();
    hardwareManager.Stop();
};


server.SendHardware(jsonText);

server.WaitForCommand(hardwareManager);
server.Shutdown();
hardwareManager.Stop();