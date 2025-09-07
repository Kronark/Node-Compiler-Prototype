# Broadcast Node ([Backend](../../backend.md))

A compiler node allowing users to configure sockets inside of a custom node space, but render them in a grandparent node instance instead of the default parent instance level. A broadcast node's sockets are defined as follows:

___

1. Switch Input - Unnamed<br>
    - Inactive: "Output"
    - Active: "Input" (Default)

If *1* is *active*:<br>

2. Named Input - "Broadcast"

Else If *1* is *inactive*:<br>

2. Named Output - "Broadcast"

End If

3. Text Input - "Name"
    - Minimum: "0"
    - Maximum: ""
    - Valid: ""
    - Default: "Broadcast"

If *1* is *active*:<br>

4. Named Output - "Channel"
    - *Notes*:
        - Needs to prevent multiple input port connections either by compiler error or automatic fix.

Else If *1* is *inactive*:<br>

4. Named Input - "Channel"

End If

___

The broadcast node renders a *named socket* in place of the socket defined by the connected [port node](./port.md). During traversal, this socket is treated like a port's channel socket. This allows users to further propagate a socket to a *greatgranparent node instance* or higher, if desired.

A broadcast node is always connected to the *channel* socket of a [port node](./port.md). This means in case of an *input port*, they are placed to the *left* within a node graph, *before* the socket definition. In case of an *output port*, they are placed to the *right* within a node node graph, *after* the socket definition.

A broadcast node is only ever associated with *one* [port node](./port.md).

## Requires

- [Node](../node.md)
- [Port](./port.md)

## Required By

*Nothing*
