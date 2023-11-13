# Architecture

Fist, the config file is deserialized into a Config structure.
The Config struct is then used to generate a Graph where each node is an item of the config.
I.e: Control, Temp, Linear, etc...
Each node have an unique ID, and is stored in a hashmap. They implement an update fonction, that take inputs value and return a value.
A node store a ref (the node id) of it's dependencies.

We can easily pass from the graph to a Config structure to serialize it.
