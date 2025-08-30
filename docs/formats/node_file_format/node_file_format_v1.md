<!--
Notes:
- N denotes variable byte integers, where the most significant bit flags an additional byte to be read, byte ordering is little endian
-->

# Markdown Representation
## Header

- `kronode\0` magic number [8]
- version number [1]

## Roots

- output root connection count [N]
- output root connections:
    - connection node [N]
    - connection socket [N]

## Nodes

- table size [N]
- table items:
    - node name length [N]
    - node name [string]

## Types

- table size [N]
- table items:
    - type name length [N]
    - type name [string]

## Instances

- instance count [N]
- instances:
    - item id [N]
    - instance type [N]
    - instance name length [N]
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
                - 110 = incoming colour
            - repetitive [1 bit]
            - connected [1 bit]
            - switch value [1 bit]
        - socket type index [N]
        - socket port slot [N]
        - if incoming:
            - if connected:
                - connection node [N]
                - connection socket [N]
            - if not connected and not switch:
                - value length [N]
                - value [string]

## Groups

- group count [N]
- groups:
    - group id [N]
    - group colour [3]
    - group name length [N]
    - group name [string]
    - group item count [N]
    - group items:
        - item id [N]

## Comments

// count isn't strictly necessary since this is the last
structure in the file, but it's included for forwards-compatibility
- comment count [N]
- comments:
    - item id [N]
    - comment colour [3]
    - comment length [N]
    - comment [string]

# [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) Representation
Assuming "1"/"0" represent on/off bits, the file format is described by the following EBNF grammar:
```ebnf
bit = "0" | "1" ;
byte = 8 * bit ;
zero prefixed byte = "0", 7 * bit ;
one prefixed byte = "1", 7 * bit ;

(* variable byte integer, little endian *)
vbi = {one prefixed byte}, zero prefixed byte ;

(* any count of items. assume any repetition
  prefixed by count is repeated that many times *)
count = vbi ;

string = count, {? valid UTF-8 character ?} ;

kronode = "01101011", "01110010", "01101111",
  "01101110", "01101111", "01100100",
  "01100101", "00000000" ;

rgb = 3 * byte ;

version number = byte ;

output root connection node = vbi ;
output root connection socket = vbi ;
output root connection = output root connection node,
  output root connection socket ;

roots = count, {output root connection} ;

names = {string} ;
table = count, names ;

nodes = table ;
types = table ;

item id = vbi ;

instance type = vbi ;
instance name = string ;

socket flag = "00", 6 * bit ;

socket type index = vbi ;
socket port slot = vbi ;
socket value = string ;

connection node = vbi ;
connection socket = vbi ;
value = string ;

instance socket = socket flag, socket type index,
  socket port slot, [(connection node, connection socket) | value] ;
instance sockets = count, {instance socket} ;

instance = item id, instance type, instance name,
  instance sockets ;
instances = count, {instance} ;

group id = vbi ;
group colour = rgb ;
group name = string ;
group items = count, {item id} ;
group = group id, group colour, group name,
  group items ;

groups = count, {group} ;

comment colour = rgb ;
comment string = string ;
comment = item id, comment colour,
  comment string ;

comments = count, {comment} ;

file = kronode, version number, roots,
  nodes, types, instances, groups,
  comments ;
```
