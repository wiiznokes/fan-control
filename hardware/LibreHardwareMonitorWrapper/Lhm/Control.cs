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

    public bool SetSpeed(int value)
    {
        if (_mSensor.Control != null)
        {
            _mSensor.Control.SetSoftware(value);
            _isSetSpeed = true;
        }
        else
        {
            return false;
        }

        Log.LogD("set control: " + Name + " = " + value);
        return true;
    }

    public bool SetAuto()
    {
        if (_mSensor.Control == null) return false;

        if (_isSetSpeed == false)
            return true;

        _mSensor.Control.SetDefault();
        _isSetSpeed = false;
        Log.LogD("set control to auto: " + Name);
        return true;
    }
}