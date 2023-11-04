using LibreHardwareMonitorWrapper.Hardware;

namespace LibreHardwareMonitorWrapper;

public static class HardwareManager
{
    private static readonly Lhm Lhm = new();


    public static void Start()
    {
        Lhm.Start();
        Lhm.CreateHardware();
    }
    
    public static int GetValue(HardwareType type, int index)
    {
        Lhm.Update();
        return type switch
        {
            HardwareType.Control => State.Controls[index].Value(),
            HardwareType.Fan => State.Fans[index].Value(),
            HardwareType.Temp => State.Temps[index].Value(),
            _ => throw new ArgumentOutOfRangeException(nameof(type), type, null)
        };
    }
    
    public static void SetValue(int index, int value)
    {
        State.Controls[index].SetSpeed(value);
    }
    
    public static void SetAuto(int index)
    {
        State.Controls[index].SetAuto();
    }

    public static void Stop()
    {
        Lhm.Stop();
    }


    public static void Update()
    {
        Lhm.Update();
    }
}