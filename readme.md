# Double Slit Sim !

## ⚠️⚠️⚠️ WASM VERSION IS EXTREMELY BUGGY ⚠️⚠️⚠️

I assume due to a complicated pile of bevy-WASM-webgpu-wgpu interop, but I didn't bother debugging since all of the above are iterating pretty quickly atm. If you'd like the nice, non-crashy experience, the desktop app runs smoothly :)

## What

This is a sim of the [double slit experiment](https://en.wikipedia.org/wiki/Double-slit_experiment) for both particles and waves.

## Why (or why not)

I picked this project because I missed physics and wanted to pick up basic GLSL/WGSL shader skills, and knew the math for the double slit experiment was pretty straightforward. I also wanted to pick up Bevy, so I figured I'd kill two birds. That, dear reader, was foolish. Bevy was not particularly well-suited to this project in numerous ways. The sane thing to do would have been to write everything in native JS rather than reinventing ~~wheel~~ basic UI in bevy. But! I wrote it and I did learn what I set out to, and I'm set up now to do more well-suited stuff in Bevy.

## TODO

- make app accessible via keyboard controls
- fix bug in WASM version where switching light > particles > light whitescreens
- fix bug in WASM version where using multiple GLSL shaders simply Doesn't Work (???) for possibly obscure memory-management reasons
- spruce up the UI, lol
