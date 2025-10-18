# Package Node ([Frontend](../../../frontend.md))

A compiler node allowing users to package multiple data entries into a single connection stream, accessible via labels. A package node's sockets are defined as follows:

___

1. Repetitive Dual (Text & Named) Input<br>
    - Text Socket:
        - Minimum: "1"
        - Maximum: ""
        - Valid: ""
        - Default: "item"

2. Named Output - "Package"

___

This node is designed to work in conjunction with the [item node](./item.md). The motivation behind this pair of built-in nodes is to allow the moving of multiple datums via a single connection across node spaces at *virtually zero* performance cost. Package nodes are placed *between* [type nodes](./type.md) on the side of output declarations within a node space. Type nodes placed *before* the package node are used to declare the types of different items within a package. Type nodes placed *after* the package node are used to declare the type associated with the package as a whole. Type nodes declaring the type of packages have their "packaged" flag automatically raised on connection. Packages are essentially *equivalent* to structured data types of traditional programming languages.

## Requires

- [Type](./type.md)

## Required By

*Nothing*
