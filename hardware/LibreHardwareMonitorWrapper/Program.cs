using LibreHardwareMonitorWrapper;
using Microsoft.Win32;


try
{
    var maybeLogFilePath = Environment.GetEnvironmentVariable("FAN_CONTROL_LOG_FILE");
    if (maybeLogFilePath != null)
    {
        var logFileNameWithoutExtension = Path.GetFileNameWithoutExtension(maybeLogFilePath);
        var logFileName = logFileNameWithoutExtension + "-lhm.txt";

        var lhmLogFilePath = Path.Combine(Path.GetDirectoryName(maybeLogFilePath) ?? throw new InvalidOperationException(), logFileName);

        Logger.LogToFile(lhmLogFilePath);
    }
}
catch (Exception)
{
    // ignored
}


if (args.Contains("--log=debug"))
{
    Logger.LogLevel = LogLevel.Debug;
}
else
if (args.Contains("--log=info"))
{
    Logger.LogLevel = LogLevel.Info;
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