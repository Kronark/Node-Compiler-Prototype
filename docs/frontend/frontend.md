# Frontend

The (GUI) frontend of the Kronark Node Compiler is written in ***C#***. It's standalone purpose is the visualisation and modification of a project's node graph. It can *not* compile a given node graph with performed modifications. It can *only* receive node graph data via its file browser or communication from the [**backend**](../backend/backend.md) and then display that node graph in an optimal fashion.

## Responsibilities

- Node graph visualisation
- Node graph modification
- Node file modification

## Overview

- [Communication](./communication/communication.md)
- [History](./history/history.md)
- Graphical User Interface
    - Graph View
        - [Camera](./camera/camera.md)
        - [Grid](./grid/grid.md)
        - [Nodes](./nodes/node.md)
        - [Overlays](./overlays/overlays.md)
    - [Menu](./menu/menu.md)
    - [Window](./window/window.md)
- Parsing
    - [Node Parsing](./node_file_format/parsing.md)
    - [Layout Parsing](./layout_file_format/parsing.md)
- Serialisation
    - [Node Serialisation](./node_file_format/serialisation.md)
    - [Layout Serialisation](./layout_file_format/serialisation.md)
- User Input
    - [Clipboard](./clipboard/clipboard.md)
    - [Drag](./drag/drag.md)
    - [Hotkeys](./hotkeys/hotkeys.md)
    - [Traversal](./traversal/traversal.md)

## Task Dependencies

TODO
