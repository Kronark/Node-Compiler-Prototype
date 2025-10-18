# Intermediate Representation ([Backend](../backend.md))

A module defining the intermediate representation (IR) utilised by the [meta nodes](../nodes/meta_node.md) system of the node compiler. The IR is specified as follows:

___
- **Control Flow**:
    - ***If Else***
    - ***Case Switch***
    - ***For Loop***
    - ***While Loop***
- **Mathematics**:
    - ***Add***
    - ***Subtract***
    - ***Multiply***
    - ***Divide***
    - ***Modulo***
    - ***Exponentiate***
    - ***Logarithm***
    - ***Sine***
    - ***Cosine***
    - ***Tangent***
    - ***Arcsine***
    - ***Arccosine***
    - ***Arctangent***
___

Note that this IR and its [compiler](./ir_compiler.md) are *temporary* features. Their intended purpose is to help bootstrap the meta nodes towards being *self-hosted*. The meta nodes will ultimately be compiled using the [node compiler](node_compiler.md) directly, utilising implemented machine code targets, instead of interfacing with a traditional AOT compiler running in the backend. This is to gain the benefit of full *self improvement cycles*, as meta nodes will benefit from all optimisations done in user space, in turn making user space more beneficial, eventually leading to further optimisations.

## Requires

*Nothing*

## Required By

- [IR Compiler](./ir_compiler.md)
