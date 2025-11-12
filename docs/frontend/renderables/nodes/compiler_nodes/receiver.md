# Receiver Node ([Frontend](../../../frontend.md))

A compiler node responsible for receiving the [target](../../../../backend/target/target.md) set by a [transmitter](./transmitter.md) during back-traversal.

___

1. Named Input - "Channel"

2. Named Input - "Name"

3. Named Output - "Activation"

___

The "name" has to be a [target identifier](../identifier.md). If no target id matching the provided id can be found, the "activation" output emits `false`. If the target id matches the target set during back traversal, `true` is emitted.

## Requires

- [Group](../../groups/group.md)

## Required By

*Nothing*
