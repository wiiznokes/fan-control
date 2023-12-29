namespace LibreHardwareMonitorWrapper;

public static class Logger
{
    private const string Tag = "[LHM] ";

    public static bool ShowDebug { private get; set; }


    public static void Debug(string str)
    {
        if (ShowDebug) Console.WriteLine(Tag + str);
    }

    public static void Info(string str)
    {
        Console.WriteLine(Tag + str);
    }

    public static void Error(string str)
    {
        Console.Error.WriteLine(Tag + "error: " + str);
    }
}