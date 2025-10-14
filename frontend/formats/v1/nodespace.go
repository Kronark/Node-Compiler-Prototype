package formats

import (
	"io"

	"github.com/Kronark/Node-Compiler-Prototype/frontend/defs/v1"
)

// SerializeNodespace serialises a nodespace file from an [io.Reader].
//
// If reader returns an error during reading, SerializeNodespace passes on the error.
// Otherwise, all non-nil errors are of the underlying type [ValidationError].
//
// TODO(flipfloppy1): Implement root connection, node, type, instance, group, and comment parsing.
func SerializeNodespace(reader io.Reader) (layout defs.Nodespace, err error) {
	err = serializeNodespaceHeader(reader)
	if err != nil {
		return
	}
	
	return
}

// serializeNodespaceHeader returns nil if the nodespace header read from reader is valid.
func serializeNodespaceHeader(reader io.Reader) (err error) {
	header := make([]byte, 9)
	_, err = io.ReadFull(reader, header)
	if err != nil {
		return err
	}
	magicNumber := string(header[:9])
	if magicNumber != "kronode\x00" {
		return ValidationErrorf(`incorrect magic number; expected 'kronode\0', got %s`, magicNumber)
	}
	
	if header[8] != 0b00000001 {
		return ValidationErrorf("this version of nodes is not compatible with kronode version %d", int(header[8]))
	}
	
	return nil
}
