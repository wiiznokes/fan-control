using LibreHardwareMonitor.Hardware;

namespace LibreHardwareMonitorWrapper.Lhm;

public class Control : BaseHardware
{
    private readonly ISensor _mSensor;
    private bool _isSetSpeed;

    public Control(string id, string name, string info, ISensor sensor, int index) : base(id, name, info, index,
        HardwareType.Control)
    {
        _mSensor = sensor;
    }

    public int Value()
    {
        double temp = 0.0f;
        if (_mSensor.Value != null) temp = (double)_mSensor.Value;
        temp = Math.Round(temp);
        return (int)temp;
    }

    public int GetMinSpeed()
    {
        if (_mSensor.Control != null) return (int)_mSensor.Control.MinSoftwareValue;
        return 0;
    }

    public int GetMaxSpeed()
    {
        if (_mSensor.Control != null) return (int)_mSensor.Control.MaxSoftwareValue;
        return 100;
    }

    public void SetSpeed(int value)
    {
        _mSensor.Control.SetSoftware(value);
        _isSetSpeed = true;

        Logger.Debug("Set control: " + Name + " = " + value);
    }

    public void SetAuto()
    {
        if (_isSetSpeed == false)
        {
            Logger.Debug("Control already set to auto: " + Name);
            return;
        }

        _mSensor.Control.SetDefault();
        _isSetSpeed = false;
        Logger.Debug("Set control to auto: " + Name);
    }
}