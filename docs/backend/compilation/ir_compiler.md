# IR Compiler ([Backend](../backend.md))

A separate JIT (just-in-time) compilation process utilised by the [meta nodes](../meta_nodes/meta_node.md) system. The [node compiler](./node_compiler.md) invokes the IR compiler when a *meta node* is encountered. The IR compiler then compiles and executes the associated code of the traversed meta node and retrives the results for further use in the [node compiler](./node_compiler.md).

## Requires

- [Intermediate Representation](./ir.md)

## Required By

- [Compiler](./node_compiler.md)
