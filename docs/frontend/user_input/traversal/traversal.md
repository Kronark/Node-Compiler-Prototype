# Traversal ([Frontend](../../frontend.md))

A module providing functionality for navigating the compiler's node graph view more efficiently. Features should include:

- `jump_to_instance`: mainly utilised by compiler errors, allows users to jump to a specific node instance
- `jump_to_space`: mainly utilised by compiler errors, allows users to jump to a specific node space

## Requires

- [Node](../../renderables/nodes/node.md)
- [History](../../history/history.md)

## Required By

- [Hotkeys](../hotkeys/hotkeys.md)