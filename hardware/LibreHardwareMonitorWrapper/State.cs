namespace LibreHardwareMonitorWrapper;

public static class State
{
    public static readonly List<BaseHardware> Hardwares = new();
}

public enum HardwareType
{
    Control = 1,
    Fan = 2,
    Temp = 3
}

public enum Command
{
    SetAuto = 1,
    SetValue = 2,
    GetValue = 3,
    Shutdown = 4
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