using System.Text.Json;
using System.Text.Json.Serialization;
using LibreHardwareMonitorWrapper;

HardwareManager.Start();

var hardwareList = State.GetHardwareData();

var serializerOptions = new JsonSerializerOptions
{
    Converters =
    {
        new JsonStringEnumConverter()
    }
};


var jsonText = JsonSerializer.Serialize(hardwareList, serializerOptions);

var server = new Server();

server.SendHardware(jsonText);

server.WaitForCommand();

server.Shutdown();