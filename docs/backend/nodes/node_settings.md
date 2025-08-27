# Node Settings ([Backend](../backend.md))

A structure of node specific settings relevant for node instances and node file storage. While node files contain colour information for possible instances, the backend only requires the node space name and module path data. This type should therefore contain the following:

1. `Node Name`: The name of the node space.
2. `Module Path`: A relative file path within the project file hierarchy.

## Requires

*Nothing*

## Required By

- [Node](./node.md)
- [Node Parsing](../node_file_format/parsing.md)
