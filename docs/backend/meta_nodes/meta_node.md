# Meta Node ([Backend](../backend.md))

A module responsible for representing *meta* nodes in memory. Meta nodes are nodes that are *executed* during the [compilation process](../compilation/node_compiler.md). They allow users to define novel functionality for the compiler.

Similarly to standard [node compilation](../compilation/node_compiler.md), the node graph of a meta node is gradually compiled in lockstep with implementation. The same caching behaviour of standard node compilation is applied to meta node graphs, drastically improving compilation speeds. Unlike standard node compilation, meta node graph output [IR](../compilation/ir.md) is further optimised ahead of usage (practically standard AOT compiler optimisations). Most notably, the node graph specific optimisation of state pointers is applied. During state pointer optimisation, possible states of a meta node are analysed and different configurations are made to point to different IR code blocks. This allows the avoidance of branches that are *known* to be *cold* ahead of time.

Whenever a meta node is encountered during standard node compilation, the [IR JIT compiler](../compilation/ir_compiler.md) is invoked to *execute* the meta node instead of traversing it's internals. The JIT compiler further optimises the produced IR code blocks using platform specific information only available locally at runtime.

The motivation behind *meta nodes* is to eliminate as many *black boxes* as possible, while also allowing for *use case specificity* without burdoning the compiler developers. In the original node compiler design, many built-in nodes were provided to accomplish different general purpose tasks such as *splitting* and *repeating* byte sequences. Problems arose with *necessary* functionality which also needed to be *extendible*, such as the *math* or *format* nodes. It was not possible to provide compile-time calculation and formatting capabilities for *every possible* mathematical operation or data format. Meta nodes *solve* this problem by moving the task of defining *data transformative* functionality into the user space. The also *eliminates* a significant chunk of remaining black boxes, reducing the areas inaccessible to users to the *bare minimum*.

## Requires

- [IR Compilation](../compilation/ir_compiler.md)

## Required By

*Nothing*
