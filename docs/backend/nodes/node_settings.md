# Node Settings ([Backend](../backend.md))

A structure of node specific settings relevant for node instances and node file storage. While node files contain colour information for possible instances, the backend only requires the node space name and module path data. This type should therefore contain the following:

1. `Node Identifier`: The module path and node name ([identifier](./identifier.md)) of the node space.

## Requires

- [Identifier](./identifier.md)

## Required By

- [Node](./node.md)
- [Node Parsing](../node_file_format/parsing.md)
