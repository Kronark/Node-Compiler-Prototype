<!--
Notes:
- N denoted variable byte integers, where fully set bytes (i.e. 11111111) signal an additional byte to be read, byte ordering is little endian
 -->

# Header

- `kronode` magic number [7]
- version number [1]

# Roots

- output root connection count [N]
- output root connections:
    - connection node [N]
    - connection socket [N]

# Nodes

- table size [N]
- table items:
    - node name length [N]
    - node name [string]

# Types

- table size [N]
- table items:
    - type name length [N]
    - type name [string]

# Groups

- group count [N]
- groups:
    - group id [N]
    - group colour [3]
    - group name length [N]
    - group name [string]
    - group item count [N]
    - group items:
        - item id [N]

# Comments

- comment count [N]
- comments:
    - item id [N]
    - comment colour [3]
    - comment length [N]
    - comment [string]

# Instances

- instances:
    - item id [N]
    - instance type [N]
    - instance name length [1]
    - instance name [string]
    - instance socket count [N]
    - instance sockets:
        - socket flags [1]:
            - PADDING [2 bit]
            - type and direction [3 bits]:
                - 000 = outgoing named
                - 001 = incoming named
                - 010 = incoming number
                - 011 = incoming select
                - 100 = incoming switch
                - 101 = incoming text
            - repetitive [1 bit]
            - connected [1 bit]
            - switch value [1 bit]
        - socket type index [N]
        - socket port slot [N]
        - if incoming:
            - if connected:
                - connection node [1]
                - connection socket [1]
            - if not connected and not switch:
                - value length [N]
                - value [string]
