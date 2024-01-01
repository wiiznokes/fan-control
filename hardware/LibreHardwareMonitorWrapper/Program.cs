using LibreHardwareMonitorWrapper;
using Microsoft.Win32;



HardwareManager hardwareManager = null!;
Server server = null!;

var isServerStarted = false;
var isHardwareManagerStarted = false;


TakeLocker shutDownTakeLocker = new();


SetupLog();


Console.CancelKeyPress += (_, _) =>
{
    Logger.Info("On canceled process");
    ShutDown();
};

SystemEvents.SessionEnding += (_, _) =>
{
    Logger.Info("On disconnected session");
    ShutDown();
};


var connectCts = new CancellationTokenSource();
var connectTask = Task.Run(() =>
{
    server = new Server();
    isServerStarted = true;
}, connectCts.Token);

try
{
    hardwareManager = new HardwareManager();
    isHardwareManagerStarted = true;
}
catch (Exception e)
{
    Logger.Error("Can't start hardware manager: " + e.Message);
    try
    {
        connectCts.Cancel();
        await connectTask;
    }
    catch (Exception)
    {
        Logger.Error("Cancel server task: " + e.Message);
    }
    ShutDown();
    return 1;
}

try
{
    await connectTask;
}
catch (Exception e)
{
    Logger.Error("Can't start server : " + e.Message);
    ShutDown();
    return 1;
}

if (!isServerStarted || !isHardwareManagerStarted)
{
    Logger.Error("Weird state: server started: " + isServerStarted + ", hardware manager started: " + isHardwareManagerStarted);
    ShutDown();
    return 1;
}


try
{
    Logger.Info("start waiting for commands");
    server.WaitAndHandleCommands(hardwareManager);
}
catch (Exception e)
{
    Logger.Error("can't wait for commands" + e.Message);
    ShutDown();
    return 1;
}

ShutDown();
return 0;



void ShutDown()
{
    if (!shutDownTakeLocker.SafeTake()) return;

    Logger.Debug("Shutdown");

    if (isServerStarted)
        server.Shutdown();

    // the warning is because Console.CancelKeyPress use this function
    // but this seems to works as expected
    if (isHardwareManagerStarted)
        hardwareManager.Stop();
}


void SetupLog()
{
    LogToFile();

    if (args.Contains("--log=debug"))
    {
        Logger.LogLevel = LogLevel.Debug;
    }
    else
    if (args.Contains("--log=info"))
    {
        Logger.LogLevel = LogLevel.Info;
    }

    return;

    void LogToFile()
    {
        try
        {
            var maybeLogFilePath = Environment.GetEnvironmentVariable("FAN_CONTROL_LOG_FILE");
            if (maybeLogFilePath == null) return;
            var logFileNameWithoutExtension = Path.GetFileNameWithoutExtension(maybeLogFilePath);
            var logFileName = logFileNameWithoutExtension + "-lhm.txt";
            var lhmLogFilePath = Path.Combine(Path.GetDirectoryName(maybeLogFilePath) ?? throw new InvalidOperationException(), logFileName);
            Logger.LogToFile(lhmLogFilePath);
        }
        catch (Exception)
        {
            // ignored
        }
    }
}