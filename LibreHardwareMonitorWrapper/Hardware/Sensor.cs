using LibreHardwareMonitor.Hardware;

namespace LibreHardwareMonitorWrapper.Hardware;

public class Sensor : BaseHardware
{
    private readonly ISensor _mSensor;

    public Sensor(string id, string name, ISensor sensor, int index, HardwareType type) : base(id, name, index, type)
    {
        _mSensor = sensor;
    }

    public int Value()
    {
        return _mSensor.Value.HasValue ? (int)_mSensor.Value : 0;
    }
}