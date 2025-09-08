<!--
Notes:
- all positions are single precision floating point
- all widths and heights are single precision floating point
- N denotes variable byte integers, where the most significant bit flags an additional byte to be read, byte ordering is little endian
-->

# Markdown Representation
## Header

- `kronlyt\0` magic number [8]
- version number [1]

## Roots

- root positions:
    - input root position x [4]
    - input root position y [4]
    - output root position x [4]
    - output root position y [4]

## Groups

- group count [N]
- groups:
    - group id [N]
    - group position:
        - group position x [4]
        - group position y [4]
    - group dimensions:
        - group width [4]
        - group height [4]

## Comments

- comment count [N]
- comments:
    - item id [N]
    - comment position:
        - comment position x [4]
        - comment position y [4]
    - comment dimensions:
        - comment width [4]
        - comment height [4]

## Instances

- instances:
    - item id [N]
    - instance position:
        - instance position x [4]
        - instance position y [4]
    - instance width [4]

# [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) Representation

Assuming "1" / "0" represent on/off bits, the file format is described by the following EBNF grammar:

```ebnf
bit = "1" | "0" ;
byte = 8 * bit ;

one prefixed byte = "1", 7 * bit ;
zero prefixed byte = "0", 7 * bit ;

(* variable byte integer, little endian *)
vbi = {one prefixed byte}, zero prefixed byte ;

(* any count of items. assume any repetition
  prefixed by count is repeated that many times *)
count = vbi ;

float = 4 * byte ;

kronlyt = "01101011", "01110010", "01101111",
  "01101110", "01101100", "01111001",
  "01110100", "00000000" ;

version number = byte ;

input root pos x = float ;
input root pos y = float ;
output root pos x = float ;
output root pos y = float ;

root positions = input root pos x,
  input root pos y, output root pos x,
  output root pos y ;

item id = vbi ;
layout x = float ;
layout y = float ;
layout width = float ;
layout height = float ;

item layout = item id, layout x,
  layout y, layout width, layout height ;

group = item layout ;
groups = count, {group} ;

comment = item layout ;
comments = count, {comment} ;

instance = item layout ;
instances = {instance} ;

file = kronlyt, version number, root positions,
  groups, comments, instances ;
```
