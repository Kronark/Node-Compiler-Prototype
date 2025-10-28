# Pin ([Frontend](../../frontend.md))

An origin, midpoint or target of a [connection](./connection.md). Mostly associated with a [socket](./socket.md), but can be created as a *sub-pin* to better control connection rendering. If not associated with a socket, should be dragable.

Should render socket pins according to the [settings](../../user_interface/settings/settings.md) of a project. Connection subpin shapes are picked separately from socket pin shapes, as they are not influenced by a connection's data type.

## Requires

- [Dragable](../../properties/dragable/dragable.md)

## Required By

- [Socket](./socket.md)
