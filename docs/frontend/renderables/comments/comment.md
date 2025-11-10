# Comment ([Frontend](../../frontend.md))

A draggable box containing text. Needs to be associated with a specific node instance or group. Serves the purpose of allowing developers of a node space to communicate functionality inside of the node graph they are building. Associated with a category index for colouring rules, where `0` indicates the default.

The content of comments is considered *extended Markdown syntax* and is rendered appropriately when not being actively edited. When a comment is clicked for editing, the content is converted to plain text.

## Requires

- [Node](../nodes/node.md)

## Required By

- [Group](../groups/group.md)
