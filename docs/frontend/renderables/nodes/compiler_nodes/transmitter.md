# Transmitter Node ([Frontend](../../../frontend.md))

A compiler node responsible for transmitting a [target](../../../../backend/target/target.md) along a connection chain during back-traversal.

___

1. Named Input - "Channel"

2. Named Input - "Name"

3. Named Output - "Continue"

___

The "name" has to be an [target identifier](../identifier.md). The "continue" output socket serves the single purpose of allowing traversel during node compilation - it does not carry any value and merely propagates activation.

## Requires

- [Group](../../groups/group.md)

## Required By

*Nothing*
