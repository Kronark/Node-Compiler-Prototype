# Dependency Graph

A data structure for efficiently querying all dependencies of a given node space. This is necessary for preventing node instance creations that would result in circular traversal during compilation. A node space can not contain itself at any level of nesting. Any node that contains a given node or other nodes that do so can therefore not be created in that given node space. This may be best solved by implementing a DAG which is created per project. Unique node IDs within a project can then be linked if one contains the other. The relevant dependency information for a given node space can then be queried by traversing along all outgoing connections in the project's DAG and accumulating a list of forbidden node IDs. The entry point for this traversal in the DAG may be found via hashmap or similar data structure for constant time lookup.

## Requires

- Project
    - Project

## Required By

*Nothing*
