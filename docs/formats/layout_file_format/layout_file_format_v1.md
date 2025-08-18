<!--
Notes:
- all positions are single precision floating point
- all widths and heights are single precision floating point
- N denoted variable byte integers, where fully set bytes (i.e. 11111111) signal an additional byte to be read, byte ordering is little endian
-->

# Header

- `kronlyt` magic number [7]
- version number [1]

# Roots

- root positions:
    - input root position x [4]
    - input root position y [4]
    - output root position x [4]
    - output root position y [4]

# Groups

- group count [N]
- groups:
    - group id [N]
    - group position:
        - group position x [4]
        - group position y [4]
    - group dimensions:
        - group width [4]
        - group height [4]

# Comments

- comment count [N]
- comments:
    - item id [N]
    - comment position:
        - comment position x [4]
        - comment position y [4]
    - comment dimensions:
        - comment width [4]
        - comment height [4]

# Instances

- instances:
    - item id [N]
    - instance position:
        - instance position x [4]
        - instance position y [4]
    - instance width [4]