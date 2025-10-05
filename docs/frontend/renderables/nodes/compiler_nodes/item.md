# Item Node ([Frontend](../../../frontend.md))

A compiler node allowing users to retrieve singular labeled data entries from a *package*. An item node's sockets are defined as follows:

___

1. Named Input - "Package"

2. Text Input - "Label"
    - Minimum: "1"
    - Maximum: ""
    - Valid: ""
    - Default: "item"

3. Named Output - "Item"

___

This node is designed to work in conjunction with the [package node](./package.md). The motivation behind this pair of built-in nodes is to allow the moving of multiple datums via a single connection across node spaces at *virtually zero* performance cost. Item nodes are placed *after* [type nodes](./type.md) on the side of input declarations within a node space. They can only connect to *input* type nodes that are associated with a *packaged* type. The input type node's "packaged" flag needs to be raised.

## Requires

- [Type](./type.md)

## Required By

*Nothing*
