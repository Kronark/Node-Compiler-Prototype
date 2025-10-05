# Value ([Backend](../backend.md))

A wrapper type representing the value of a socket. Needs to be able to carry arbitrary forms of data. Two possible implementation paths could be considered:

1. An arbitrary length array of bytes, with functionality to interpret these bytes in different ways
2. An enumeration of all potential data types, with functionality to convert between them

Data packaging, i.e. the association of *sub-values* with *label strings*, needs to be supported.

## Requires

*Nothing*

## Required By

- [Node Parsing](../node_file_format/parsing.md)
- [Socket](./socket.md)
