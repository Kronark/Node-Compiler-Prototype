# Receiver Node ([Backend](../../backend.md))

A compiler node responsible for getting the value of a [*compiler state*](../../state/state.md).

TODO: utilise identifier for state - since only used for compilation targets --> output activation instead of value

___

1. Named Input - "Channel"

2. Named Input - "Name"

3. Named Output - "Value"

___

The "name" has to be an [identifier](../identifier.md). If no state id matching the provided id can be found, the output socket returns an empty value.

## Requires

- [Node](../node.md)
- [Target](../../target/target.md)

## Required By

*Nothing*
