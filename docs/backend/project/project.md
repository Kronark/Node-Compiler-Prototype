# Project ([Backend](../backend.md))

An abstraction layer to keep track of the currently active project root space for the compilation procedure. Includes functionality for:

- opening
- closing (only implicitely called by a project switch or software close)
- switching projects
- creating
- renaming

## Requires

- [Communication](../communication/communication.md)

## Required By

- [Compiler](../compilation/node_compiler.md)
- [Dependencies](../dependencies/dependency_graph.md)
