# Connection ([Frontend](../../frontend.md))

A type representing a connection from a target to an origin. Needs to contain the following:

1. The instance id of the node instance the connection originates from
2. The socket slot within the node instance the connection originates from
3. The type identifier of data being carried by the connection

While the above contents can be directly parsed from the node file format, it may be beneficial if it also contained the instance id and socket slot of the target node instance it is stored in. Further connection data may be entirely stored in a separate data structure outside of the socket data for performance considerations.

## Requires

- [Type](./type.md)

## Required By

- [Socket](./socket.md)
