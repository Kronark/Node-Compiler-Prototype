# Port Node ([Backend](../../backend.md))

A compiler node responsible for defining a socket (in a parent node instance) and providing its data to or receiving its data from the node graph within a given node space. A port node's sockets are defined as follows:

___

1. Switch Input - Unnamed<br>
    - Inactive: "Output"
    - Active: "Input" (Default)

If *1* is *active*:<br>

2. Named Input - "Channel"

Else If *1* is *inactive*:<br>

2. Named Output - "Channel"

End If

3. Number Input - "Slot"<br>
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"
    - *Notes*:
        - Slot values put all possible sockets of a node definition into a specific order (regardless if a given socket is visible or not). The value therefore needs to be unique within a given node space. *However this approach to socket ordering is not compatible with port broadcasting and thus needs to be revisited*.

4. Text Input - "Name"<br>
    - Minimum: "0"
    - Maximum: ""
    - Valid: ""
    - Default: "Socket"

If *1* is *active*:<br>

5. Select Input - "Type"<br>
    - "Colour"
    - "Named"
    - "Number"
    - "Select"
    - "Switch"
    - "Text" (Default)

6. Switch Input - Unnamed<br>
    - Inactive: "No Repetition" (Default)
    - Active: "Repetition"

If *6* is *active*:<br>

7. Number Input - "Repetition Minimum"
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"

8. Number Input - "Repetition Maximum"
    - Minimum: "1"
    - Maximum: ""
    - Step: "1"
    - Default: "1"

End If

End If

9. *Variable amount of socket-type specific settings - see [socket parameters](../socket_parameters.md)*

If *1* is *active*:<br>

10. Named Output - *Dynamic Name*

Else If *1* is *inactive*:<br>

10. Named Input - *Dynamic Name*

End If

___

The final socket of a port node, the socket providing or receiving data to be retained from or provided to the higher level socket instantiation, has a *dynamic name*. This is merely a quality of life feature, more precisely describing what kind of data a given port node is expecting based on its socket type. The mapping of types to socket names is as follows:

- Colour: "Colour"
- Named: "Data"
- Number: "Number"
- Select: "Selection"
- Switch: "Truth"
- Text: "Text"

In the backend this task includes functionality for *expression evaluation*. The parameter values of *minima* and *maxima* for both **text** and **number** sockets, as well as the *number step* parameter value can be defined as an expression instead of a concrete value. This allows users to specify limitations mathematically, which is especially useful for the *number step* parameter e.g. for forcing an input to be *power of 2*. See [socket parameters](../socket_parameters.md) for more details. The textual syntax of various mathematical expressions needs to be re-evaluated on re-compilation of a port node. It should support most common ascii representations of mathematical expressions such as `x/y` for division or `x^y` for exponentiation.

## Requires

- [Input](./input.md)
- [Output](./output.md)

## Required By

- [Broadcast](./broadcast.md)
- [Option](./option.md)
- [Pair](./pair.md)
- [Type](./type.md)
