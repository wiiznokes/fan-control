namespace LibreHardwareMonitorWrapper;

public static class Logger
{
    public static bool ShowDebug { private get; set; }


    public static void Debug(string str)
    {
        if (ShowDebug) Console.WriteLine("[DEBUG LHM] " + str);
    }

    public static void Info(string str)
    {
        if (ShowDebug) Console.WriteLine("[INFO LHM] " + str);
    }

    public static void Error(string str)
    {
        Console.Error.WriteLine("[ERROR LHM] " + str);
    }
}