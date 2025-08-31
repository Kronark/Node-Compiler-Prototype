# Instance ([Frontend](../frontend.md))

An internal data type representing a node instance. Parsed from any of the file formats specified in `formats/node_file_format/` or created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `type`: The definition this node is an instance of.
2. `name`: The name of the node instance.
3. `position`: The position of the node instance.
3. `sockets`: An array of sockets within the node instance.

The colour associated with the node space a node instance represents should fill the entire (rounded rectangular) shape in its background. This is opposed to the common approach of applying a node's colour only to the title area of an instance. The reasoning behind this design rule is, that full background colouring more clearly defines which elements on screen belong to a particular instance. Further, it rightfully highlights the fact that a node instance is draggable not only at its top area, but effectively anywhere within its shape which is not covered by socket inputs or labels.

## Requires

- [Socket](./socket.md)

## Required By

- [Node](./node.md)
- [Node Parsing](../node_file_format/parsing.md)
- [Node Serialisation](../node_file_format/serialisation.md)
- [Layout Parsing](../layout_file_format/parsing.md)
- [Layout Serialisation](../layout_file_format/serialisation.md)
