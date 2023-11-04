using LibreHardwareMonitor.Hardware;

namespace LibreHardwareMonitorWrapper.Hardware;

public class Lhm : IVisitor
{
    private readonly Computer _mComputer = new()
    {
        IsCpuEnabled = true,
        IsMotherboardEnabled = true,
        IsControllerEnabled = true,
        IsGpuEnabled = true,
        IsStorageEnabled = true,
        IsMemoryEnabled = true
    };

    private bool _isStarted;

    /////////////////////////// Visitor ///////////////////////////
    public void VisitComputer(IComputer computer)
    {
        computer.Traverse(this);
    }

    public void VisitHardware(IHardware hardware)
    {
        hardware.Update();
        foreach (var subHardware in hardware.SubHardware)
            subHardware.Accept(this);
    }

    public void VisitSensor(ISensor sensor)
    {
    }

    public void VisitParameter(IParameter parameter)
    {
    }

    public void Start()
    {
        if (_isStarted)
            return;

        _mComputer.Open();
        _mComputer.Accept(this);

        _isStarted = true;
    }

    public void Stop()
    {
        if (!_isStarted)
            return;

        _mComputer.Close();
        _isStarted = false;
    }


    public void CreateHardware()
    {
        if (!_isStarted)
            return;

        var nbControl = 0;
        var nbFan = 0;
        var nbTemp = 0;


        var hardwareArray = _mComputer.Hardware;
        foreach (var hardware in hardwareArray)
        {
            var sensorArray = hardware.Sensors;
            foreach (var sensor in sensorArray)
            {
                AddHardware(sensor);
            }

            var subHardwareArray = hardware.SubHardware;
            foreach (var subHardware in subHardwareArray)
            {
                var subSensorArray = subHardware.Sensors;
                foreach (var subSensor in subSensorArray)
                {
                    AddHardware(subSensor);
                }
            }
        }

        return;

        void AddHardware(ISensor sensor)
        {
            if (sensor.SensorType != SensorType.Control && sensor.SensorType != SensorType.Temperature &&
                sensor.SensorType != SensorType.Fan)
                return;

            var id = sensor.Identifier.ToString();
            var name = sensor.Name.Length > 0 ? sensor.Name : sensor.SensorType.ToString();
            switch (sensor.SensorType)
            {
                case SensorType.Control:
                    id ??= SensorType.Control.ToString() + nbControl;
                    State.Controls.Add(new Control(id, name, sensor, nbControl));
                    nbControl += 1;
                    break;
                case SensorType.Fan:
                    id ??= SensorType.Control.ToString() + nbFan;
                    State.Fans.Add(new Sensor(id, name, sensor, nbFan));
                    nbFan += 1;
                    break;
                case SensorType.Temperature:
                    id ??= SensorType.Control.ToString() + nbTemp;
                    State.Temps.Add(new Sensor(id, name, sensor, nbTemp));
                    nbTemp += 1;
                    break;

                default: throw new Exception("wrong sensor type");
            }
        }
    }

    public void Update()
    {
        _mComputer.Accept(this);
    }
}