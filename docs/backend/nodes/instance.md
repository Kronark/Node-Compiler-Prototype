# Instance ([Backend](../backend.md))

An internal data type representing a node instance. Parsed from any of the file formats specified in `formats/node_file_format/` or created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `type`: The definition this node is an instance of.
2. `name`: The name of the node instance.
3. `sockets`: An array of sockets within the node instance.

## Requires

- Nodes
    - Socket

## Required By

- [Node Parsing](../node_file_format/parsing.md)
- [Node](./node.md)
