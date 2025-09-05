# IR Compiler ([Backend](../backend.md))

A separate compilation process utilised by the [meta nodes](../nodes/meta_node.md) system. The IR compiler is responsible for converting the *temporary* [IR](./ir.md) into machine code runnable on the local system.

Note that the IR and this separate compiler loop are *temporary*, as their purpose is to help us bootstrap our way into *self-hosting* the meta node system. The goal is to compile meta nodes using custom programming nodes that emit machine code for the current platform - the *exact* same used in non-meta node spaces.

## Requires

- [Intermediate Representation](./ir.md)
- [Meta Nodes](../nodes/meta_node.md)

## Required By

- [Compiler](./node_compiler.md)
