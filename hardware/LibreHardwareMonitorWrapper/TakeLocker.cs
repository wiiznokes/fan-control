namespace LibreHardwareMonitorWrapper;

public class TakeLocker
{
    private readonly object _lock = new();
    private bool _isTaken;

    public bool SafeTake()
    {
        lock (_lock)
        {
            if (_isTaken) return false;
            _isTaken = true;
            return true;
        }
    }
}