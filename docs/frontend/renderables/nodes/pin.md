# Pin ([Frontend](../../frontend.md))

An origin, midpoint or target of a [connection](./connection.md). Mostly associated with a [socket](./socket.md), but can be created as a *sub-pin* to better control connection rendering. If not associated with a socket, should be dragable.

Should change appearance if carried data is *packaged*, to more easily identify the two different forms of data. Connection sub-pins should also be visually distinct.

## Requires

- [Dragable](../../properties/dragable/dragable.md)

## Required By

- [Socket](./socket.md)
