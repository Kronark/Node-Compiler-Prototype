# Settings

The settings file format for the node compiler *prototype* is the TOML file format. Available settings, changable via the user interface or in a text editor before opening, are listed below:

1. Project
    - Name: Text
    - Local Folder: Filepath
2. Icons
    - Input / Output Colour: Colour
    - Identifier Colour: Colour
    - Type / Option / Package / Item Colour: Colour
    - Port / Pair / Broadcast Colour: Colour
    - Transmitter / Receiver Colour: Colour
    - Tunnel Colour: Colour
    - TODO: add colour configurations for other icons
3. Sockets
    - Sorting: Selection (`Ascending Alphanumerical`, `Descending Alphanumerical`)
    - Split I/O: Checkbox
4. Pins
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
6. Comments
    - Default: Colour
    - Rules: Colour -> Number[]
7. Groups
    - Default: Colour
    - Rules: Colour -> Number[]
8. Nodes
    - Rules: Colour -> Identifier[]

Identifiers can be any subset of a type or node identifier, subsets targetting entire modules. Wildcard behaviour is achieved by solely stating the project identifier, applying a rule to the entire project. If a more specific rule is encountered, it overwrites the effect of the more general rule. Pin shape, pin colour and connection colour identifiers are expected to reference *data* types, comment colour and group colour identifiers are expected to reference *category* types and node colour identifiers are expected to reference *node* types.

On project creation, a new settings file is generated and the contents of the last opened project are copied to the new file. If the created project is the first node compiler project on the given machine, a default settings file is generated, with most customisations initialising as blank fields.

Example file (`<project_hash>` represents the random hash string assigned to a project):
```TOML
[project]
name = "My Project"
local_folder = "path/to/some/folder/my_project"

[icons]
input_output = "FF0000"
identifier = "00FF00"
type_option_package_item = "0000FF"
port_pair_broadcast = "FFFF00"
transmitter_receiver = "00FFFF"
tunnel = "FFFFFF"

[sockets]
sorting = 0
split = true

[pins]
circle = ["<project_hash>"]
square = ["<project_hash>:module-1:type-a", "<project_hash>:module-1:module-2:type-b"]
diamond = []
rectangle = ["<project_hash>:module-3"]
default = "FF26FF"
[pins.rules]
FF0000 = ["<project_hash>:module-1:type-a", "<project_hash>:module-2"]
12FFAA = ["<project_hash>"]

[connections]
shape = 1
subpins = 0
default = "FFFFFF"
[connections.rules]
00F0F0 = ["<project_hash>:module-1:type-a", "<project_hash>:module-2"]
121212 = ["<project_hash>"]

[comments]
default = "101010"
[comments.rules]
FFFF00 = [0, 1]
FF0000 = [2]

[groups]
default = "333333"
[groups.rules]
FFFF00 = [1]
FF0000 = [0, 2]

[nodes]
FA0000 = ["<project_hash>:module-1", "<project_hash>:module-2"]
009900 = ["<project_hash>:module-3"]
00FF00 = ["<project_hash>:module-3:node-a"]
```
