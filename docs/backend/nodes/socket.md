# Socket ([Backend](../backend.md))

An internal data type representing a node socket. Parsed from any of the file formats specified in `formats/node_file_format/` or indirectly created during use of the software. May contain additional data structures not included in the original file format. Furthermore, components encoded in the file format may be moved to external data structures. Potential fields include the following:

1. `is_outgoing`: Whether or not the socket is incoming or outgoing.
2. `is_repetition`: Whether or not the socket is part of a repetitive sequence.
3. `slot`: The slot index within the hosting node instance for the socket.
4. `type`: The type of the socket. See `socket_type` for further information.
5. `parameters`: The parameters of the socket. See `socket_parameters` for further information.
6. `permitted`: A set of type identifiers that are permitted to connect to the socket.
7. `value`: The value of the socket.
8. `connection`: The connection data of the socket.

## Requires

- Nodes
    - Socket Parameters
    - Socket Type
    - Type
    - Value
    - Connection

## Required By

- Node File Format
    - Parsing V1
- Nodes
    - Instance
