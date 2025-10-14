package defs

import "image"

type Nodespace struct {
	OutputRootConnections []Connection
	Nodes []Node
	Types []Type
	Instances []Instance
	Groups []Group
	Comments []Comment
}

type Connection struct {
	Node Vbi
	Socket Vbi
}

// A node is identified by its name within the nodespace.
type Node string

// A type is identified by its name within the nodespace.
type Type string

type Instance struct {
	Id Vbi
	Type Vbi
	Name string
	Sockets []Socket
}

type Socket struct {
	Flags SocketFlags
	TypeIdx Vbi // index of the type in [Nodespace.Types].
	PortSlot Vbi
	Connection *Connection // nil if not [Flags.Incoming] and [Flags.Connected].
	Value *string // nil if [Flags.Connected] or [Flags.Switch].
}

type SocketFlags byte

// True if the socket is outgoing, otherwise false.
func (this SocketFlags) Outgoing() bool {
	return this & 0b00111000 == 0
}

// True if the socket is named, otherwise false.
func (this SocketFlags) Named() bool {
	return this & 0b00111000 == 0b00001000 || this & 0b00111000 == 0
}

// True if the socket is a number, otherwise false.
func (this SocketFlags) Number() bool {
	return this & 0b00111000 == 0b00010000
}

// True if the socket is a select, otherwise false.
func (this SocketFlags) Select() bool {
	return this & 0b00111000 == 0b00011000
}

// True if the socket is a switch, otherwise false.
func (this SocketFlags) Switch() bool {
	return this & 0b00111000 == 0b00100000
}

// True if the socket is text, otherwise false.
func (this SocketFlags) Text() bool {
	return this & 0b00111000 == 0b00101000
}

// True if the socket is a colour, otherwise false.
func (this SocketFlags) Color() bool {
	return this & 0b00111000 == 0b00110000
}

// True if the socket is repetitive, otherwise false.
func (this SocketFlags) Repetitive() bool {
	return this & 0b00000100 != 0
}

// True if the socket is connected, otherwise false.
func (this SocketFlags) Connected() bool {
	return this & 0b00000010 != 0
}

// True if the socket is a switch value, otherwise false.
func (this SocketFlags) SwitchValue() bool {
	return this & 0b00000001 != 0
}

type Group struct {
	Id Vbi
	DefaultColor image.NRGBA
	Name string
	Items []Vbi // each element is an ID to a comment or instance
}

type Comment struct {
	Id Vbi
	DefaultColor image.NRGBA
	Comment string
}
