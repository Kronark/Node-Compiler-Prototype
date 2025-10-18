# Type ([Frontend](../../frontend.md))

A wrapper type representing compiler types. Due to their abundance and variable size, should be a heap allocated String internally. Needs to be compatible with a Set data structure for implementation of the `permitted` field in *sockets*. Can **not** be empty.

## Requires

*Nothing*

## Required By

- [Connection](./connection.md)
