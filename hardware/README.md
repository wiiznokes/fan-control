# Architecture

On Linux, we use libsensors to query values of sensors. We use a custom fork with pwm support.

On Windows, Rust will launch a server written in C# in a child process. It will then connect to it, and query all hardwares. Then, we update all value C# side at once with one call, and query specific value. All of this with simple TCP request.

The only internal value exposed is an internal index, used to retreive a specific sensors, in each implementation.