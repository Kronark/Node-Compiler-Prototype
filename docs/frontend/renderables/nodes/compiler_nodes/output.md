# Output Node ([Frontend](../../frontend.md))

A compiler node responsible for providing a node graph with an output stream, as well as a place to define a node's settings. An output node's sockets are defined as follows:

___

1. Text Input - "Module"<br>
    - Minimum: "0"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ /_"
    - Default: ""
    - Permitted: *module paths*

2. Text Input - "Name"<br>
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_ 0123456789"
    - Default: "New Node"

3. Colour Input - "Colour"<br>
    - Default: "0, 0, 0"

4. Named Repetitive Input - Unnamed

___

An output node, much like its [input node](./input.md) counterpart, can not be instantiated by the user and can only exist *once*. Further, relevant associated configurations are stored in a separate location within a [node](../node.md) and not mixed in with all other instances of a node space. Output nodes are instantiated *once* on creation of a node space.

## Requires

- [Node](../node.md)

## Required By

*Nothing*
