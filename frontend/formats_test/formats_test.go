// Package formats_test implements tests for package formats
package formats_test

import (
	"bytes"
	"testing"

	"github.com/Kronark/Node-Compiler-Prototype/frontend/defs/v1"
	"github.com/Kronark/Node-Compiler-Prototype/frontend/formats/v1"
)

// TestReadWriteVbi tests that [defs.Vbi] s written to by [WriteVbi] are read the same by [ReadVbi].
func TestReadWriteVbi(t *testing.T) {
	for i := range 1024 {
		vbi := defs.Vbi(i)
		var buffer bytes.Buffer
		err := formats.WriteVbi(&buffer, vbi)
		
		if err != nil {
			t.Errorf("failed to write vbi: %s", err.Error())
		}
		
		// t.Logf("buffer after writing: %v", buffer.Bytes())
		
		readVbi, err := formats.ReadVbi(&buffer)
		if err != nil {
			t.Errorf("failed to read vbi: %s", err.Error())
		}
		if readVbi != vbi {
			t.Errorf("written and read vbis did not match: written: %d, read: %d", vbi, readVbi)
		}
	}
}
