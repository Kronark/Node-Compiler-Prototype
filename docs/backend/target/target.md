# Target ([Backend](../../backend.md))

The target state system of the node compiler. Targets allow down-stream nodes to specify a state, which will be passed along during back-traversal to trigger the activation of specific subgraphs. This provides the functionality of targetting multiple different outputs using a singular node definition.

Targets are identified via unique [identifiers](../nodes/identifier.md). This module is responsible for storing these identifiers and providing functionality to keep track of a target during connection traversal of the [compiler](../compilation/node_compiler.md).

## Requires

*Nothing*

## Required By

- [Transmitter Node](../nodes/compiler_nodes/transmitter.md)
- [Receiver Node](../nodes/compiler_nodes/receiver.md)
