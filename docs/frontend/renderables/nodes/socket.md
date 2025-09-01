# Socket ([Frontend](../../frontend.md))

An internal data type representing a node socket. Parsed from any of the file formats specified in `formats/node_file_format/` or indirectly created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `is_outgoing`: Whether or not the socket is incoming or outgoing.
2. `is_connectible`: Whether or not the socket can be connected to. Always true if *outgoing*. Always false if *selection*
3. `is_repetition`: Whether or not the socket is part of a repetitive sequence.
4. `slot`: The slot index within the hosting node instance for the socket.
5. `type`: The type of the socket. See `socket_type` for further information.
6. `name`: The label text associated with the socket, rendered beside its input.
7. `parameters`: The parameters of the socket. See `socket_parameters` for further information.
8. `permitted`: A set of type identifiers that are permitted to connect to the socket.
9. `value`: The value of the socket.
10. `connection`: The connection data of the socket.

## Requires

- [Socket Parameters](./socket_parameters.md)
- [Socket Type](./socket_type.md)
- [Type](./type.md)
- [Value](./value.md)
- [Connection](./connection.md)

## Required By

- [Instance](./instance.md)
- [Node Parsing](../../node_file_format/parsing.md)
- [Node Serialisation](../../node_file_format/serialisation.md)
- [Layout Parsing](../../layout_file_format/parsing.md)
- [Layout Serialisation](../../layout_file_format/serialisation.md)
