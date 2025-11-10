# Socket ([Frontend](../../frontend.md))

An internal data type representing a node socket. Parsed from any of the file formats specified in `formats/node_file_format/` or indirectly created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `is_outgoing`: Whether or not the socket is incoming or outgoing.
2. `is_repetition`: Whether or not the socket is part of a repetitive sequence.
3. `slot`: The unique socket slow of a node space.
4. `type`: The type of the socket. See `socket_type` for further information.
5. `name`: The label text associated with the socket, rendered beside its input.
5. `parameters`: The parameters of the socket. See `socket_parameters` for further information.
6. `permitted`: A set of type identifiers that are permitted to connect to the socket.
7. `value`: The value of the socket.
8. `connection`: The connection data of the socket.

The socket slot is an arbitrary 32 bit integer that is unique within its node space. Memory footprint is minimised by assigning socket ids via usage tracking - utilising the next lowest possible id on socket change or creation.

## Requires

- [Socket Parameters](./socket_parameters.md)
- [Socket Type](./socket_type.md)
- [Value](./value.md)
- [Connection](./connection.md)
- [Pin](./pin.md)

## Required By

- [Instance](./instance.md)
