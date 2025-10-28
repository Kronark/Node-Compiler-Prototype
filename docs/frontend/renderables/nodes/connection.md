# Connection ([Frontend](../../frontend.md))

A type representing a connection from a target to an origin. Needs to contain the following:

1. The instance id of the node instance the connection originates from
2. The socket slot within the node instance the connection originates from
3. The type identifier of data being carried by the connection

While the above contents can be directly parsed from the node file format, it may be beneficial if it also contained the instance id and socket slot of the target node instance it is stored in. Further connection data may be entirely stored in a separate data structure outside of the socket data for performance considerations.

Rendering of connections should include a mechanism for adding [*sub-pins*](./pin.md) to more precisely control how the connection is visualised. Sub-pins act like socket-associated pins, but are freely dragable in a node space. Connections can be associated with an ordered list of pins. If such a list exists, the connection is to be rendered as line segments connecting the pins in the specified order. If no sub-pins exist, the connection is to be rendered in the way defined by the project's [settings](../../user_interface/settings/settings.md).

## Requires

- [Type](./type.md)

## Required By

- [Socket](./socket.md)
