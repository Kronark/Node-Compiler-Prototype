# Output Node ([Backend](../../backend.md))

A compiler node responsible for providing a node graph with an output stream, as well as a place to define a node's settings. An output node's sockets are defined as follows:

___

1. Named Input - "Identifier"<br>

2. Repetitive Named Input - Unnamed

___

An output node, much like its [input node](./input.md) counterpart, can not be instantiated by the user and can only exist *once*. Further, relevant associated configurations are stored in a separate location within a [node](../node.md) and not mixed in with all other instances of a node space. Output nodes are instantiated *once* on creation of a node space.

## Requires

- [Identifier](./identifier.md)

## Required By

- [Port](./port.md)
