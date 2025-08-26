# Frontend

The (GUI) frontend of the Kronark Node Compiler is written in ***C#***. It's standalone purpose is the visualisation and modification of a project's node graph. It can *not* compile a given node graph with performed modifications. It can *only* receive node graph data via its file browser or communication from the [**backend**](../backend/backend.md) and then display that node graph in an optimal fashion.

## Responsibilities

- Node graph visualisation
- Node graph modification
- Node file modification

## Overview

- [Grid](./grid/grid.md)
- [Camera](./camera/camera.md)
- [Nodes](./nodes/node.md)
