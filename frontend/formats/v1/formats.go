// Package formats implements serialisation/deserialisation for the nodespace and layout file formats.
package formats

import (
	"fmt"
	"io"

	"github.com/Kronark/Node-Compiler-Prototype/frontend/defs/v1"
)

// An error returned by [SerializeNodespace] or [SerializeLayout] when
// the provided file data does not constitute a valid nodespace or layout file.
type ValidationError string

func (this ValidationError) Error() string {
	return string(this) 
}

// ValidationErrorf is equivalent to [ValidationError]([fmt.Sprintf](format, args))
func ValidationErrorf(format string, args ...any) ValidationError {
	return ValidationError(fmt.Sprintf(format, args...))
}

// Reads in a variable byte integer as a [defs.Vbi].
// 
// As any byte can be a valid [defs.Vbi] part, ReadVbi cannot return a [ValidationError].
// It only errors when reader reports an unexpected EOF.
func ReadVbi(reader io.Reader) (defs.Vbi, error) {
	var vbi defs.Vbi
	continueReading := true
	for continueReading {
		bytes := make([]byte, 1)
		_, err := io.ReadFull(reader, bytes)
		vbiByte := bytes[0]
		if err != nil {
			return 0, fmt.Errorf("error parsing vbi: %s", err.Error())
		}
		part := vbiByte & 0b01111111
		continueReading = vbiByte & 0b10000000 != 0
		vbi <<= 7
		vbi += defs.Vbi(part)
	}
	
	return vbi, nil
}

// WriteVbi deserialises vbi into writer in a form compatible with [ReadVbi].
// 
// WriteVbi only errors when [io.Writer.Write] returns an error.
func WriteVbi(writer io.Writer, vbi defs.Vbi) error {
	firstInfo := false
	for i := range 9 {
		part := byte((vbi >> ((8 - i) * 7)) & 0b01111111)
		if part == 0 && i < 8 && !firstInfo {
			continue
		}
		firstInfo = true
		if i < 8 {
			part += 0b10000000
		}
		_, err := writer.Write([]byte{part})
		if err != nil {
			return fmt.Errorf("error writing vbi: %s", err.Error())
		}
	}
	
	
	return nil
}
