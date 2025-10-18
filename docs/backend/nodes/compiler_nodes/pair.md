# Pair Node ([Backend](../../backend.md))

A compiler node allowing users to render two input sockets next to each other in the same slot. A pair node's sockets are defined as follows:

___

1. Named Input - "Channel"

2. Number Input - "Slot"<br>
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"
    - *Notes*:
        - Slot values put all possible sockets of a node definition into a specific order (regardless if a given socket is visible or not). The value therefore needs to be unique within a given node space. *However this approach to socket ordering is not compatible with port broadcasting and thus needs to be revisited*.

3. Switch Input - Unnamed<br>
    - Inactive: "No Repetition" (Default)
    - Active: "Repetition"

If *3* is *active*:<br>

4. Number Input - "Repetition Minimum"
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"

5. Number Input - "Repetition Maximum"
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"

End If

6. Named Output - "Left"

7. Named Output - "Right"

___

The motiviation behind this node is simplifying the common task of defining key-item pairs. Pair nodes are placed to the left of the [port nodes](./port.md) they are supposed to pair up. The *slot* value of the pair node overrides the slot values defined in the connect port nodes. The same is the case for the *repetition* settings. A pair node's *channel* acts as. A pair node plugged into another pair node is not possible due to spacing constraints. Assigning more than a single item to a key can be achieved by connecting external custom nodes representing an item datum.

## Requires

- [Port](./port.md)

## Required By

*Nothing*
