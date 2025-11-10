# Connection ([Backend](../backend.md))

A type representing a connection from a target to an origin. Needs to contain the following:

1. The instance id of the node instance the connection originates from
2. The socket slot of the socket within the above node instance the connection originates from
3. The type identifier of data being carried by the connection

The socket slot is an arbitrary 32 bit integer that is unique within its node space. Memory footprint is minimised by assigning socket ids via usage tracking - utilising the next lowest possible id on socket change or creation.

While the above contents can be directly parsed from the node file format, it may be beneficial if it also contained the instance id and socket slot of the target node instance it is stored in. Further connection data may be entirely stored in a separate data structure outside of the socket data for performance considerations.

## Requires

- [Type](./type.md)

## Required By

- [Node Parsing](../node_file_format/parsing.md)
- [Socket](./socket.md)
