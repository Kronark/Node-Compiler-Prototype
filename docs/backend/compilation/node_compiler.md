# Node Compilation ([Backend](../backend.md))

The core compilation loop for evaluating a given [node graph](../nodes/node.md) and triggering [meta node](../nodes/meta_node.md) execution. The procedure of the node compilation loop can be outlined as follows:

___
1. Read output node instance of node space
2. Push next input socket of the current node instance onto the [stack](./stack.md)
3. Check if all input values of the current node instance have been obtained
    1. If no<br>
    --> Consume a stack entry<br>
        1. If the socket is not connected<br>
        --> go to 3
        2. If the socket is connected<br>
        --> traverse the connection to the origin node instance<br>
        ---> go to 2
    2. If yes<br>
    --> evaluate the node<br>
        1. If the node instance is a [*custom* node](../nodes/node.md)<br>
        --> traverse the internal node graph<br>
        --> go to 1
        2. If the node instance is a [*meta* node](../nodes/meta_node.md)<br>
        --> execute associated instruction code<br>
        --> go to 2
        3. If the node instance is a built-in *compiler* node<br>
        --> execute associated hardcoded function<br>
        --> apply compiler node side effects (e.g. construct socket defined by a port node)<br>
        --> go to 2
___

Note that a significant component of the node compilation approach is its efficient [*caching*](../cache/cache.md). The above compilation loop only scales adequately if the traversal of unaffected node sub-graphs is skipped for a given user input.

Further note that this compilation loop needs to keep track of node graph *activation* states, which is not outlined above. Essentially, if a node is connected to an [input node](../nodes/compiler_nodes/input.md), that node is activated. Other nodes connected to an activated node are also activated. Additionally, truth values can be utilised for activation - however, further design work is necessary, as shown below.

___
Input Node --> Input Port Node for *truth socket* --> Type Node --> ... --> Type Node --> Output Port Node --> Output Node

If the truth value is set to *false* in this scenario, the output socket would not be rendered. However, propagating activation to output ports is necessary to allow for different outputs for different scenarios. It needs to be ensured that sockets vanishing due to truth value change do *not* cause fatal compiler errors.
___

## Requires

- [Node](../nodes/node.md)
- [Meta Node](../nodes/meta_node.md)
- [Stack](./stack.md)
- [Project](../project/project.md)

## Required By

- [IR Compiler](./ir_compiler.md)
- [Cache](../cache/cache.md)
