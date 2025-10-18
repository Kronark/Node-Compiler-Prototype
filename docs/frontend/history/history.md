# History ([Frontend](../frontend.md))

A tree data structure holding a history of edits for a given project. Provides functionality for rolling back and moving forward along the edit history, as well as switching between branches. The history module should support the following features:

- Tracking of maximum usage, maximum usage being configurable in the compiler's settings
- Addition of edits to the history
- Pruning of edits based on a threshold related to the configured memory limit e.g. `maximum - 50 KB`
- Serialisation and parsing of the history per compiler project for local permanence

Pruning should occur ***before*** the configured memory limit is reached. The configured memory limit should *never* be exceeded. A threshold should be put in place to trigger pruning. Pruning should occur in order of oldest to newest actions in the history. If a branch point in the *undo tree* is pruned, then all *except* the **active** branch are pruned in their entirety - a branch being considered active, if a state within it is currently displayed. Pruning should continue until the pruning threshold is satisfied once more.

Edits with a memory footprint *larger* than a separate history-item-size threshold (not configurable directly, but a fraction of the configured maximum) should be split into smaller edits until all sub-parts are below defined item-size threshold. This ensures that the currently used state within the edit history is never pruned.

In the edge case of the *current state* being at a branching point within the undo-tree and the memory usage of the old branch exceeding the pruning threshold, pruning should revert to deleting *future* items beyond the current state. This needs to be done in this emergency case, as the current state can never be pruned from the history. Note that this should *only* be done if the current state is the *oldest* edit within the undo-tree. This case is unlikely to occur during normal usage - but needs to be handled to avoid desaster.

## Requires

- [Window](../user_interface/window/window.md)

## Required By

- [Traversal](../user_input/traversal/traversal.md)
