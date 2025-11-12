# Targets ([Backend](../../backend.md))

TODO: state system used only for targets now using identifiers, remove any value usages etc

The state system of the node compiler. States provide users with access to some memory during the compilation process. Practically, states function as a node editor equivalent to a variable in textual programming. They store some form of data, which can be referenced by a unique string considered the *state id*.

The node compiler only provides built-in functionality for *setting* and *getting* state values. All additional functionality such as *appending*, *incrementing* or *boolean checking* are to be constructed via [meta nodes](../nodes/meta_node.md).

## Requires

*Nothing*

## Required By

- [Transmitter Node](../nodes/compiler_nodes/transmitter.md)
- [Receiver Node](../nodes/compiler_nodes/receiver.md)
