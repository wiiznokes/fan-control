using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using LibreHardwareMonitorWrapper.Lhm;

namespace LibreHardwareMonitorWrapper;

public class HardwareManager
{
    private readonly List<BaseHardware> _hardwareList;

    private readonly HardwareResearcher _hardwareResearcher = new();


    public HardwareManager()
    {
        _hardwareResearcher.Start();
        _hardwareList = _hardwareResearcher.GetHardwareList();
    }

    public void Update()
    {
        _hardwareResearcher.Update();
    }

    public int GetValue(int index)
    {
        var hardware = _hardwareList[index];
        return hardware.Type switch
        {
            HardwareType.Control => (hardware as Control)!.Value(),
            HardwareType.Fan => (hardware as Sensor)!.Value(),
            HardwareType.Temp => (hardware as Sensor)!.Value(),
            _ => throw new ArgumentOutOfRangeException(nameof(hardware.Type), hardware.Type, "Unknown hardware type")
        };
    }

    public void SetValue(int index, int value)
    {
        var control = _hardwareList[index] as Control;
        control!.SetSpeed(value);
    }

    public void SetAuto(int index)
    {
        var control = _hardwareList[index] as Control;
        control!.SetAuto();
    }

    public void Stop()
    {
        SetAllAuto();
        _hardwareResearcher.Stop();
    }

    public string HardwareListToJson()
    {
        var serializerOptions = new JsonSerializerOptions
        {
            Converters =
            {
                new JsonStringEnumConverter()
            }
        };
        var jsonText = JsonSerializer.Serialize(_hardwareList, serializerOptions);

        var stringBuilder = new StringBuilder(jsonText);
        stringBuilder.Append('\n');
        return stringBuilder.ToString();
    }

    private void SetAllAuto()
    {
        foreach (var control in _hardwareList)
        {
            (control as Control)?.SetAuto();
        }
    }
}