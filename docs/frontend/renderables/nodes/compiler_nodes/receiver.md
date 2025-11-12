# Receiver Node ([Frontend](../../../frontend.md))

A compiler node responsible for getting the value of a [*compiler state*](../../state/state.md).

___

1. Named Input - "Channel"

2. Named Input - "Name"

3. Named Output - "Value"

___

The "name" has to be an [identifier](../identifier.md). If no state id matching the provided id can be found, the output socket returns an empty value.

## Requires

- [Group](../../groups/group.md)

## Required By

*Nothing*
