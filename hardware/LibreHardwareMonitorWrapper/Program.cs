using Microsoft.Win32;

namespace LibreHardwareMonitorWrapper;

internal static class Program
{

    private static HardwareManager _hardwareManager = null!;
    private static Server _server = null!;

    private static bool _isServerStarted;
    private static bool _isHardwareManagerStarted;
    
    private static readonly ShutdownManager ShutdownManager = new();

    private static void ShutDown()
    {
        if (!ShutdownManager.SafeTakeShutdown()) return;
        if (_isServerStarted)
            _server.Shutdown();
        if (_isHardwareManagerStarted)
            _hardwareManager.Stop();
    }
    
    private static async Task<int> Main(string[] args)
    {
        SetupLog(args);

        
        var connectCts = new CancellationTokenSource();
        var connectTask = Task.Run(() =>
        {
            _server =  new Server();
            _isServerStarted = true;
        }, connectCts.Token);
      
        try
        {
            _hardwareManager = new HardwareManager();
            _isHardwareManagerStarted = true;
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


        string jsonText;
        try
        {
            jsonText = _hardwareManager.ToJson();
        }
        catch (Exception e)
        {
            Logger.Error("Can't serialize hardware to json: " + e.Message);
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

        if (!_isServerStarted || !_isHardwareManagerStarted)
        {
            Logger.Error("Weird state: server started: " + _isServerStarted + ", hardware manager started: " + _isHardwareManagerStarted);
            ShutDown();
            return 1;
        }
        
        try
        {
            _server.SendHardware(jsonText);
        }
        catch (Exception e)
        {
            Logger.Error("can't send hardware to the app" + e.Message);
            ShutDown();
            return 1;
        }
   
        
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
        
        
        try
        {
            _server.WaitForCommand(_hardwareManager);
        }
        catch (Exception e)
        {
            Logger.Error("can't wait for commands" + e.Message);
            ShutDown();
            return 1;
        }
        
        ShutDown();
        return 0;
    }
    
    private static void SetupLog(string[] args)
    {
        // should log to a file part
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
    
        // level part
        if (args.Contains("--log=debug"))
        {
            Logger.LogLevel = LogLevel.Debug;
        }
        else
        if (args.Contains("--log=info"))
        {
            Logger.LogLevel = LogLevel.Info;
        }
    
    }
}







internal class ShutdownManager
{
    private readonly object _shutdownLock = new();
    private bool _isShutdown;

    public bool SafeTakeShutdown()
    {
        lock (_shutdownLock)
        {
            if (_isShutdown) return true;
            _isShutdown = true;
            return false;
        }
    }
}