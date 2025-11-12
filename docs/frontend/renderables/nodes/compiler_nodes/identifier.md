# Identifier Node ([Frontend](../../../frontend.md))

A compiler node responsible for declaring [identifiers](../identifier.md) in a structured manner. An identifier node's sockets are defined as follows:

___

1. Repetitive Text Input - "Module"
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyz-"
    - Default: *random valid string*

2. Text Input - "Name"
    - Minimum: "1"
    - Maximum: ""
    - Valid: "abcdefghijklmnopqrstuvwxyz-"
    - Default: *random valid string*

3. Named Output - "Identifier"

___

The random valid string inserted as a default value in both *module* and *name* sockets should not collide with any existing *modules* or *names*. Clicking a *module* input should provide an list of possible auto-completes, based on the module names declared before that active input. These should include any module names currently loaded that are sub-modules of the module path defined so far.

## Requires

- [Group](../../groups/group.md)

## Required By

- [Output](./output.md)
