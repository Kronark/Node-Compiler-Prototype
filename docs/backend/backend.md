# Backend

The backend of the Kronark Node Compiler is written in ***rust***. It's standalone purpose is the compilation of a project's node graph. It can *not* save node graph state changes or modify node files in any other way. It can *only* recieve a node graph either via CLI or communication from the [**frontend**](../frontend/frontend.md) and then compile that node graph in an optimal fashion. Output of a node graph compilation is stored in output files or streams as specified within the node graph. Socket values may be queried via CLI after compilation has concluded.

## Responsibilities

- Node graph compilation
- Cache file maintenance

## Overview

- [Cache](./cache/cache.md)
- [Communication](./communication/communication.md)
- Compilation:
    - [IR Compilation](./compilation/ir_compiler.md) (*temporary*)
    - [IR](./compilation/ir.md) (*temporary*)
    - [Node Compilation](./compilation/node_compiler.md)
    - [Stack](./compilation/stack.md)
- [Dependencies](./dependencies/dependency_graph.md)
- Nodes:
    - [Node](./nodes/node.md)
    - [Meta Node](./nodes/meta_node.md)
    - Compiler Nodes:
        - [Broadcast Node](./nodes/compiler_nodes/broadcast.md)
        - [Input Node](./nodes/compiler_nodes/input.md)
        - [Item Node](./nodes/compiler_nodes/item.md)
        - [Option Node](./nodes/compiler_nodes/option.md)
        - [Output Node](./nodes/compiler_nodes/output.md)
        - [Package Node](./nodes/compiler_nodes/package.md)
        - [Pair Node](./nodes/compiler_nodes/pair.md)
        - [Port Node](./nodes/compiler_nodes/port.md)
        - [Receiver Node](./nodes/compiler_nodes/receiver.md)
        - [Transmitter Node](./nodes/compiler_nodes/transmitter.md)
        - [Tunnel Node](./nodes/compiler_nodes/tunnel.md)
        - [Type Node](./nodes/compiler_nodes/type.md)
- [Dependencies](./dependencies/dependency_graph.md)
- [Node Parsing](./node_file_format/parsing.md)
- [Project](./project/project.md)
- [Target](./target/target.md)
