# Input Node ([Frontend](../../frontend.md))

A compiler node responsible for providing a node graph with an input stream of outside data. An input node's sockets are defined as follows:

___

1. Named Output - Unnamed<br>

___

An input node, much like its [output node](./output.md) counterpart, can not be instantiated by the user and can only exist *once*. Further, relevant associated configurations are stored in a separate location within a [node](../node.md) and not mixed in with all other instances of a node space. Input nodes are instantiated *once* on creation of a node space.

## Requires

- [Node](../node.md)

## Required By

*Nothing*
