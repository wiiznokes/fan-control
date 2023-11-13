using LibreHardwareMonitorWrapper.Hardware;

namespace LibreHardwareMonitorWrapper;

public enum HardwareType
{
    Control,
    Fan,
    Temp
}

public static class State
{
    public static readonly List<Control> Controls = new();
    public static readonly List<Sensor> Fans = new();
    public static readonly List<Sensor> Temps = new();

    public static List<BaseHardware> GetHardwareData()
    {
        var list = new List<BaseHardware>();

        list.AddRange(Controls);
        list.AddRange(Fans);
        list.AddRange(Temps);

        return list;
    }
}