# Lexicon

## Node Compiler (also: "Compiler")

The tool developed in this repository. It is a *compiler* due to the fact that it gathers a high level abstraction in its entirety (the node graphs) and transforms it into a lower level abstraction (any target format of a byte sequence). It is ***not*** an *interpreter* because it does *not necessarily* process one node at a time. Further, it does *not* transform nodes into an intermediate representation from the perspective of the program. Furthermore, nodes do *not* output the result of their operation, they output instructions to achieve such a result. Node compiler projects are *not* runnable executables, they are a source format that requires transformation to become an instance of the desired target - transformation that is considered *compilation*.

## Node

A draggable collection of sockets which is expected to perform a specific operation on provided input values. Contains at least one nodule for connection establishment to other nodes. In context of the compiler, a node is an abstract concept living in memory only.

## Node Instance (also: "Node", "Instance")

An instance is an individual of a group, each member of which shares the same underlying definition or purpose. A *node* instance is an instance of a definition describing a node. In context of the compiler, a node instance is the actual visualisation of a node, visible to the user within a node space. Each node instance is associated with a (possibly, but not necessarily) unique title. Note that the appreviated referral to node instances as "node" should only happen in a context where it is clear that "node instances" are meant.

## Socket

An individual input of a node instance. Associated with a label and / or an input field of a specific type. Depending on this type, a nodule is associated with the given socket for connection establishment. Sockets are associated with a direction. In context of the compiler, a socket can only be associated with an input field if it is of incoming / input direction. If the socket is of outgoing / output direction, it can merely be labeled.

## Nodule

A connection point of a socket within a node instance. In context of the compiler, the creation of a new connection is initialised by clicking a given nodule. Further, the completion of a connection procedure is accomplished by clicking another nodule. Additionally note that a nodule of an output socket can initialise *many* connections and one of an input socket can only ever hold *one* connection.

## Connection

A line of communication between two node instances. Can only be created between an output socket of one node and an input socket of another node. Transfers data from the origin to the target.

## Node Appearance (also: "Appearance")

The visual manifestation of a node. It encompasses not only the array of sockets possibly contained within a given node's instances, but also its colour. Note that in context of the compiler, the title of a node *instance* is not considered part of the underlying node's appearance, as it is unique to each instantiation. Similarly, specific values of a node instance's sockets are also not considered part of a node's appearance - merely their (possible) existence and associated ordering.

## Built-in Node

A node that represents embedded functionality of the compiler. It is not defined via internal node network and thus can not be opened up. Their purpose is to give users node representations for compiler specific operations associated with the appearance of a node.

## Custom Node

A node that has been constructed by a user of the compiler, representing functionality that is not defined by the compiler's underlying programming, but by the node graph within it. It can therefore be opened up, read and modified at all times.

## Node Definition (also: "Definition)

A graph of nodes that defines the behaviour and appearance of a custom node. It lives within the node space associated with a specific custom node. Note that *not all* node graphs are node definitions, since a node graph in a projects *main space* does not define a custom node.

## Node Space (also: "Space")

A virtual area in which a node graph lives. Note that all node definitions live in a node space, but not all node spaces contain a node definition.

## Main Space (also. "Main")

A node space representing the *entry point* of a given project. It does not define a custom node and thus can not be instantiated. Formerly considered the "root space".
