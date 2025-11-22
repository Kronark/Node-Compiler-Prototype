# Lexicon

## Node Compiler (also: "compiler")

The Node Compiler is developed here, in this repository. It is called a *compiler* because it takes the **node graphs**, a high level abstraction, and transforms it into a target format of a byte sequence, a lower level abstraction. It is not considered an *interpreter* because it does not restricted to processing one **node** at a time, nor does it need to transform nodes into an *intermediary state* from the perspective of the program. Furthermore, **nodes** output instructions for a program, rather than processing that program themselves.

Note that Node Compiler projects are not runnable executables, they are a source format that requires transformation, *compilation*, to become an instance of the desired target.

## Node

A node is draggable collection of **sockets** which is expected to perform a specific operation on provided input values. A node contains at least one **pin** in order to connect to other nodes. In the context of **the compiler**, a node is an abstract concept living in memory only.

## Node Instance (also: "node", "instance")

A node instance is a single realisation of a node. In the context of the compiler, a node instance is the visual representation of a **node**, which allows it to be used within a **node space**. Each node instance can be associated with a unique title. Note that the abbreviated reference to node instances as "node" should only happen in a context where it is clear that "node instances" are meant.

## Node Graph

A node graph is a chain of connected **node instances**. In the context of **the compiler**, this is what will generate the result of a program when run.

## Socket

A socket is an individual input of a **node instance**. Associated with a label and/or an **input field** of a specific type. Depending on this type, a **pin** is created for the given socket to allow for other sockets of the same type to connect. In the context of the compiler, sockets are associated with a direction. A socket can only be associated with an **input field** if it is in the incoming / input direction. Otherwise, the socket is in the outgoing / output direction, it can only be labeled.

## Pin

A pin is the connection point of a **socket** within a **node instance**. In the context of the compiler, a new **connection** is created by selecting a pin on one **node**, then selecting a different pin on another. Additionally, note that while a pin of an output socket can initialise *multiple* connections, a pin of an input socket can only ever hold *one* connection.

## Connection

A line of communication between two **node instances**. Can only be created between an output socket of one node and an input socket of another node. Transfers data from the origin / output to the target / input.

## Input Field

In the context of the compiler, an input field is where the user can input information into a **node**. An input field is ignored if a **connection** is already established with the associated **pin**. The type of data that can be put into an input field are determined by the associated **socket's** **type**.

## Node Appearance (also: "appearance")

A node's appearance is how a **node** looks. It encompasses not only the list of **sockets** possibly contained within a given node's **instances**, but also its colour. In the context of the compiler, the title of a node *instance* is not considered part of the underlying node's appearance, as it is unique to each instantiation. Similarly, specific values of a node instance's sockets are also not considered part of a node's appearance - merely their (possible) existence and associated ordering.

## Built-in Node (also: "built-in")

A built-in node is a **node** that represents embedded functionality of the compiler. It is not defined via a **node graph** and thus it is not possible to view its components. Their purpose is to give users nodes that depict compiler specific operations associated with the appearance of a node.

## Custom Node

A custom node is a **node** that has been constructed by a user of the compiler. Its purpose is not defined by the compiler's underlying programming, but by the **node graph** within it. Because of this, it can be opened up, read and modified at all times.

## Node Definition (also: "definition")

A node definition is a graph of nodes that defines the behaviour and **appearance** of a custom node. It lives within the node space associated with a specific custom node. Note that not *all* node graphs are node definitions, since a node graph in a projects *main space* does not define a custom node.

## Node Space (also: "space")

A node space is a virtual area in which a **node graph** lives. Note that all **node definitions** live in a node space, but not all node spaces contain a node definition.

## Main Space (also: "main")

A main space is the **node space** denoting the *entry point* of a given project. It does not define a custom node and thus can not be instantiated. Formerly considered the "root space".
