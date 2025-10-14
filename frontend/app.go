// Frontend is a GUI for editing Kronark Node Compiler nodespaces.
package main

import (
	"os"

	"gioui.org/app"
	"gioui.org/op"
	"gioui.org/unit"
)

func main() {

	// window has to be spawned on another goroutine
	// because some platforms require control of the main thread
	go func() {

		window := new(app.Window)
		window.Option(
			app.Title("Kronark Node Compiler"),
			app.Size(unit.Dp(1280), unit.Dp(720)),
		)
		var operations op.Ops

		for {
			switch event := window.Event().(type) {
				
			case app.DestroyEvent:
				os.Exit(0)
				
			case app.FrameEvent:
				operations.Reset()
				
				graphicsContext := app.NewContext(&operations, event)
				
				// TODO: Rendering

				event.Frame(graphicsContext.Ops)
			}
		}
	}()

	app.Main()
}
