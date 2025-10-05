# Type Node ([Backend](../../backend.md))

A compiler node responsible for adding type information to a data stream. The name associated with a type is used to control which connections can be established. A type node's sockets are defined as follows:

___

1. Named Input - "Data"

2. Switch Input - Unnamed
    - Inactive: "Not Default" (Default)
    - Active: "Default"
    - *Notes*:
        - Exactly one of multiple type nodes connected to a specific [port node](./port.md) can be marked as *default type*. It needs to be ensured that multiple or no default types are either automatically fixed or appropriately highlighted by the [compiler backend](../../backend.md).

3. Switch Input - Unnamed
    - Inactive: "Not Packaged" (Default)
    - Active: "Packaged"

4. Text Input - "Name"
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/_"
    - Default: "new_type"
    - *Notes*:
        - Type name specification may be overhauled with a module path system akin to namespaces in traditional programming.

5. Named Output - "Data"

___

A type node is always connected to the *data* socket of a [port node](./port.md) or an *item* socket of a [package node](./package.md). This means in case of an *input port*, they are placed to the *right* within a node graph, *after* the socket definition. In case of an *output port* or a *package*, they are placed to the *left* within a node graph, *before* the socket or package definition.

While an *input port* can accept *multiple* type node connections, an output port can only ever by associated with *a single one*.

## Requires

- [Node](../node.md)
- [Port](./port.md)

## Required By

*Nothing*
