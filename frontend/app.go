package main

import (
	"gioui.org/app"
	"gioui.org/op"
	"gioui.org/widget/material"
)

func main() {

	go func() {
		window := new(app.Window)
		var operations op.Ops
		_ = material.NewTheme()

		for {
			switch event := window.Event().(type) {
			case app.DestroyEvent:
				return
			case app.FrameEvent:
				operations.Reset()

				graphics_context := app.NewContext(&operations, event)

				// layout.Flex{}.Layout(graphics_context,
				// 	layout.Rigid(
				// 		func(gtx layout.Context) layout.Dimensions {
				// 			defer clip.Rect{Min: image.Pt(0, 0), Max: image.Pt(100, 100)}.Push(gtx.Ops).Pop()
				// 			paint.Fill(gtx.Ops, color.NRGBA{255, 0, 0, 255})
				// 			return layout.Dimensions{Size: image.Pt(100, 100)}
				// 		},
				// 	),
				// 	layout.Rigid(title.Layout),
				// )

				event.Frame(graphics_context.Ops)
			}
		}
	}()

	app.Main()
}
