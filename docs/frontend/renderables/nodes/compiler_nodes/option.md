# Option Node ([Frontend](../../../frontend.md))

A compiler node responsible for adding options to a selection input. Connected options to a singular selection port are ordered alphanumerically. An option node's sockets are defined as follows:

___

1. Named Input - "When"

2. Text Input - "Name"
    - Minimum: "1"
    - Maximum: ""
    - Valid: ""
    - Default: "New Option"

3. Named Output - "Then"

___

An option node is always connected to an [input port node](./port.md) if it is of type "select".

## Requires

- [Node](../node.md)

## Required By

*Nothing*
