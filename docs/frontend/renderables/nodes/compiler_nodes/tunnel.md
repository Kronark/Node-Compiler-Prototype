# Tunnel Node ([Frontend](../../../frontend.md))

A compiler node allowing users to provide data at different places within a node graph without needing to draw connections. A tunnel node's sockets are defined as follows:

___

1. Switch Input - Unnamed<br>
    - Inactive: "Entrance" (Default)
    - Active: "Exit"

2. Text Input - "Identifier"
    - Minimum: "1"
    - Maximum: "64"
    - Valid: ""
    - Default: "New Tunnel"

If *1* is *inactive*:

3. Named Input - "Data"

Else If *1* is *active*:

3. Named Output - "Data"

End If

___

The motiviation behind this node is node graph clarity, as highly complex node graphs inevitably lead to overlapping connections and thus unreadable visuals. Tunnels act like connections under the hood by assocating a data stream with an identifier local to the opened node space. All *exit* tunnels with a specific identifier receive the data provided to the sole *entrance* tunnel with the same identifier. 

## Requires

- [Node](../node.md)

## Required By

*Nothing*
