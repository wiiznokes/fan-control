namespace LibreHardwareMonitorWrapper;

internal static class LogLevelManager
{
    public static string ToString(LogLevel level)
    {
        return level switch
        {
            LogLevel.Debug => "DEBUG",
            LogLevel.Info => "INFO ",
            LogLevel.Error => "ERROR",
            _ => throw new ArgumentOutOfRangeException(nameof(level), level, null)
        };
    }
}

public enum LogLevel
{
    Debug,
    Info,
    Error
}

public static class Logger
{
    public static LogLevel LogLevel { private get; set; } = LogLevel.Error;


    private static string? _filePath;

    public static void LogToFile(string filePath)
    {
        File.WriteAllText(filePath, "");
        _filePath = filePath;
    }

    public static void Debug(string str)
    {
        if (LogLevel == LogLevel.Debug)
        {
            Write(LogLevel.Debug, str);
        }
    }

    public static void Info(string str)
    {
        if (LogLevel is LogLevel.Info or LogLevel.Debug)
        {
            Write(LogLevel.Info, str);
        }
    }

    public static void Error(string str)
    {
        Write(LogLevel.Error, str);
    }

    private static void Write(LogLevel level, string log)
    {
        var currentTime = DateTime.Now;
        var currentTimeString = currentTime.ToString("HH:mm:ss");
        var finalLog = "[" + LogLevelManager.ToString(level) + " LHM " + currentTimeString + "] " + log + ".";
        if (_filePath != null) File.AppendAllText(_filePath, finalLog + Environment.NewLine);
        Console.WriteLine(finalLog);
    }
}