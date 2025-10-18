# Layout Parsing ([Frontend](../frontend.md))

Module responsible for parsing the different versions of the layout file format as described in `formats/layout_file_format`. Parsed data does not necessarily need to be written to the frontend implementation of the [node data type](../renderables/nodes/node.md) and can be written to peripheral data structures as seen fit. Needs to include *invalidation* checks to prevent if the associated node space is overwritten by an updated version.

## Requires

- [Node](../renderables/nodes/node.md)
- [Layout Serialisation](./serialisation.md)

## Required By

*Nothing*
