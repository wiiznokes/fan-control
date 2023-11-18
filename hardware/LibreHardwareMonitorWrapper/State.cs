namespace LibreHardwareMonitorWrapper;

public static class Log
{
    private const string Tag = "[LHM] ";

    public static void LogD(string str)
    {
        Console.WriteLine(Tag + str);
    }

    public static void LogE(string str)
    {
        Console.WriteLine(Tag + "error: " + str);
    }
}

public enum HardwareType
{
    Control = 1,
    Fan = 2,
    Temp = 3
}

public abstract class BaseHardware
{
    protected BaseHardware(string id, string name, string info, int index, HardwareType type)
    {
        Id = id;
        Name = name;
        Index = index;
        Type = type;
        Info = info;
    }

    public string Id { get; }
    public string Name { get; }
    public string Info { get; }
    public int Index { get; }
    public HardwareType Type { get; }
}