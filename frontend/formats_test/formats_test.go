// Package formats_test implements tests for package formats
package formats_test

import (
	"bytes"
	"math"
	"testing"

	"github.com/Kronark/Node-Compiler-Prototype/frontend/defs/v1"
	"github.com/Kronark/Node-Compiler-Prototype/frontend/formats/v1"
)

// TestReadWriteVbi tests that [defs.Vbi] s written to by [WriteVbi] are read the same by [ReadVbi].
// 
// It tests the range [0, 2^14].
func TestReadWriteVbi(t *testing.T) {
	for i := range int(math.Pow(2,14)) + 1 {
		vbi := defs.Vbi(i)
		var buffer bytes.Buffer
		err := formats.WriteVbi(&buffer, vbi)
		
		if err != nil {
			t.Errorf("failed to write vbi: %s", err.Error())
		}
		
		readVbi, err := formats.ReadVbi(&buffer)
		if err != nil {
			t.Errorf("failed to read vbi: %s", err.Error())
		}
		if readVbi != vbi {
			t.Errorf("written and read vbis did not match: written: %d, read: %d", vbi, readVbi)
		}
	}
}

// TestMinVbiSize tests that [WriteVbi] writes the minimum number of bytes required to express a vbi.
// 
// It tests the range [0, 2^14].
func TestMinVbiSize(t *testing.T) {
	for i := range int(math.Pow(2,14)) + 1 {
		vbi := defs.Vbi(i)
		var buffer bytes.Buffer
		err := formats.WriteVbi(&buffer, vbi)
		
		if err != nil {
			t.Errorf("failed to write vbi: %s", err.Error())
		}
		
		bytesRequired := int(math.Max(1, math.Ceil(math.Log2(float64(i+1)) / 7)))
		
		t.Logf("%#b", buffer.Bytes())
		
		if bytesRequired != buffer.Len() {
			t.Errorf("number %d requires %d bytes but %d bytes were written (%#b)", i, bytesRequired, buffer.Len(), buffer.Bytes())
		}
	}
}
