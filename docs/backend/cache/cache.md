# Cache ([Backend](../backend.md))

The most important component for achieving the node compilation approach's high performance. The purpose of the compilation cache is to save the latest state of every node instance in a project. This can then be used by the [node compiler](../compilation/node_compiler.md) to skip large portions of a node graph when applying a certain change made by the user. If a given change does not affect a node either directly or indirectly, its traversal can be skipped and the cached values and side effects can be utilised instead.

This module not only needs to maintain a cache for all node instances, but also be capable of updating cache files after a successful node graph compilation. The associated file format specification, as well as the relevant parsing and serialisation tasks still require definition.

## Requires

- [Node Compiler](../compilation/node_compiler.md)

## Required By

*Nothing*
