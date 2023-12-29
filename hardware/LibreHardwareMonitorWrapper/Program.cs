using LibreHardwareMonitorWrapper;


if (args.Contains("--log"))
{
    Logger.ShowDebug = true;
}

var connectTask = Task.Run(() => new Server());

var hardwareManager = new HardwareManager();
var jsonText = hardwareManager.ToJson();

var server = await connectTask;

server.SendHardware(jsonText);

server.WaitForCommand(hardwareManager);
server.Shutdown();
hardwareManager.Stop();