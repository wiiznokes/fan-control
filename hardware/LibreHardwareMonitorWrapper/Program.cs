using LibreHardwareMonitorWrapper;





if (args.Contains("--log"))
{
    Logger.ShowDebug = true;
}

var connectTask = Task.Run(() => new Server());

var hardwareManager = new HardwareManager();
var jsonText = hardwareManager.ToJson();

var server = await connectTask;

Console.CancelKeyPress += (sender, e) =>
{
    Logger.Info("Exit signal captured");
    server.Shutdown();
    hardwareManager.Stop();
};

server.SendHardware(jsonText);

server.WaitForCommand(hardwareManager);
server.Shutdown();
hardwareManager.Stop();