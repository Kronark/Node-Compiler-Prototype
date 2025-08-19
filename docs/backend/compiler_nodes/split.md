# Split Node

A built-in node that splits provided input data according to a specified rule and outputs each component separatly or as a single collection stream. Splitting rules may include:

- Index<br>
    --> split along an index into the byte array representation of the input<br>
    --> index is zero-based and the data it points to is included in the right side of the split
- Pattern<br>
    --> pattern defined via simple match or regex sequence

## Requires

- Compilation
    - Compiler

## Required By

*Nothing*
