# Frontend

The (GUI) frontend of the Kronark Node Compiler is written in ***C#***. It's standalone purpose is the visualisation and modification of a project's node graph. It can *not* compile a given node graph with performed modifications. It can *only* receive node graph data via its file browser or communication from the [**backend**](../backend/backend.md) and then display that node graph in an optimal fashion.

## Responsibilities

- Node graph visualisation
- Node graph modification
- Node file modification

## Overview

- <a href="./grid/grid.md" style="background:crimson;padding:0 5px;color:white">Grid</a>
- <a href="./tools/tools.md" style="background:chartreuse;padding:0 5px;color:black">Tools</a>
- <a href="./menu/menu.md" style="background:darkmagenta;padding:0 5px;color:white">Menu</a>

<div id="grid" style="aspect-ratio:16/9;width:98%;background:crimson;padding:1%">
    <div id="tools" style="width:98%;height:6%;background:chartreuse;padding:1%">
        <div id="menu" style="width:10%;height:100%;background:darkmagenta"></div>
    </div>
</div>
