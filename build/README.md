## Where the magic happens

You may notice the the actual `gltrace` library is quite small, that is because most of the actual work happens here and in `glhooker`. `gltrace` just stiches the two together.

Originally my plan was for all OpenGL calls be hooked by one function, which would then gather all the information about the call and trace it. This turned out to be too difficult to figure out though, so instead a hook is automatically generated for each OpenGL function with the help of the `gl_generator` library.

Whats another few KBs of code? Rust binaries are big as is.