# Get Node ([Frontend](../../../frontend.md))

A compiler node responsible for getting the value of a [*compiler state*](../../state/state.md).

___

1. Named Input - "Channel"

2. Repetitive Text Input - "Name"<br>
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_ 0123456789"
    - Default: "New State"

3. Named Output - "Value"

___

Internally, the sequence of values from socket 2 are concatenated to form the unique *state id*. The delimiter is the `/` character e.g. `name_1/name_2/name_3`. This allows for namespacing. If no state id matching the provided id can be found, the output socket returns an empty value.

## Requires

- [Node](../node.md)

## Required By

*Nothing*
