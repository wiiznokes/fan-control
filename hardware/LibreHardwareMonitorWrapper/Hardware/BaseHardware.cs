namespace LibreHardwareMonitorWrapper.Hardware;

public abstract class BaseHardware
{
    protected BaseHardware(string id, string name, int index, HardwareType type)
    {
        Id = id;
        Name = name;
        Index = index;
        Type = type;
    }

    public string Id { get; }
    public string Name { get; }
    public int Index { get; }


    public HardwareType Type { get; }
}