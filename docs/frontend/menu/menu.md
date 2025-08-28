# Menu ([Frontend](../frontend.md))

A GUI component and underlying data structure providing an organised view of available nodes to the user. Nodes are grouped by their corresponding modules. Available nodes include:

1. Built-in nodes (in a module "compiler")
2. Custom nodes stored inside the open project folder
3. Custom nodes of imported modules

The menu component needs to query the dependencies of a node space when it is opened by sending a corresponding request to the backend. It then needs to disable the creation of any node instances that would lead to circular dependencies.

## Requires

- [Window](../window/window.md)

## Required By

*Nothing*
