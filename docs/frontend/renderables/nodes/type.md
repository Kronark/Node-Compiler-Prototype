# Type ([Frontend](../../frontend.md))

A representation of compiler data types. Needs to be compatible with a Set data structure for implementation of the `permitted` field in [sockets](./socket.md). Potential fields include the following:

1. `is_package`: Flag indicating whether or not the associated data is a package of discrete values or a discrete value itself.
2. `identifier`: Primary characteristic of any data type.

## Requires

- [Identifier](./identifier.md)

## Required By

- [Connection](./connection.md)
