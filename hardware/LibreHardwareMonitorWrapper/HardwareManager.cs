using LibreHardwareMonitorWrapper.Hardware;
using LibreHardwareMonitorWrapper.Lhm;

namespace LibreHardwareMonitorWrapper;

public static class HardwareManager
{
    private static readonly HardwareResearcher HardwareResearcher = new();


    public static void Start()
    {
        HardwareResearcher.Start();
        HardwareResearcher.CreateHardware();
    }

    public static int GetValue(int index)
    {
        HardwareResearcher.Update();
        var hardware = State.Hardwares[index];
        return hardware.Type switch
        {
            HardwareType.Control => (hardware as Control)!.Value(),
            HardwareType.Fan => (hardware as Sensor)!.Value(),
            HardwareType.Temp => (hardware as Sensor)!.Value(),
            _ => throw new ArgumentOutOfRangeException()
        };
    }

    public static void SetValue(int index, int value)
    {
        var control = State.Hardwares[index] as Control;
        control!.SetSpeed(value);
    }

    public static void SetAuto(int index)
    {
        var control = State.Hardwares[index] as Control;
        control!.SetAuto();
    }

    public static void Stop()
    {
        HardwareResearcher.Stop();
    }


    public static void Update()
    {
        HardwareResearcher.Update();
    }
}