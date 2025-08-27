# Backend

The backend of the Kronark Node Compiler is written in ***rust***. It's standalone purpose is the compilation of a project's node graph. It can *not* save node graph state changes or modify node files in any other way. It can *only* recieve a node graph either via CLI or communication from the [**frontend**](../frontend/frontend.md) and then compile that node graph in an optimal fashion. Output of a node graph compilation is stored in output files or streams as specified within the node graph. Socket values may be queried via CLI after compilation has concluded.

## Responsibilities

- Node graph compilation

## Overview

- [Communication](./communication/communication.md)
- [Compilation](./compilation/compiler.md)
- Compiler Nodes:
    - TODO
- [Dependencies](./dependencies/dependency_graph.md)
- [Node Parsing](./node_file_format/parsing.md)
- [Node Data](./nodes/node.md)
- [Project](./project/project.md)


## Task Dependencies

TODO
