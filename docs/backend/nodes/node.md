# Node ([Backend](../backend.md))

An internal data type representing a custom node space. Parsed from any of the file formats specified in `formats/node_file_format/` or created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `is_compiler_node`: Whether or not this node definition is that of a built-in compiler node. Depending on implementation details, this flag may be irrelevant.
2. `settings`: Information about the node space defined by an internal `settings` node. See `node_settings` for more information.
3. `roots`: Connection data pertaining to the input and output roots of the node space. See `node_roots` for more information.
4. `instances`: An array of node instances. See `instance` for more information.

## Requires

- [Instance](./instance.md)
- [Node Settings](./node_settings.md)
- [Node Roots](./node_roots.md)

## Required By

- [Compiler](../compilation/node_compiler.md)
- [Node Parsing](../node_file_format/parsing.md)
