# Set Node ([Frontend](../../../frontend.md))

A compiler node responsible for setting a value of a [*compiler state*](../../state/state.md).

___

1. Named Input - "Channel"

2. Repetitive Text Input - "Name"<br>
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_ 0123456789"
    - Default: "New State"

3. Named Input - "Value"

4. Named Output - "Continue"

___

Internally, the sequence of values from socket 2 are concatenated to form the unique *state id*. The delimiter is the `/` character e.g. `name_1/name_2/name_3`. This allows for namespacing. The "continue" output socket serves the single purpose of allowing traversel during node compilation - it does not carry any value and merely propagates activation.

## Requires

- [Group](../../groups/group.md)

## Required By

*Nothing*
