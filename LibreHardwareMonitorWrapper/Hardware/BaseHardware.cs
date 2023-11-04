namespace LibreHardwareMonitorWrapper.Hardware;

public abstract class BaseHardware
{
    protected BaseHardware(string id, string name, int index)
    {
        Id = id;
        Name = name;
        Index = index;
    }

    public string Id { get; }
    public string Name { get; }
    public int Index { get; }

   
}