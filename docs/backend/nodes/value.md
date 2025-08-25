# Value

A wrapper type representing the value of a socket. Needs to be able to carry arbitrary forms of data. Two possible implementation paths could be considered:

1. An arbitrary length array of bytes, with functionality to interpret these bytes in different ways
2. An enumeration of all potential data types, with functionality to convert between them

## Requires

*Nothing*

## Required By

- Node File Format
    - Parsing V1
- Nodes
    - Socket
