# Connection ([Frontend](../../frontend.md))

A type representing a connection from a target to an origin. Needs to contain the following:

1. The instance id of the node instance the connection originates from
2. The socket slot of the socket within the above node instance the connection originates from
3. The type identifier of data being carried by the connection

The socket slot is an arbitrary 32 bit integer that is unique within its node space. Memory footprint is minimised by assigning socket ids via usage tracking - utilising the next lowest possible id on socket change or creation.

While the above contents can be directly parsed from the node file format, it may be beneficial if it also contained the instance id and socket slot of the target node instance it is stored in. Further connection data may be entirely stored in a separate data structure outside of the socket data for performance considerations.

Rendering of connections should include a mechanism for adding [*sub-pins*](./pin.md) to more precisely control how the connection is visualised. Sub-pins act like socket-associated pins, but are freely dragable in a node space. Connections can be associated with an ordered list of pins. If such a list exists, the connection is to be rendered as line segments connecting the pins in the specified order. If no sub-pins exist, the connection is to be rendered in the way defined by the project's [settings](../../user_interface/settings/settings.md).

Feature Idea:
Initialising a connection via socket pin and subsequently terminating via the grid while holding a certain [hotkey](../../user_input/hotkeys/hotkeys.md) could trigger the creation of a new node instance. The list of node instances could be filtered down to those that can provide a valid connection endpoint.

## Requires

- [Type](./type.md)

## Required By

- [Socket](./socket.md)
