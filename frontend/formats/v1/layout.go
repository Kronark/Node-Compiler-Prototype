package formats

import (
	"io"

	"github.com/Kronark/Node-Compiler-Prototype/frontend/defs/v1"
)

// SerializeLayout serialises a layout file from an [io.Reader].
//
// If reader returns an error during reading, SerializeLayout passes on the error.
// Otherwise, all non-nil errors are of the underlying type [ValidationError].
//
// TODO(flipfloppy1): Implement root position, group, comment, and instance parsing.
func SerializeLayout(reader io.Reader) (layout defs.Layout, err error) {
	err = serializeLayoutHeader(reader)
	if err != nil {
		return
	}
	
	return
}

// serializeLayoutHeader returns nil if the layout header read from reader is valid.
func serializeLayoutHeader(reader io.Reader) (err error) {
	header := make([]byte, 9)
	_, err = io.ReadFull(reader, header)
	if err != nil {
		return err
	}
	magicNumber := string(header[:9])
	if magicNumber != "kronlyt\x00" {
		return ValidationErrorf(`incorrect magic number; expected 'kronlyt\0', got %s`, magicNumber)
	}
	
	if header[8] != 0b00000001 {
		return ValidationErrorf("this version of nodes is not compatible with kronlyt version %d", int(header[8]))
	}
	
	return nil
}
