# GLTrace

## OpenGL Tracing Library

This tool allows you trace OpenGL calls, a la `RenderDoc`, the big difference here is that it is integrated directly into your program, instead of launching your program through something like `RenderDoc` or `Nvidia Nsight`. 

This is still a WIP but as it stands it is quite neat I think; and at the very least is a cool tech demo. And it also took me a while to do.

It works by hooking into OpenGL calls using my other library I created for this project: [gl_hooker](https://github.com/lappn99/gl_hooker). Since the hooks it installs perserve the hooked function, and the hooks themselves call the hooked function after its done: your program otherwise runs as normal while the tracing is going on, and incurs little overhead.

## Use

First the aforementioned [gl_hooker](https://github.com/lappn99/gl_hooker) is required on your system. See install instructions for that. 

Obviously easy integration into the project is the #1 priority for something like this. I believe the API as it stands now is *decent* regarding this but it will probably change.

It seems to work best when using a loading library like [gl_loader](https://github.com/maeln/gl_loader), as the name of each symbol is required. Using the example on that page, this is what initialzing the tracer and installing the hooks would look like:

```
extern crate gl_loader;
use gltrace_rs;

fn load_gl_symbol() -> Result<(), Box<dyn std::error::Error>> {
    //Needs to happen before you load the functions, obviously.
    let mut gltracer = GLTracer::new()?;
    gl_loader::init_gl();
    gl::load_with(|symbol| {
        if let Err(e) = gltracer.trace_func(symbol) {
            println!("{}",e);
        }
        gl_loader::get_proc_address(symbol) as *const _
    });
    Ok(())
    /*Application code*/

```
The library pretty much does the rest of the work for you regarding the actual tracing. The "trace" can be found in `gltracer.trace` which is  `Box<Vec<String>>` of all the calls made with their parameter values.

The trace can then be written to whatever implements the `fmt::Write` trait, which is input to the very simple `TraceOutputGenerator` trait which takes all the traces and just writes them into the writer. All in all, to print out the trace to a file it might look like this:

```
/*Application code*/
^^^^^^^^^^^^^^^^^^^^

let mut trace_file = fs::File::create(format!("trace.{}", std::process::id())).unwrap();
let _ = gltrace.trace.write_trace(gltrace_rs::TraceTextGenerator, &mut trace_file);

}
```

> **_NOTE:_**  It is not wise to capture the trace of a *whole* program that you expect to run for a while as memory use will wise very quickly. Instead only the trace of a *single* frame should be captured. As of right now this can be done by simply clearing the `gltracer.trace` Vector after (or before, it depends) your `glClear` call or when you swap buffers. In the future a better facility will be implemented to handle this better. 

## Future

1. Playback of trace, with the ability to inspect the calls more closely. 
2. Iron out trace output generation. 
    * It may seem like the design of the `TraceOutputGenerator` is over engineering for such a simple thing. My thought is that different implementations can format the trace output differently. Like one for outputting it in a binary format etc. Basically, to provide a facility to easily extend the library.
3. Capture data buffers/shaders/mappings/textures etc.
    * This is related to #1 and #2. Really just capture *all* the data that is flowing to/from the GPU and store it.

