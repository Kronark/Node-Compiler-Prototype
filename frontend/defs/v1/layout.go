package defs

import "image"

// Represents all user-specific layout information provided by a layout file.
type Layout struct {
	InputRootPosition image.Point
	OutputRootPosition image.Point
	Groups []GroupLayout
	Comments []CommentLayout
	Instances []InstanceLayout
}

type GroupLayout struct {
	Id Vbi
	Position image.Point
	Dimensions image.Point
}

type CommentLayout struct {
	Id Vbi
	Position image.Point
	Dimensions image.Point
}

type InstanceLayout struct {
	Id Vbi
	Position image.Point
	Width float32
	Sockets []SocketLayout
}

type SocketLayout struct {
	ConnectionPins []ConnectionPinLayout
}

type ConnectionPinLayout struct {
	Position image.Point
}
