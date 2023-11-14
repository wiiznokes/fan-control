using System.Text.Json;
using System.Text.Json.Serialization;
using LibreHardwareMonitorWrapper;

HardwareManager.Start();


var serializerOptions = new JsonSerializerOptions
{
    Converters =
    {
        new JsonStringEnumConverter()
    }
};


var jsonText = JsonSerializer.Serialize(State.Hardwares, serializerOptions);

var server = new Server();

server.SendHardware(jsonText);

server.WaitForCommand();

server.Shutdown();
HardwareManager.Stop();