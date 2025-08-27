# Socket Type ([Backend](../backend.md))

An enumeration of possible types a socket can take. Needs to be printable. The possible socket types so far are as follows:

1. Named:
    - No *direct* user input possible
    - Only allows *connections*
2. Number:
    - Provides a *number range* input if not connected
    - Range's minimum, maximum and step size controlled via parameters
3. Select:
    - Provides a *drop down selection* input
    - Can not be connected
    - Possible selectable options controlled via ***Option*** node
4. Switch:
    - Provides a *toggle button* input if not connected
    - Allows connection holding *truth values*
    - Text displayed in active and inactive state controlled via parameters
5. Text:
    - Provides a *single line text* input if not connected
    - Permitted characters, minimum length and maximum length controlled via parameters
6. Colour:
    - Provides a *colour picker* input if not connected

## Requires

*Nothing*

## Required By

- [Node Parsing](../node_file_format/parsing.md)
- [Socket](./socket.md)
