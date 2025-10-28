# Settings

The settings file format for the node compiler *prototype* is the TOML file format. Available settings, changable via the user interface or in a text editor before opening, are listed below:

1. Project
    - Name: Text
    - Local Folder: Filepath
2. Socket Order
    - Sorting: Selection (`Ascending Alphanumerical`, `Descending Alphanumerical`)
    - Split I/O: Checkbox
3. Pins
    - Circle: Identifier[]
    - Square: Identifier[]
    - Diamond: Identifier[]
    - Rectangle: Identifier[]
    - Default: Colour
    - Rules: Colour -> Identifier[]
5. Connections
    - Shape: Selection (`Linear`, `Smooth`, `Square`)
    - Subpins: Selection (`Circle`, `Square`, `Diamond`)
    - Default: Colour
    - Rules: Colour -> Identifier[]
7. Comments
    - Default: Colour
    - Rules: Colour -> Number[]
8. Groups
    - Default: Colour
    - Rules: Colour -> Number[]
9. Nodes
    - Rules: Colour -> Identifier[]

Identifiers can be any subset of a type or node identifier, subsets targetting entire modules. Wildcard behaviour is achieved by solely stating the project identifier, applying a rule to the entire project. Pin shape, pin colour and connection colour identifiers are expected to reference *data* types, comment colour and group colour identifiers are expected to reference *category* types and node colour identifiers are expected to reference *node* types.

On project creation, a new settings file is generated and the contents of the last opened project are copied to the new file. If the created project is the first node compiler project on the given machine, a default settings file is generated, with most customisations initialising as blank fields.

Example file:
```TOML
[]
```
