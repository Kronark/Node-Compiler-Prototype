# Selectable ([Frontend](../../frontend.md))

A utility module providing selection functionality for renderables. Needs to maintain a set of selected objects for collective drag and drop or cut, copy and paste. Further needs to provide a function that returns the *primary* selected renderable, which the inspector tool can use to provide debug information. The definition of the *primary* selected renderable should be the *last clicked* renderable.

## Requires

- [Comments](../../renderables/comments/comment.md)
- [Groups](../../renderables/groups/group.md)
- [Nodes](../../renderables/nodes/node.md)

## Required By

- [Clipboard](../../user_input/clipboard/clipboard.md)
- [Dragable](../dragable/dragable.md)
- [Inspecter](../../user_interface/tools/inspector/inspector.md)
