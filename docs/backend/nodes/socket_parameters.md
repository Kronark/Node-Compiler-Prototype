# Socket Parameters

An enumeration of possible parameters a socket of a certain type can take. Needs to be printable. All parameters are defined as heap allocated strings. The parameters of each socket type are defined as follows:

1. Named:
    - Default: The default value.
2. Number:
    - Minimum:
        - The minimum value of the number range.
        - Empty string signals open endedness.
    - Maximum:
        - The maximum value of the number range (inclusive).
        - Empty string signals open endedness.
    - Step:
        - The maximum step that can be taken within the number range.
        - Empty string signals no step size limitation.
        - Input rounded to closest multiple of step size.
    - Default:
        - The default value.
3. Select:
    - Default:
        - The default value.
4. Switch:
    - Active:
        - The label visible within the switch's button if it's state is *active*.
    - Inactive:
        - The label visible within the switch's button if it's state is *inactive*.
    - Default:
        - The default value.
5. Text:
    - Minimum:
        - The minimum length of the input text.
        - Empty string signals no lower bound.
    - Maximum:
        - The maximum length of the input text (inclusive).
        - Empty string signals no upper bound.
    - Valid:
        - A sequence of valid characters, that are allowed to be entered.
        - Empty string signals no character limitation.
    - Default:
        - The default value.

## Requires

*Nothing*

## Required By

- Node File Format
    - Parsing V1
- Nodes
    - Socket
