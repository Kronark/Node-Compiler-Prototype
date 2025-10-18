# Frontend

The (GUI) frontend of the Kronark Node Compiler is written in ***Go***. It's standalone purpose is the visualisation and modification of a project's node graph. It can *not* compile a given node graph with performed modifications. It can *only* receive node graph data via its file browser or communication from the [**backend**](../backend/backend.md) and then display that node graph in an optimal fashion.

## Responsibilities

- Node graph visualisation
- Node graph modification
- Node file modification

## Overview

- [Communication](./communication/communication.md)
- [History](./history/history.md)
- Parsing
    - [Node Parsing](./node_file_format/parsing.md)
    - [Layout Parsing](./layout_file_format/parsing.md)
- Properties
    - [Collapsible](./properties/collapsible/collapsible.md)
    - [Dragable](./properties/dragable/dragable.md)
    - [Selectable](./properties/selectable/selectable.md)
- Renderables
    - [Comments](./renderables/comments/comment.md)
    - [Group](./renderables/groups/group.md)
    - [Nodes](./renderables/nodes/node.md)
    - Compiler Nodes:
        - [Broadcast Node](./renderables/nodes/compiler_nodes/broadcast.md)
        - [Get Node](./renderables/nodes/compiler_nodes/get.md)
        - [Input Node](./renderables/nodes/compiler_nodes/input.md)
        - [Item Node](./renderables/nodes/compiler_nodes/item.md)
        - [Option Node](./renderables/nodes/compiler_nodes/option.md)
        - [Output Node](./renderables/nodes/compiler_nodes/output.md)
        - [Package Node](./renderables/nodes/compiler_nodes/package.md)
        - [Pair Node](./renderables/nodes/compiler_nodes/pair.md)
        - [Port Node](./renderables/nodes/compiler_nodes/port.md)
        - [Set Node](./renderables/nodes/compiler_nodes/set.md)
        - [Tunnel Node](./renderables/nodes/compiler_nodes/tunnel.md)
        - [Type Node](./renderables/nodes/compiler_nodes/type.md)
- Serialisation
    - [Node Serialisation](./node_file_format/serialisation.md)
    - [Layout Serialisation](./layout_file_format/serialisation.md)
- User Input
    - [Clipboard](./user_input/clipboard/clipboard.md)
    - [Hotkeys](./user_input/hotkeys/hotkeys.md)
    - [Traversal](./user_input/traversal/traversal.md)
- User Interface
    - Graph View
        - [Camera](./user_interface/graph_view/camera/camera.md)
        - [Grid](./user_interface/graph_view/grid/grid.md)
        - [Overlays](./user_interface/graph_view/overlays/overlays.md)
        - [Layout Generation](./layout_file_format/generation.md)
    - Tools
        - [Inspector](./user_interface/tools/inspector/inspector.md)
        - [Issues](./user_interface/tools/issues/issues.md)
        - [Menu](./user_interface/tools/menu/menu.md)
    - [Window](./user_interface/window/window.md)
