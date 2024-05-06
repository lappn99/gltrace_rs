use crate::types::types;

//include!(concat!(env!("OUT_DIR"), "/hooks.rs"));

pub mod __gl_imports {
    pub use std::os::raw;
}
        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glActiveShaderProgram(pipeline: types::GLuint, program: types::GLuint){
            println!("Call ActiveShaderProgram with pipeline: {:?}, program: {:?}", pipeline, program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glActiveTexture(texture: types::GLenum){
            println!("Call ActiveTexture with texture: {:?}", texture);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glAttachShader(program: types::GLuint, shader: types::GLuint){
            println!("Call AttachShader with program: {:?}, shader: {:?}", program, shader);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBeginConditionalRender(id: types::GLuint, mode: types::GLenum){
            println!("Call BeginConditionalRender with id: {:?}, mode: {:?}", id, mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBeginQuery(target: types::GLenum, id: types::GLuint){
            println!("Call BeginQuery with target: {:?}, id: {:?}", target, id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBeginQueryIndexed(target: types::GLenum, index: types::GLuint, id: types::GLuint){
            println!("Call BeginQueryIndexed with target: {:?}, index: {:?}, id: {:?}", target, index, id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBeginTransformFeedback(primitiveMode: types::GLenum){
            println!("Call BeginTransformFeedback with primitiveMode: {:?}", primitiveMode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindAttribLocation(program: types::GLuint, index: types::GLuint, name: *const types::GLchar){
            println!("Call BindAttribLocation with program: {:?}, index: {:?}, name: {:?}", program, index, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindBuffer(target: types::GLenum, buffer: types::GLuint){
            println!("Call BindBuffer with target: {:?}, buffer: {:?}", target, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindBufferBase(target: types::GLenum, index: types::GLuint, buffer: types::GLuint){
            println!("Call BindBufferBase with target: {:?}, index: {:?}, buffer: {:?}", target, index, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindBufferRange(target: types::GLenum, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call BindBufferRange with target: {:?}, index: {:?}, buffer: {:?}, offset: {:?}, size: {:?}", target, index, buffer, offset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindBuffersBase(target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint){
            println!("Call BindBuffersBase with target: {:?}, first: {:?}, count: {:?}, buffers: {:?}", target, first, count, buffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindBuffersRange(target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, sizes: *const types::GLsizeiptr){
            println!("Call BindBuffersRange with target: {:?}, first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, sizes: {:?}", target, first, count, buffers, offsets, sizes);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindFragDataLocation(program: types::GLuint, color: types::GLuint, name: *const types::GLchar){
            println!("Call BindFragDataLocation with program: {:?}, color: {:?}, name: {:?}", program, color, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindFragDataLocationIndexed(program: types::GLuint, colorNumber: types::GLuint, index: types::GLuint, name: *const types::GLchar){
            println!("Call BindFragDataLocationIndexed with program: {:?}, colorNumber: {:?}, index: {:?}, name: {:?}", program, colorNumber, index, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindFramebuffer(target: types::GLenum, framebuffer: types::GLuint){
            println!("Call BindFramebuffer with target: {:?}, framebuffer: {:?}", target, framebuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindImageTexture(unit: types::GLuint, texture: types::GLuint, level: types::GLint, layered: types::GLboolean, layer: types::GLint, access: types::GLenum, format: types::GLenum){
            println!("Call BindImageTexture with unit: {:?}, texture: {:?}, level: {:?}, layered: {:?}, layer: {:?}, access: {:?}, format: {:?}", unit, texture, level, layered, layer, access, format);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindImageTextures(first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint){
            println!("Call BindImageTextures with first: {:?}, count: {:?}, textures: {:?}", first, count, textures);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindProgramPipeline(pipeline: types::GLuint){
            println!("Call BindProgramPipeline with pipeline: {:?}", pipeline);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindRenderbuffer(target: types::GLenum, renderbuffer: types::GLuint){
            println!("Call BindRenderbuffer with target: {:?}, renderbuffer: {:?}", target, renderbuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindSampler(unit: types::GLuint, sampler: types::GLuint){
            println!("Call BindSampler with unit: {:?}, sampler: {:?}", unit, sampler);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindSamplers(first: types::GLuint, count: types::GLsizei, samplers: *const types::GLuint){
            println!("Call BindSamplers with first: {:?}, count: {:?}, samplers: {:?}", first, count, samplers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindTexture(target: types::GLenum, texture: types::GLuint){
            println!("Call BindTexture with target: {:?}, texture: {:?}", target, texture);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindTextureUnit(unit: types::GLuint, texture: types::GLuint){
            println!("Call BindTextureUnit with unit: {:?}, texture: {:?}", unit, texture);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindTextures(first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint){
            println!("Call BindTextures with first: {:?}, count: {:?}, textures: {:?}", first, count, textures);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindTransformFeedback(target: types::GLenum, id: types::GLuint){
            println!("Call BindTransformFeedback with target: {:?}, id: {:?}", target, id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindVertexArray(array: types::GLuint){
            println!("Call BindVertexArray with array: {:?}", array);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindVertexBuffer(bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei){
            println!("Call BindVertexBuffer with bindingindex: {:?}, buffer: {:?}, offset: {:?}, stride: {:?}", bindingindex, buffer, offset, stride);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBindVertexBuffers(first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei){
            println!("Call BindVertexBuffers with first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, strides: {:?}", first, count, buffers, offsets, strides);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat){
            println!("Call BlendColor with red: {:?}, green: {:?}, blue: {:?}, alpha: {:?}", red, green, blue, alpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendEquation(mode: types::GLenum){
            println!("Call BlendEquation with mode: {:?}", mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendEquationSeparate(modeRGB: types::GLenum, modeAlpha: types::GLenum){
            println!("Call BlendEquationSeparate with modeRGB: {:?}, modeAlpha: {:?}", modeRGB, modeAlpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendEquationSeparatei(buf: types::GLuint, modeRGB: types::GLenum, modeAlpha: types::GLenum){
            println!("Call BlendEquationSeparatei with buf: {:?}, modeRGB: {:?}, modeAlpha: {:?}", buf, modeRGB, modeAlpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendEquationi(buf: types::GLuint, mode: types::GLenum){
            println!("Call BlendEquationi with buf: {:?}, mode: {:?}", buf, mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendFunc(sfactor: types::GLenum, dfactor: types::GLenum){
            println!("Call BlendFunc with sfactor: {:?}, dfactor: {:?}", sfactor, dfactor);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendFuncSeparate(sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum){
            println!("Call BlendFuncSeparate with sfactorRGB: {:?}, dfactorRGB: {:?}, sfactorAlpha: {:?}, dfactorAlpha: {:?}", sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendFuncSeparatei(buf: types::GLuint, srcRGB: types::GLenum, dstRGB: types::GLenum, srcAlpha: types::GLenum, dstAlpha: types::GLenum){
            println!("Call BlendFuncSeparatei with buf: {:?}, srcRGB: {:?}, dstRGB: {:?}, srcAlpha: {:?}, dstAlpha: {:?}", buf, srcRGB, dstRGB, srcAlpha, dstAlpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlendFunci(buf: types::GLuint, src: types::GLenum, dst: types::GLenum){
            println!("Call BlendFunci with buf: {:?}, src: {:?}, dst: {:?}", buf, src, dst);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlitFramebuffer(srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum){
            println!("Call BlitFramebuffer with srcX0: {:?}, srcY0: {:?}, srcX1: {:?}, srcY1: {:?}, dstX0: {:?}, dstY0: {:?}, dstX1: {:?}, dstY1: {:?}, mask: {:?}, filter: {:?}", srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBlitNamedFramebuffer(readFramebuffer: types::GLuint, drawFramebuffer: types::GLuint, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum){
            println!("Call BlitNamedFramebuffer with readFramebuffer: {:?}, drawFramebuffer: {:?}, srcX0: {:?}, srcY0: {:?}, srcX1: {:?}, srcY1: {:?}, dstX0: {:?}, dstY0: {:?}, dstX1: {:?}, dstY1: {:?}, mask: {:?}, filter: {:?}", readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBufferData(target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum){
            println!("Call BufferData with target: {:?}, size: {:?}, data: {:?}, usage: {:?}", target, size, data, usage);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBufferStorage(target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: types::GLbitfield){
            println!("Call BufferStorage with target: {:?}, size: {:?}, data: {:?}, flags: {:?}", target, size, data, flags);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glBufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void){
            println!("Call BufferSubData with target: {:?}, offset: {:?}, size: {:?}, data: {:?}", target, offset, size, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCheckFramebufferStatus(target: types::GLenum){
            println!("Call CheckFramebufferStatus with target: {:?}", target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCheckNamedFramebufferStatus(framebuffer: types::GLuint, target: types::GLenum){
            println!("Call CheckNamedFramebufferStatus with framebuffer: {:?}, target: {:?}", framebuffer, target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClampColor(target: types::GLenum, clamp: types::GLenum){
            println!("Call ClampColor with target: {:?}, clamp: {:?}", target, clamp);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClear(mask: types::GLbitfield){
            println!("Call Clear with mask: {:?}", mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferData(target: types::GLenum, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearBufferData with target: {:?}, internalformat: {:?}, format: {:?}, type_: {:?}, data: {:?}", target, internalformat, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferSubData(target: types::GLenum, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearBufferSubData with target: {:?}, internalformat: {:?}, offset: {:?}, size: {:?}, format: {:?}, type_: {:?}, data: {:?}", target, internalformat, offset, size, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferfi(buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint){
            println!("Call ClearBufferfi with buffer: {:?}, drawbuffer: {:?}, depth: {:?}, stencil: {:?}", buffer, drawbuffer, depth, stencil);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferfv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat){
            println!("Call ClearBufferfv with buffer: {:?}, drawbuffer: {:?}, value: {:?}", buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint){
            println!("Call ClearBufferiv with buffer: {:?}, drawbuffer: {:?}, value: {:?}", buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearBufferuiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint){
            println!("Call ClearBufferuiv with buffer: {:?}, drawbuffer: {:?}, value: {:?}", buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat){
            println!("Call ClearColor with red: {}, green: {}, blue: {}, alpha: {}", red, green, blue, alpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearDepth(depth: types::GLdouble){
            println!("Call ClearDepth with depth: {:?}", depth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearDepthf(d: types::GLfloat){
            println!("Call ClearDepthf with d: {:?}", d);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedBufferData(buffer: types::GLuint, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearNamedBufferData with buffer: {:?}, internalformat: {:?}, format: {:?}, type_: {:?}, data: {:?}", buffer, internalformat, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedBufferSubData(buffer: types::GLuint, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearNamedBufferSubData with buffer: {:?}, internalformat: {:?}, offset: {:?}, size: {:?}, format: {:?}, type_: {:?}, data: {:?}", buffer, internalformat, offset, size, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedFramebufferfi(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint){
            println!("Call ClearNamedFramebufferfi with framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, depth: {:?}, stencil: {:?}", framebuffer, buffer, drawbuffer, depth, stencil);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedFramebufferfv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat){
            println!("Call ClearNamedFramebufferfv with framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?}", framebuffer, buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedFramebufferiv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint){
            println!("Call ClearNamedFramebufferiv with framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?}", framebuffer, buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearNamedFramebufferuiv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint){
            println!("Call ClearNamedFramebufferuiv with framebuffer: {:?}, buffer: {:?}, drawbuffer: {:?}, value: {:?}", framebuffer, buffer, drawbuffer, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearStencil(s: types::GLint){
            println!("Call ClearStencil with s: {:?}", s);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearTexImage(texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearTexImage with texture: {:?}, level: {:?}, format: {:?}, type_: {:?}, data: {:?}", texture, level, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClearTexSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, data: *const __gl_imports::raw::c_void){
            println!("Call ClearTexSubImage with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, type_: {:?}, data: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClientWaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64){
            println!("Call ClientWaitSync with sync: {:?}, flags: {:?}, timeout: {:?}", sync, flags, timeout);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glClipControl(origin: types::GLenum, depth: types::GLenum){
            println!("Call ClipControl with origin: {:?}, depth: {:?}", origin, depth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean){
            println!("Call ColorMask with red: {:?}, green: {:?}, blue: {:?}, alpha: {:?}", red, green, blue, alpha);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorMaski(index: types::GLuint, r: types::GLboolean, g: types::GLboolean, b: types::GLboolean, a: types::GLboolean){
            println!("Call ColorMaski with index: {:?}, r: {:?}, g: {:?}, b: {:?}, a: {:?}", index, r, g, b, a);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorP3ui(type_: types::GLenum, color: types::GLuint){
            println!("Call ColorP3ui with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorP3uiv(type_: types::GLenum, color: *const types::GLuint){
            println!("Call ColorP3uiv with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorP4ui(type_: types::GLenum, color: types::GLuint){
            println!("Call ColorP4ui with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glColorP4uiv(type_: types::GLenum, color: *const types::GLuint){
            println!("Call ColorP4uiv with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompileShader(shader: types::GLuint){
            println!("Call CompileShader with shader: {:?}", shader);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexImage1D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, border: {:?}, imageSize: {:?}, data: {:?}", target, level, internalformat, width, border, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexImage2D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, border: {:?}, imageSize: {:?}, data: {:?}", target, level, internalformat, width, height, border, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexImage3D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, border: {:?}, imageSize: {:?}, data: {:?}", target, level, internalformat, width, height, depth, border, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexSubImage1D with target: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", target, level, xoffset, width, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexSubImage2D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", target, level, xoffset, yoffset, width, height, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTexSubImage3D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTextureSubImage1D with texture: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", texture, level, xoffset, width, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTextureSubImage2D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", texture, level, xoffset, yoffset, width, height, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCompressedTextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void){
            println!("Call CompressedTextureSubImage3D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, imageSize: {:?}, data: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyBufferSubData(readTarget: types::GLenum, writeTarget: types::GLenum, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call CopyBufferSubData with readTarget: {:?}, writeTarget: {:?}, readOffset: {:?}, writeOffset: {:?}, size: {:?}", readTarget, writeTarget, readOffset, writeOffset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyImageSubData(srcName: types::GLuint, srcTarget: types::GLenum, srcLevel: types::GLint, srcX: types::GLint, srcY: types::GLint, srcZ: types::GLint, dstName: types::GLuint, dstTarget: types::GLenum, dstLevel: types::GLint, dstX: types::GLint, dstY: types::GLint, dstZ: types::GLint, srcWidth: types::GLsizei, srcHeight: types::GLsizei, srcDepth: types::GLsizei){
            println!("Call CopyImageSubData with srcName: {:?}, srcTarget: {:?}, srcLevel: {:?}, srcX: {:?}, srcY: {:?}, srcZ: {:?}, dstName: {:?}, dstTarget: {:?}, dstLevel: {:?}, dstX: {:?}, dstY: {:?}, dstZ: {:?}, srcWidth: {:?}, srcHeight: {:?}, srcDepth: {:?}", srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyNamedBufferSubData(readBuffer: types::GLuint, writeBuffer: types::GLuint, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call CopyNamedBufferSubData with readBuffer: {:?}, writeBuffer: {:?}, readOffset: {:?}, writeOffset: {:?}, size: {:?}", readBuffer, writeBuffer, readOffset, writeOffset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint){
            println!("Call CopyTexImage1D with target: {:?}, level: {:?}, internalformat: {:?}, x: {:?}, y: {:?}, width: {:?}, border: {:?}", target, level, internalformat, x, y, width, border);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint){
            println!("Call CopyTexImage2D with target: {:?}, level: {:?}, internalformat: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}, border: {:?}", target, level, internalformat, x, y, width, height, border);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei){
            println!("Call CopyTexSubImage1D with target: {:?}, level: {:?}, xoffset: {:?}, x: {:?}, y: {:?}, width: {:?}", target, level, xoffset, x, y, width);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call CopyTexSubImage2D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", target, level, xoffset, yoffset, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call CopyTexSubImage3D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", target, level, xoffset, yoffset, zoffset, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei){
            println!("Call CopyTextureSubImage1D with texture: {:?}, level: {:?}, xoffset: {:?}, x: {:?}, y: {:?}, width: {:?}", texture, level, xoffset, x, y, width);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call CopyTextureSubImage2D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", texture, level, xoffset, yoffset, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCopyTextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call CopyTextureSubImage3D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", texture, level, xoffset, yoffset, zoffset, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateBuffers(n: types::GLsizei, buffers: *mut types::GLuint){
            println!("Call CreateBuffers with n: {:?}, buffers: {:?}", n, buffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint){
            println!("Call CreateFramebuffers with n: {:?}, framebuffers: {:?}", n, framebuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateProgram(){
            println!("Call CreateProgram with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateProgramPipelines(n: types::GLsizei, pipelines: *mut types::GLuint){
            println!("Call CreateProgramPipelines with n: {:?}, pipelines: {:?}", n, pipelines);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateQueries(target: types::GLenum, n: types::GLsizei, ids: *mut types::GLuint){
            println!("Call CreateQueries with target: {:?}, n: {:?}, ids: {:?}", target, n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint){
            println!("Call CreateRenderbuffers with n: {:?}, renderbuffers: {:?}", n, renderbuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateSamplers(n: types::GLsizei, samplers: *mut types::GLuint){
            println!("Call CreateSamplers with n: {:?}, samplers: {:?}", n, samplers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateShader(type_: types::GLenum){
            println!("Call CreateShader with type_: {:?}", type_);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateShaderProgramv(type_: types::GLenum, count: types::GLsizei, strings: *const *const types::GLchar){
            println!("Call CreateShaderProgramv with type_: {:?}, count: {:?}, strings: {:?}", type_, count, strings);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateTextures(target: types::GLenum, n: types::GLsizei, textures: *mut types::GLuint){
            println!("Call CreateTextures with target: {:?}, n: {:?}, textures: {:?}", target, n, textures);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateTransformFeedbacks(n: types::GLsizei, ids: *mut types::GLuint){
            println!("Call CreateTransformFeedbacks with n: {:?}, ids: {:?}", n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCreateVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint){
            println!("Call CreateVertexArrays with n: {:?}, arrays: {:?}", n, arrays);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glCullFace(mode: types::GLenum){
            println!("Call CullFace with mode: {:?}", mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDebugMessageCallback(callback: types::GLDEBUGPROC, userParam: *const __gl_imports::raw::c_void){
            println!("Call DebugMessageCallback with callback: {:?}, userParam: {:?}", callback, userParam);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDebugMessageControl(source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean){
            println!("Call DebugMessageControl with source: {:?}, type_: {:?}, severity: {:?}, count: {:?}, ids: {:?}, enabled: {:?}", source, type_, severity, count, ids, enabled);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDebugMessageInsert(source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar){
            println!("Call DebugMessageInsert with source: {:?}, type_: {:?}, id: {:?}, severity: {:?}, length: {:?}, buf: {:?}", source, type_, id, severity, length, buf);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteBuffers(n: types::GLsizei, buffers: *const types::GLuint){
            println!("Call DeleteBuffers with n: {:?}, buffers: {:?}", n, buffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteFramebuffers(n: types::GLsizei, framebuffers: *const types::GLuint){
            println!("Call DeleteFramebuffers with n: {:?}, framebuffers: {:?}", n, framebuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteProgram(program: types::GLuint){
            println!("Call DeleteProgram with program: {:?}", program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteProgramPipelines(n: types::GLsizei, pipelines: *const types::GLuint){
            println!("Call DeleteProgramPipelines with n: {:?}, pipelines: {:?}", n, pipelines);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteQueries(n: types::GLsizei, ids: *const types::GLuint){
            println!("Call DeleteQueries with n: {:?}, ids: {:?}", n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteRenderbuffers(n: types::GLsizei, renderbuffers: *const types::GLuint){
            println!("Call DeleteRenderbuffers with n: {:?}, renderbuffers: {:?}", n, renderbuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteSamplers(count: types::GLsizei, samplers: *const types::GLuint){
            println!("Call DeleteSamplers with count: {:?}, samplers: {:?}", count, samplers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteShader(shader: types::GLuint){
            println!("Call DeleteShader with shader: {:?}", shader);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteSync(sync: types::GLsync){
            println!("Call DeleteSync with sync: {:?}", sync);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteTextures(n: types::GLsizei, textures: *const types::GLuint){
            println!("Call DeleteTextures with n: {:?}, textures: {:?}", n, textures);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteTransformFeedbacks(n: types::GLsizei, ids: *const types::GLuint){
            println!("Call DeleteTransformFeedbacks with n: {:?}, ids: {:?}", n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDeleteVertexArrays(n: types::GLsizei, arrays: *const types::GLuint){
            println!("Call DeleteVertexArrays with n: {:?}, arrays: {:?}", n, arrays);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthFunc(func: types::GLenum){
            println!("Call DepthFunc with func: {:?}", func);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthMask(flag: types::GLboolean){
            println!("Call DepthMask with flag: {:?}", flag);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthRange(n: types::GLdouble, f: types::GLdouble){
            println!("Call DepthRange with n: {:?}, f: {:?}", n, f);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthRangeArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLdouble){
            println!("Call DepthRangeArrayv with first: {:?}, count: {:?}, v: {:?}", first, count, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthRangeIndexed(index: types::GLuint, n: types::GLdouble, f: types::GLdouble){
            println!("Call DepthRangeIndexed with index: {:?}, n: {:?}, f: {:?}", index, n, f);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDepthRangef(n: types::GLfloat, f: types::GLfloat){
            println!("Call DepthRangef with n: {:?}, f: {:?}", n, f);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDetachShader(program: types::GLuint, shader: types::GLuint){
            println!("Call DetachShader with program: {:?}, shader: {:?}", program, shader);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDisable(cap: types::GLenum){
            println!("Call Disable with cap: {:?}", cap);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDisableVertexArrayAttrib(vaobj: types::GLuint, index: types::GLuint){
            println!("Call DisableVertexArrayAttrib with vaobj: {:?}, index: {:?}", vaobj, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDisableVertexAttribArray(index: types::GLuint){
            println!("Call DisableVertexAttribArray with index: {:?}", index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDisablei(target: types::GLenum, index: types::GLuint){
            println!("Call Disablei with target: {:?}, index: {:?}", target, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDispatchCompute(num_groups_x: types::GLuint, num_groups_y: types::GLuint, num_groups_z: types::GLuint){
            println!("Call DispatchCompute with num_groups_x: {:?}, num_groups_y: {:?}, num_groups_z: {:?}", num_groups_x, num_groups_y, num_groups_z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDispatchComputeIndirect(indirect: types::GLintptr){
            println!("Call DispatchComputeIndirect with indirect: {:?}", indirect);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei){
            println!("Call DrawArrays with mode: {:?}, first: {:?}, count: {:?}", mode, first, count);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawArraysIndirect(mode: types::GLenum, indirect: *const __gl_imports::raw::c_void){
            println!("Call DrawArraysIndirect with mode: {:?}, indirect: {:?}", mode, indirect);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawArraysInstanced(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei){
            println!("Call DrawArraysInstanced with mode: {:?}, first: {:?}, count: {:?}, instancecount: {:?}", mode, first, count, instancecount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawArraysInstancedBaseInstance(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei, baseinstance: types::GLuint){
            println!("Call DrawArraysInstancedBaseInstance with mode: {:?}, first: {:?}, count: {:?}, instancecount: {:?}, baseinstance: {:?}", mode, first, count, instancecount, baseinstance);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawBuffer(buf: types::GLenum){
            println!("Call DrawBuffer with buf: {:?}", buf);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawBuffers(n: types::GLsizei, bufs: *const types::GLenum){
            println!("Call DrawBuffers with n: {:?}, bufs: {:?}", n, bufs);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void){
            println!("Call DrawElements with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}", mode, count, type_, indices);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint){
            println!("Call DrawElementsBaseVertex with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, basevertex: {:?}", mode, count, type_, indices, basevertex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsIndirect(mode: types::GLenum, type_: types::GLenum, indirect: *const __gl_imports::raw::c_void){
            println!("Call DrawElementsIndirect with mode: {:?}, type_: {:?}, indirect: {:?}", mode, type_, indirect);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsInstanced(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei){
            println!("Call DrawElementsInstanced with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, instancecount: {:?}", mode, count, type_, indices, instancecount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsInstancedBaseInstance(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, baseinstance: types::GLuint){
            println!("Call DrawElementsInstancedBaseInstance with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, instancecount: {:?}, baseinstance: {:?}", mode, count, type_, indices, instancecount, baseinstance);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsInstancedBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint){
            println!("Call DrawElementsInstancedBaseVertex with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, instancecount: {:?}, basevertex: {:?}", mode, count, type_, indices, instancecount, basevertex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawElementsInstancedBaseVertexBaseInstance(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint, baseinstance: types::GLuint){
            println!("Call DrawElementsInstancedBaseVertexBaseInstance with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, instancecount: {:?}, basevertex: {:?}, baseinstance: {:?}", mode, count, type_, indices, instancecount, basevertex, baseinstance);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawRangeElements(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void){
            println!("Call DrawRangeElements with mode: {:?}, start: {:?}, end: {:?}, count: {:?}, type_: {:?}, indices: {:?}", mode, start, end, count, type_, indices);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawRangeElementsBaseVertex(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint){
            println!("Call DrawRangeElementsBaseVertex with mode: {:?}, start: {:?}, end: {:?}, count: {:?}, type_: {:?}, indices: {:?}, basevertex: {:?}", mode, start, end, count, type_, indices, basevertex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawTransformFeedback(mode: types::GLenum, id: types::GLuint){
            println!("Call DrawTransformFeedback with mode: {:?}, id: {:?}", mode, id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawTransformFeedbackInstanced(mode: types::GLenum, id: types::GLuint, instancecount: types::GLsizei){
            println!("Call DrawTransformFeedbackInstanced with mode: {:?}, id: {:?}, instancecount: {:?}", mode, id, instancecount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawTransformFeedbackStream(mode: types::GLenum, id: types::GLuint, stream: types::GLuint){
            println!("Call DrawTransformFeedbackStream with mode: {:?}, id: {:?}, stream: {:?}", mode, id, stream);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glDrawTransformFeedbackStreamInstanced(mode: types::GLenum, id: types::GLuint, stream: types::GLuint, instancecount: types::GLsizei){
            println!("Call DrawTransformFeedbackStreamInstanced with mode: {:?}, id: {:?}, stream: {:?}, instancecount: {:?}", mode, id, stream, instancecount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEnable(cap: types::GLenum){
            println!("Call Enable with cap: {:?}", cap);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEnableVertexArrayAttrib(vaobj: types::GLuint, index: types::GLuint){
            println!("Call EnableVertexArrayAttrib with vaobj: {:?}, index: {:?}", vaobj, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEnableVertexAttribArray(index: types::GLuint){
            println!("Call EnableVertexAttribArray with index: {:?}", index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEnablei(target: types::GLenum, index: types::GLuint){
            println!("Call Enablei with target: {:?}, index: {:?}", target, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEndConditionalRender(){
            println!("Call EndConditionalRender with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEndQuery(target: types::GLenum){
            println!("Call EndQuery with target: {:?}", target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEndQueryIndexed(target: types::GLenum, index: types::GLuint){
            println!("Call EndQueryIndexed with target: {:?}, index: {:?}", target, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glEndTransformFeedback(){
            println!("Call EndTransformFeedback with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFenceSync(condition: types::GLenum, flags: types::GLbitfield){
            println!("Call FenceSync with condition: {:?}, flags: {:?}", condition, flags);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFinish(){
            println!("Call Finish with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFlush(){
            println!("Call Flush with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFlushMappedBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr){
            println!("Call FlushMappedBufferRange with target: {:?}, offset: {:?}, length: {:?}", target, offset, length);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFlushMappedNamedBufferRange(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr){
            println!("Call FlushMappedNamedBufferRange with buffer: {:?}, offset: {:?}, length: {:?}", buffer, offset, length);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint){
            println!("Call FramebufferParameteri with target: {:?}, pname: {:?}, param: {:?}", target, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferRenderbuffer(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint){
            println!("Call FramebufferRenderbuffer with target: {:?}, attachment: {:?}, renderbuffertarget: {:?}, renderbuffer: {:?}", target, attachment, renderbuffertarget, renderbuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferTexture(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint){
            println!("Call FramebufferTexture with target: {:?}, attachment: {:?}, texture: {:?}, level: {:?}", target, attachment, texture, level);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferTexture1D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint){
            println!("Call FramebufferTexture1D with target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?}", target, attachment, textarget, texture, level);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferTexture2D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint){
            println!("Call FramebufferTexture2D with target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?}", target, attachment, textarget, texture, level);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferTexture3D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint, zoffset: types::GLint){
            println!("Call FramebufferTexture3D with target: {:?}, attachment: {:?}, textarget: {:?}, texture: {:?}, level: {:?}, zoffset: {:?}", target, attachment, textarget, texture, level, zoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFramebufferTextureLayer(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint){
            println!("Call FramebufferTextureLayer with target: {:?}, attachment: {:?}, texture: {:?}, level: {:?}, layer: {:?}", target, attachment, texture, level, layer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glFrontFace(mode: types::GLenum){
            println!("Call FrontFace with mode: {:?}", mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenBuffers(n: types::GLsizei, buffers: *mut types::GLuint){
            println!("Call GenBuffers with n: {:?}, buffers: {:?}", n, buffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint){
            println!("Call GenFramebuffers with n: {:?}, framebuffers: {:?}", n, framebuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenProgramPipelines(n: types::GLsizei, pipelines: *mut types::GLuint){
            println!("Call GenProgramPipelines with n: {:?}, pipelines: {:?}", n, pipelines);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenQueries(n: types::GLsizei, ids: *mut types::GLuint){
            println!("Call GenQueries with n: {:?}, ids: {:?}", n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint){
            println!("Call GenRenderbuffers with n: {:?}, renderbuffers: {:?}", n, renderbuffers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenSamplers(count: types::GLsizei, samplers: *mut types::GLuint){
            println!("Call GenSamplers with count: {:?}, samplers: {:?}", count, samplers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenTextures(n: types::GLsizei, textures: *mut types::GLuint){
            println!("Call GenTextures with n: {:?}, textures: {:?}", n, textures);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenTransformFeedbacks(n: types::GLsizei, ids: *mut types::GLuint){
            println!("Call GenTransformFeedbacks with n: {:?}, ids: {:?}", n, ids);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint){
            println!("Call GenVertexArrays with n: {:?}, arrays: {:?}", n, arrays);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenerateMipmap(target: types::GLenum){
            println!("Call GenerateMipmap with target: {:?}", target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGenerateTextureMipmap(texture: types::GLuint){
            println!("Call GenerateTextureMipmap with texture: {:?}", texture);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveAtomicCounterBufferiv(program: types::GLuint, bufferIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetActiveAtomicCounterBufferiv with program: {:?}, bufferIndex: {:?}, pname: {:?}, params: {:?}", program, bufferIndex, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveAttrib(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar){
            println!("Call GetActiveAttrib with program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, type_: {:?}, name: {:?}", program, index, bufSize, length, size, type_, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveSubroutineName(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar){
            println!("Call GetActiveSubroutineName with program: {:?}, shadertype: {:?}, index: {:?}, bufsize: {:?}, length: {:?}, name: {:?}", program, shadertype, index, bufsize, length, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveSubroutineUniformName(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar){
            println!("Call GetActiveSubroutineUniformName with program: {:?}, shadertype: {:?}, index: {:?}, bufsize: {:?}, length: {:?}, name: {:?}", program, shadertype, index, bufsize, length, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveSubroutineUniformiv(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, pname: types::GLenum, values: *mut types::GLint){
            println!("Call GetActiveSubroutineUniformiv with program: {:?}, shadertype: {:?}, index: {:?}, pname: {:?}, values: {:?}", program, shadertype, index, pname, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveUniform(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar){
            println!("Call GetActiveUniform with program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, type_: {:?}, name: {:?}", program, index, bufSize, length, size, type_, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveUniformBlockName(program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar){
            println!("Call GetActiveUniformBlockName with program: {:?}, uniformBlockIndex: {:?}, bufSize: {:?}, length: {:?}, uniformBlockName: {:?}", program, uniformBlockIndex, bufSize, length, uniformBlockName);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveUniformBlockiv(program: types::GLuint, uniformBlockIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetActiveUniformBlockiv with program: {:?}, uniformBlockIndex: {:?}, pname: {:?}, params: {:?}", program, uniformBlockIndex, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveUniformName(program: types::GLuint, uniformIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformName: *mut types::GLchar){
            println!("Call GetActiveUniformName with program: {:?}, uniformIndex: {:?}, bufSize: {:?}, length: {:?}, uniformName: {:?}", program, uniformIndex, bufSize, length, uniformName);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetActiveUniformsiv(program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetActiveUniformsiv with program: {:?}, uniformCount: {:?}, uniformIndices: {:?}, pname: {:?}, params: {:?}", program, uniformCount, uniformIndices, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetAttachedShaders(program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint){
            println!("Call GetAttachedShaders with program: {:?}, maxCount: {:?}, count: {:?}, shaders: {:?}", program, maxCount, count, shaders);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetAttribLocation(program: types::GLuint, name: *const types::GLchar){
            println!("Call GetAttribLocation with program: {:?}, name: {:?}", program, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBooleani_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLboolean){
            println!("Call GetBooleani_v with target: {:?}, index: {:?}, data: {:?}", target, index, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBooleanv(pname: types::GLenum, data: *mut types::GLboolean){
            println!("Call GetBooleanv with pname: {:?}, data: {:?}", pname, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBufferParameteri64v(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint64){
            println!("Call GetBufferParameteri64v with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetBufferParameteriv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBufferPointerv(target: types::GLenum, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void){
            println!("Call GetBufferPointerv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetBufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void){
            println!("Call GetBufferSubData with target: {:?}, offset: {:?}, size: {:?}, data: {:?}", target, offset, size, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetCompressedTexImage(target: types::GLenum, level: types::GLint, img: *mut __gl_imports::raw::c_void){
            println!("Call GetCompressedTexImage with target: {:?}, level: {:?}, img: {:?}", target, level, img);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetCompressedTextureImage(texture: types::GLuint, level: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetCompressedTextureImage with texture: {:?}, level: {:?}, bufSize: {:?}, pixels: {:?}", texture, level, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetCompressedTextureSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetCompressedTextureSubImage with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, bufSize: {:?}, pixels: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetDebugMessageLog(count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar){
            println!("Call GetDebugMessageLog with count: {:?}, bufSize: {:?}, sources: {:?}, types: {:?}, ids: {:?}, severities: {:?}, lengths: {:?}, messageLog: {:?}", count, bufSize, sources, types, ids, severities, lengths, messageLog);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetDoublei_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLdouble){
            println!("Call GetDoublei_v with target: {:?}, index: {:?}, data: {:?}", target, index, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetDoublev(pname: types::GLenum, data: *mut types::GLdouble){
            println!("Call GetDoublev with pname: {:?}, data: {:?}", pname, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetError(){
            println!("Call GetError with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFloati_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLfloat){
            println!("Call GetFloati_v with target: {:?}, index: {:?}, data: {:?}", target, index, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFloatv(pname: types::GLenum, data: *mut types::GLfloat){
            println!("Call GetFloatv with pname: {:?}, data: {:?}", pname, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFragDataIndex(program: types::GLuint, name: *const types::GLchar){
            println!("Call GetFragDataIndex with program: {:?}, name: {:?}", program, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFragDataLocation(program: types::GLuint, name: *const types::GLchar){
            println!("Call GetFragDataLocation with program: {:?}, name: {:?}", program, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFramebufferAttachmentParameteriv(target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetFramebufferAttachmentParameteriv with target: {:?}, attachment: {:?}, pname: {:?}, params: {:?}", target, attachment, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetFramebufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetFramebufferParameteriv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetGraphicsResetStatus(){
            println!("Call GetGraphicsResetStatus with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetInteger64i_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint64){
            println!("Call GetInteger64i_v with target: {:?}, index: {:?}, data: {:?}", target, index, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetInteger64v(pname: types::GLenum, data: *mut types::GLint64){
            println!("Call GetInteger64v with pname: {:?}, data: {:?}", pname, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetIntegeri_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint){
            println!("Call GetIntegeri_v with target: {:?}, index: {:?}, data: {:?}", target, index, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetIntegerv(pname: types::GLenum, data: *mut types::GLint){
            println!("Call GetIntegerv with pname: {:?}, data: {:?}", pname, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetInternalformati64v(target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint64){
            println!("Call GetInternalformati64v with target: {:?}, internalformat: {:?}, pname: {:?}, bufSize: {:?}, params: {:?}", target, internalformat, pname, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetInternalformativ(target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint){
            println!("Call GetInternalformativ with target: {:?}, internalformat: {:?}, pname: {:?}, bufSize: {:?}, params: {:?}", target, internalformat, pname, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetMultisamplefv(pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat){
            println!("Call GetMultisamplefv with pname: {:?}, index: {:?}, val: {:?}", pname, index, val);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedBufferParameteri64v(buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint64){
            println!("Call GetNamedBufferParameteri64v with buffer: {:?}, pname: {:?}, params: {:?}", buffer, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedBufferParameteriv(buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetNamedBufferParameteriv with buffer: {:?}, pname: {:?}, params: {:?}", buffer, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedBufferPointerv(buffer: types::GLuint, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void){
            println!("Call GetNamedBufferPointerv with buffer: {:?}, pname: {:?}, params: {:?}", buffer, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedBufferSubData(buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void){
            println!("Call GetNamedBufferSubData with buffer: {:?}, offset: {:?}, size: {:?}, data: {:?}", buffer, offset, size, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedFramebufferAttachmentParameteriv(framebuffer: types::GLuint, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetNamedFramebufferAttachmentParameteriv with framebuffer: {:?}, attachment: {:?}, pname: {:?}, params: {:?}", framebuffer, attachment, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedFramebufferParameteriv(framebuffer: types::GLuint, pname: types::GLenum, param: *mut types::GLint){
            println!("Call GetNamedFramebufferParameteriv with framebuffer: {:?}, pname: {:?}, param: {:?}", framebuffer, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetNamedRenderbufferParameteriv(renderbuffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetNamedRenderbufferParameteriv with renderbuffer: {:?}, pname: {:?}, params: {:?}", renderbuffer, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetObjectLabel(identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar){
            println!("Call GetObjectLabel with identifier: {:?}, name: {:?}, bufSize: {:?}, length: {:?}, label: {:?}", identifier, name, bufSize, length, label);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetObjectPtrLabel(ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar){
            println!("Call GetObjectPtrLabel with ptr: {:?}, bufSize: {:?}, length: {:?}, label: {:?}", ptr, bufSize, length, label);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetPointerv(pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void){
            println!("Call GetPointerv with pname: {:?}, params: {:?}", pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramBinary(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, binaryFormat: *mut types::GLenum, binary: *mut __gl_imports::raw::c_void){
            println!("Call GetProgramBinary with program: {:?}, bufSize: {:?}, length: {:?}, binaryFormat: {:?}, binary: {:?}", program, bufSize, length, binaryFormat, binary);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramInfoLog(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar){
            println!("Call GetProgramInfoLog with program: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?}", program, bufSize, length, infoLog);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramInterfaceiv(program: types::GLuint, programInterface: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetProgramInterfaceiv with program: {:?}, programInterface: {:?}, pname: {:?}, params: {:?}", program, programInterface, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramPipelineInfoLog(pipeline: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar){
            println!("Call GetProgramPipelineInfoLog with pipeline: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?}", pipeline, bufSize, length, infoLog);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramPipelineiv(pipeline: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetProgramPipelineiv with pipeline: {:?}, pname: {:?}, params: {:?}", pipeline, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramResourceIndex(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar){
            println!("Call GetProgramResourceIndex with program: {:?}, programInterface: {:?}, name: {:?}", program, programInterface, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramResourceLocation(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar){
            println!("Call GetProgramResourceLocation with program: {:?}, programInterface: {:?}, name: {:?}", program, programInterface, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramResourceLocationIndex(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar){
            println!("Call GetProgramResourceLocationIndex with program: {:?}, programInterface: {:?}, name: {:?}", program, programInterface, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramResourceName(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar){
            println!("Call GetProgramResourceName with program: {:?}, programInterface: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, name: {:?}", program, programInterface, index, bufSize, length, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramResourceiv(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, propCount: types::GLsizei, props: *const types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, params: *mut types::GLint){
            println!("Call GetProgramResourceiv with program: {:?}, programInterface: {:?}, index: {:?}, propCount: {:?}, props: {:?}, bufSize: {:?}, length: {:?}, params: {:?}", program, programInterface, index, propCount, props, bufSize, length, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramStageiv(program: types::GLuint, shadertype: types::GLenum, pname: types::GLenum, values: *mut types::GLint){
            println!("Call GetProgramStageiv with program: {:?}, shadertype: {:?}, pname: {:?}, values: {:?}", program, shadertype, pname, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetProgramiv(program: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetProgramiv with program: {:?}, pname: {:?}, params: {:?}", program, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryBufferObjecti64v(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr){
            println!("Call GetQueryBufferObjecti64v with id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?}", id, buffer, pname, offset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryBufferObjectiv(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr){
            println!("Call GetQueryBufferObjectiv with id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?}", id, buffer, pname, offset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryBufferObjectui64v(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr){
            println!("Call GetQueryBufferObjectui64v with id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?}", id, buffer, pname, offset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryBufferObjectuiv(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr){
            println!("Call GetQueryBufferObjectuiv with id: {:?}, buffer: {:?}, pname: {:?}, offset: {:?}", id, buffer, pname, offset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryIndexediv(target: types::GLenum, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetQueryIndexediv with target: {:?}, index: {:?}, pname: {:?}, params: {:?}", target, index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryObjecti64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint64){
            println!("Call GetQueryObjecti64v with id: {:?}, pname: {:?}, params: {:?}", id, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryObjectiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetQueryObjectiv with id: {:?}, pname: {:?}, params: {:?}", id, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryObjectui64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint64){
            println!("Call GetQueryObjectui64v with id: {:?}, pname: {:?}, params: {:?}", id, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryObjectuiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint){
            println!("Call GetQueryObjectuiv with id: {:?}, pname: {:?}, params: {:?}", id, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetQueryiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetQueryiv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetRenderbufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetRenderbufferParameteriv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetSamplerParameterIiv with sampler: {:?}, pname: {:?}, params: {:?}", sampler, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLuint){
            println!("Call GetSamplerParameterIuiv with sampler: {:?}, pname: {:?}, params: {:?}", sampler, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetSamplerParameterfv with sampler: {:?}, pname: {:?}, params: {:?}", sampler, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetSamplerParameteriv with sampler: {:?}, pname: {:?}, params: {:?}", sampler, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetShaderInfoLog(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar){
            println!("Call GetShaderInfoLog with shader: {:?}, bufSize: {:?}, length: {:?}, infoLog: {:?}", shader, bufSize, length, infoLog);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetShaderPrecisionFormat(shadertype: types::GLenum, precisiontype: types::GLenum, range: *mut types::GLint, precision: *mut types::GLint){
            println!("Call GetShaderPrecisionFormat with shadertype: {:?}, precisiontype: {:?}, range: {:?}, precision: {:?}", shadertype, precisiontype, range, precision);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetShaderSource(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar){
            println!("Call GetShaderSource with shader: {:?}, bufSize: {:?}, length: {:?}, source: {:?}", shader, bufSize, length, source);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetShaderiv(shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetShaderiv with shader: {:?}, pname: {:?}, params: {:?}", shader, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetString(name: types::GLenum){
            println!("Call GetString with name: {:?}", name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetStringi(name: types::GLenum, index: types::GLuint){
            println!("Call GetStringi with name: {:?}, index: {:?}", name, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSubroutineIndex(program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar){
            println!("Call GetSubroutineIndex with program: {:?}, shadertype: {:?}, name: {:?}", program, shadertype, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSubroutineUniformLocation(program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar){
            println!("Call GetSubroutineUniformLocation with program: {:?}, shadertype: {:?}, name: {:?}", program, shadertype, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetSynciv(sync: types::GLsync, pname: types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint){
            println!("Call GetSynciv with sync: {:?}, pname: {:?}, bufSize: {:?}, length: {:?}, values: {:?}", sync, pname, bufSize, length, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetTexImage with target: {:?}, level: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetTexLevelParameterfv with target: {:?}, level: {:?}, pname: {:?}, params: {:?}", target, level, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTexLevelParameteriv with target: {:?}, level: {:?}, pname: {:?}, params: {:?}", target, level, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTexParameterIiv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLuint){
            println!("Call GetTexParameterIuiv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetTexParameterfv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTexParameteriv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureImage(texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetTextureImage with texture: {:?}, level: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, pixels: {:?}", texture, level, format, type_, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureLevelParameterfv(texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetTextureLevelParameterfv with texture: {:?}, level: {:?}, pname: {:?}, params: {:?}", texture, level, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureLevelParameteriv(texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTextureLevelParameteriv with texture: {:?}, level: {:?}, pname: {:?}, params: {:?}", texture, level, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureParameterIiv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTextureParameterIiv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureParameterIuiv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLuint){
            println!("Call GetTextureParameterIuiv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureParameterfv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetTextureParameterfv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureParameteriv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetTextureParameteriv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTextureSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetTextureSubImage with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, pixels: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTransformFeedbackVarying(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar){
            println!("Call GetTransformFeedbackVarying with program: {:?}, index: {:?}, bufSize: {:?}, length: {:?}, size: {:?}, type_: {:?}, name: {:?}", program, index, bufSize, length, size, type_, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTransformFeedbacki64_v(xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint64){
            println!("Call GetTransformFeedbacki64_v with xfb: {:?}, pname: {:?}, index: {:?}, param: {:?}", xfb, pname, index, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTransformFeedbacki_v(xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint){
            println!("Call GetTransformFeedbacki_v with xfb: {:?}, pname: {:?}, index: {:?}, param: {:?}", xfb, pname, index, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetTransformFeedbackiv(xfb: types::GLuint, pname: types::GLenum, param: *mut types::GLint){
            println!("Call GetTransformFeedbackiv with xfb: {:?}, pname: {:?}, param: {:?}", xfb, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformBlockIndex(program: types::GLuint, uniformBlockName: *const types::GLchar){
            println!("Call GetUniformBlockIndex with program: {:?}, uniformBlockName: {:?}", program, uniformBlockName);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformIndices(program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint){
            println!("Call GetUniformIndices with program: {:?}, uniformCount: {:?}, uniformNames: {:?}, uniformIndices: {:?}", program, uniformCount, uniformNames, uniformIndices);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformLocation(program: types::GLuint, name: *const types::GLchar){
            println!("Call GetUniformLocation with program: {:?}, name: {:?}", program, name);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformSubroutineuiv(shadertype: types::GLenum, location: types::GLint, params: *mut types::GLuint){
            println!("Call GetUniformSubroutineuiv with shadertype: {:?}, location: {:?}, params: {:?}", shadertype, location, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformdv(program: types::GLuint, location: types::GLint, params: *mut types::GLdouble){
            println!("Call GetUniformdv with program: {:?}, location: {:?}, params: {:?}", program, location, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformfv(program: types::GLuint, location: types::GLint, params: *mut types::GLfloat){
            println!("Call GetUniformfv with program: {:?}, location: {:?}, params: {:?}", program, location, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformiv(program: types::GLuint, location: types::GLint, params: *mut types::GLint){
            println!("Call GetUniformiv with program: {:?}, location: {:?}, params: {:?}", program, location, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetUniformuiv(program: types::GLuint, location: types::GLint, params: *mut types::GLuint){
            println!("Call GetUniformuiv with program: {:?}, location: {:?}, params: {:?}", program, location, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexArrayIndexed64iv(vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint64){
            println!("Call GetVertexArrayIndexed64iv with vaobj: {:?}, index: {:?}, pname: {:?}, param: {:?}", vaobj, index, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexArrayIndexediv(vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint){
            println!("Call GetVertexArrayIndexediv with vaobj: {:?}, index: {:?}, pname: {:?}, param: {:?}", vaobj, index, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexArrayiv(vaobj: types::GLuint, pname: types::GLenum, param: *mut types::GLint){
            println!("Call GetVertexArrayiv with vaobj: {:?}, pname: {:?}, param: {:?}", vaobj, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribIiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetVertexAttribIiv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribIuiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLuint){
            println!("Call GetVertexAttribIuiv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribLdv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble){
            println!("Call GetVertexAttribLdv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribPointerv(index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void){
            println!("Call GetVertexAttribPointerv with index: {:?}, pname: {:?}, pointer: {:?}", index, pname, pointer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribdv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble){
            println!("Call GetVertexAttribdv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribfv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat){
            println!("Call GetVertexAttribfv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetVertexAttribiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint){
            println!("Call GetVertexAttribiv with index: {:?}, pname: {:?}, params: {:?}", index, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnColorTable(target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, table: *mut __gl_imports::raw::c_void){
            println!("Call GetnColorTable with target: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, table: {:?}", target, format, type_, bufSize, table);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnCompressedTexImage(target: types::GLenum, lod: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetnCompressedTexImage with target: {:?}, lod: {:?}, bufSize: {:?}, pixels: {:?}", target, lod, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnConvolutionFilter(target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, image: *mut __gl_imports::raw::c_void){
            println!("Call GetnConvolutionFilter with target: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, image: {:?}", target, format, type_, bufSize, image);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnHistogram(target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void){
            println!("Call GetnHistogram with target: {:?}, reset: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, values: {:?}", target, reset, format, type_, bufSize, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnMapdv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLdouble){
            println!("Call GetnMapdv with target: {:?}, query: {:?}, bufSize: {:?}, v: {:?}", target, query, bufSize, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnMapfv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLfloat){
            println!("Call GetnMapfv with target: {:?}, query: {:?}, bufSize: {:?}, v: {:?}", target, query, bufSize, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnMapiv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLint){
            println!("Call GetnMapiv with target: {:?}, query: {:?}, bufSize: {:?}, v: {:?}", target, query, bufSize, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnMinmax(target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void){
            println!("Call GetnMinmax with target: {:?}, reset: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, values: {:?}", target, reset, format, type_, bufSize, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnPixelMapfv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLfloat){
            println!("Call GetnPixelMapfv with map: {:?}, bufSize: {:?}, values: {:?}", map, bufSize, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnPixelMapuiv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLuint){
            println!("Call GetnPixelMapuiv with map: {:?}, bufSize: {:?}, values: {:?}", map, bufSize, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnPixelMapusv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLushort){
            println!("Call GetnPixelMapusv with map: {:?}, bufSize: {:?}, values: {:?}", map, bufSize, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnPolygonStipple(bufSize: types::GLsizei, pattern: *mut types::GLubyte){
            println!("Call GetnPolygonStipple with bufSize: {:?}, pattern: {:?}", bufSize, pattern);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnSeparableFilter(target: types::GLenum, format: types::GLenum, type_: types::GLenum, rowBufSize: types::GLsizei, row: *mut __gl_imports::raw::c_void, columnBufSize: types::GLsizei, column: *mut __gl_imports::raw::c_void, span: *mut __gl_imports::raw::c_void){
            println!("Call GetnSeparableFilter with target: {:?}, format: {:?}, type_: {:?}, rowBufSize: {:?}, row: {:?}, columnBufSize: {:?}, column: {:?}, span: {:?}", target, format, type_, rowBufSize, row, columnBufSize, column, span);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void){
            println!("Call GetnTexImage with target: {:?}, level: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, pixels: {:?}", target, level, format, type_, bufSize, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnUniformdv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLdouble){
            println!("Call GetnUniformdv with program: {:?}, location: {:?}, bufSize: {:?}, params: {:?}", program, location, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnUniformfv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLfloat){
            println!("Call GetnUniformfv with program: {:?}, location: {:?}, bufSize: {:?}, params: {:?}", program, location, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnUniformiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLint){
            println!("Call GetnUniformiv with program: {:?}, location: {:?}, bufSize: {:?}, params: {:?}", program, location, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glGetnUniformuiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLuint){
            println!("Call GetnUniformuiv with program: {:?}, location: {:?}, bufSize: {:?}, params: {:?}", program, location, bufSize, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glHint(target: types::GLenum, mode: types::GLenum){
            println!("Call Hint with target: {:?}, mode: {:?}", target, mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateBufferData(buffer: types::GLuint){
            println!("Call InvalidateBufferData with buffer: {:?}", buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateBufferSubData(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr){
            println!("Call InvalidateBufferSubData with buffer: {:?}, offset: {:?}, length: {:?}", buffer, offset, length);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum){
            println!("Call InvalidateFramebuffer with target: {:?}, numAttachments: {:?}, attachments: {:?}", target, numAttachments, attachments);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateNamedFramebufferData(framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum){
            println!("Call InvalidateNamedFramebufferData with framebuffer: {:?}, numAttachments: {:?}, attachments: {:?}", framebuffer, numAttachments, attachments);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateNamedFramebufferSubData(framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call InvalidateNamedFramebufferSubData with framebuffer: {:?}, numAttachments: {:?}, attachments: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", framebuffer, numAttachments, attachments, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateSubFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call InvalidateSubFramebuffer with target: {:?}, numAttachments: {:?}, attachments: {:?}, x: {:?}, y: {:?}, width: {:?}, height: {:?}", target, numAttachments, attachments, x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateTexImage(texture: types::GLuint, level: types::GLint){
            println!("Call InvalidateTexImage with texture: {:?}, level: {:?}", texture, level);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glInvalidateTexSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei){
            println!("Call InvalidateTexSubImage with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsBuffer(buffer: types::GLuint){
            println!("Call IsBuffer with buffer: {:?}", buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsEnabled(cap: types::GLenum){
            println!("Call IsEnabled with cap: {:?}", cap);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsEnabledi(target: types::GLenum, index: types::GLuint){
            println!("Call IsEnabledi with target: {:?}, index: {:?}", target, index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsFramebuffer(framebuffer: types::GLuint){
            println!("Call IsFramebuffer with framebuffer: {:?}", framebuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsProgram(program: types::GLuint){
            println!("Call IsProgram with program: {:?}", program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsProgramPipeline(pipeline: types::GLuint){
            println!("Call IsProgramPipeline with pipeline: {:?}", pipeline);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsQuery(id: types::GLuint){
            println!("Call IsQuery with id: {:?}", id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsRenderbuffer(renderbuffer: types::GLuint){
            println!("Call IsRenderbuffer with renderbuffer: {:?}", renderbuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsSampler(sampler: types::GLuint){
            println!("Call IsSampler with sampler: {:?}", sampler);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsShader(shader: types::GLuint){
            println!("Call IsShader with shader: {:?}", shader);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsSync(sync: types::GLsync){
            println!("Call IsSync with sync: {:?}", sync);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsTexture(texture: types::GLuint){
            println!("Call IsTexture with texture: {:?}", texture);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsTransformFeedback(id: types::GLuint){
            println!("Call IsTransformFeedback with id: {:?}", id);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glIsVertexArray(array: types::GLuint){
            println!("Call IsVertexArray with array: {:?}", array);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glLineWidth(width: types::GLfloat){
            println!("Call LineWidth with width: {:?}", width);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glLinkProgram(program: types::GLuint){
            println!("Call LinkProgram with program: {:?}", program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glLogicOp(opcode: types::GLenum){
            println!("Call LogicOp with opcode: {:?}", opcode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMapBuffer(target: types::GLenum, access: types::GLenum){
            println!("Call MapBuffer with target: {:?}, access: {:?}", target, access);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMapBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield){
            println!("Call MapBufferRange with target: {:?}, offset: {:?}, length: {:?}, access: {:?}", target, offset, length, access);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMapNamedBuffer(buffer: types::GLuint, access: types::GLenum){
            println!("Call MapNamedBuffer with buffer: {:?}, access: {:?}", buffer, access);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMapNamedBufferRange(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield){
            println!("Call MapNamedBufferRange with buffer: {:?}, offset: {:?}, length: {:?}, access: {:?}", buffer, offset, length, access);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMemoryBarrier(barriers: types::GLbitfield){
            println!("Call MemoryBarrier with barriers: {:?}", barriers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMemoryBarrierByRegion(barriers: types::GLbitfield){
            println!("Call MemoryBarrierByRegion with barriers: {:?}", barriers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMinSampleShading(value: types::GLfloat){
            println!("Call MinSampleShading with value: {:?}", value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiDrawArrays(mode: types::GLenum, first: *const types::GLint, count: *const types::GLsizei, drawcount: types::GLsizei){
            println!("Call MultiDrawArrays with mode: {:?}, first: {:?}, count: {:?}, drawcount: {:?}", mode, first, count, drawcount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiDrawArraysIndirect(mode: types::GLenum, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei){
            println!("Call MultiDrawArraysIndirect with mode: {:?}, indirect: {:?}, drawcount: {:?}, stride: {:?}", mode, indirect, drawcount, stride);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiDrawElements(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei){
            println!("Call MultiDrawElements with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, drawcount: {:?}", mode, count, type_, indices, drawcount);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiDrawElementsBaseVertex(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei, basevertex: *const types::GLint){
            println!("Call MultiDrawElementsBaseVertex with mode: {:?}, count: {:?}, type_: {:?}, indices: {:?}, drawcount: {:?}, basevertex: {:?}", mode, count, type_, indices, drawcount, basevertex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiDrawElementsIndirect(mode: types::GLenum, type_: types::GLenum, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei){
            println!("Call MultiDrawElementsIndirect with mode: {:?}, type_: {:?}, indirect: {:?}, drawcount: {:?}, stride: {:?}", mode, type_, indirect, drawcount, stride);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP1ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint){
            println!("Call MultiTexCoordP1ui with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP1uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint){
            println!("Call MultiTexCoordP1uiv with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP2ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint){
            println!("Call MultiTexCoordP2ui with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP2uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint){
            println!("Call MultiTexCoordP2uiv with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP3ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint){
            println!("Call MultiTexCoordP3ui with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP3uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint){
            println!("Call MultiTexCoordP3uiv with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP4ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint){
            println!("Call MultiTexCoordP4ui with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glMultiTexCoordP4uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint){
            println!("Call MultiTexCoordP4uiv with texture: {:?}, type_: {:?}, coords: {:?}", texture, type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedBufferData(buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum){
            println!("Call NamedBufferData with buffer: {:?}, size: {:?}, data: {:?}, usage: {:?}", buffer, size, data, usage);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedBufferStorage(buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: types::GLbitfield){
            println!("Call NamedBufferStorage with buffer: {:?}, size: {:?}, data: {:?}, flags: {:?}", buffer, size, data, flags);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedBufferSubData(buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void){
            println!("Call NamedBufferSubData with buffer: {:?}, offset: {:?}, size: {:?}, data: {:?}", buffer, offset, size, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferDrawBuffer(framebuffer: types::GLuint, buf: types::GLenum){
            println!("Call NamedFramebufferDrawBuffer with framebuffer: {:?}, buf: {:?}", framebuffer, buf);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferDrawBuffers(framebuffer: types::GLuint, n: types::GLsizei, bufs: *const types::GLenum){
            println!("Call NamedFramebufferDrawBuffers with framebuffer: {:?}, n: {:?}, bufs: {:?}", framebuffer, n, bufs);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferParameteri(framebuffer: types::GLuint, pname: types::GLenum, param: types::GLint){
            println!("Call NamedFramebufferParameteri with framebuffer: {:?}, pname: {:?}, param: {:?}", framebuffer, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferReadBuffer(framebuffer: types::GLuint, src: types::GLenum){
            println!("Call NamedFramebufferReadBuffer with framebuffer: {:?}, src: {:?}", framebuffer, src);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferRenderbuffer(framebuffer: types::GLuint, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint){
            println!("Call NamedFramebufferRenderbuffer with framebuffer: {:?}, attachment: {:?}, renderbuffertarget: {:?}, renderbuffer: {:?}", framebuffer, attachment, renderbuffertarget, renderbuffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferTexture(framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint){
            println!("Call NamedFramebufferTexture with framebuffer: {:?}, attachment: {:?}, texture: {:?}, level: {:?}", framebuffer, attachment, texture, level);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedFramebufferTextureLayer(framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint){
            println!("Call NamedFramebufferTextureLayer with framebuffer: {:?}, attachment: {:?}, texture: {:?}, level: {:?}, layer: {:?}", framebuffer, attachment, texture, level, layer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedRenderbufferStorage(renderbuffer: types::GLuint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call NamedRenderbufferStorage with renderbuffer: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", renderbuffer, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNamedRenderbufferStorageMultisample(renderbuffer: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call NamedRenderbufferStorageMultisample with renderbuffer: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", renderbuffer, samples, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNormalP3ui(type_: types::GLenum, coords: types::GLuint){
            println!("Call NormalP3ui with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glNormalP3uiv(type_: types::GLenum, coords: *const types::GLuint){
            println!("Call NormalP3uiv with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glObjectLabel(identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar){
            println!("Call ObjectLabel with identifier: {:?}, name: {:?}, length: {:?}, label: {:?}", identifier, name, length, label);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glObjectPtrLabel(ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar){
            println!("Call ObjectPtrLabel with ptr: {:?}, length: {:?}, label: {:?}", ptr, length, label);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPatchParameterfv(pname: types::GLenum, values: *const types::GLfloat){
            println!("Call PatchParameterfv with pname: {:?}, values: {:?}", pname, values);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPatchParameteri(pname: types::GLenum, value: types::GLint){
            println!("Call PatchParameteri with pname: {:?}, value: {:?}", pname, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPauseTransformFeedback(){
            println!("Call PauseTransformFeedback with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPixelStoref(pname: types::GLenum, param: types::GLfloat){
            println!("Call PixelStoref with pname: {:?}, param: {:?}", pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPixelStorei(pname: types::GLenum, param: types::GLint){
            println!("Call PixelStorei with pname: {:?}, param: {:?}", pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPointParameterf(pname: types::GLenum, param: types::GLfloat){
            println!("Call PointParameterf with pname: {:?}, param: {:?}", pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPointParameterfv(pname: types::GLenum, params: *const types::GLfloat){
            println!("Call PointParameterfv with pname: {:?}, params: {:?}", pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPointParameteri(pname: types::GLenum, param: types::GLint){
            println!("Call PointParameteri with pname: {:?}, param: {:?}", pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPointParameteriv(pname: types::GLenum, params: *const types::GLint){
            println!("Call PointParameteriv with pname: {:?}, params: {:?}", pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPointSize(size: types::GLfloat){
            println!("Call PointSize with size: {:?}", size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPolygonMode(face: types::GLenum, mode: types::GLenum){
            println!("Call PolygonMode with face: {:?}, mode: {:?}", face, mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPolygonOffset(factor: types::GLfloat, units: types::GLfloat){
            println!("Call PolygonOffset with factor: {:?}, units: {:?}", factor, units);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPopDebugGroup(){
            println!("Call PopDebugGroup with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPrimitiveRestartIndex(index: types::GLuint){
            println!("Call PrimitiveRestartIndex with index: {:?}", index);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramBinary(program: types::GLuint, binaryFormat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei){
            println!("Call ProgramBinary with program: {:?}, binaryFormat: {:?}, binary: {:?}, length: {:?}", program, binaryFormat, binary, length);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramParameteri(program: types::GLuint, pname: types::GLenum, value: types::GLint){
            println!("Call ProgramParameteri with program: {:?}, pname: {:?}, value: {:?}", program, pname, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1d(program: types::GLuint, location: types::GLint, v0: types::GLdouble){
            println!("Call ProgramUniform1d with program: {:?}, location: {:?}, v0: {:?}", program, location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call ProgramUniform1dv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1f(program: types::GLuint, location: types::GLint, v0: types::GLfloat){
            println!("Call ProgramUniform1f with program: {:?}, location: {:?}, v0: {:?}", program, location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call ProgramUniform1fv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1i(program: types::GLuint, location: types::GLint, v0: types::GLint){
            println!("Call ProgramUniform1i with program: {:?}, location: {:?}, v0: {:?}", program, location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call ProgramUniform1iv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1ui(program: types::GLuint, location: types::GLint, v0: types::GLuint){
            println!("Call ProgramUniform1ui with program: {:?}, location: {:?}, v0: {:?}", program, location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform1uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call ProgramUniform1uiv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble){
            println!("Call ProgramUniform2d with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}", program, location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call ProgramUniform2dv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat){
            println!("Call ProgramUniform2f with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}", program, location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call ProgramUniform2fv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint){
            println!("Call ProgramUniform2i with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}", program, location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call ProgramUniform2iv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint){
            println!("Call ProgramUniform2ui with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}", program, location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform2uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call ProgramUniform2uiv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble){
            println!("Call ProgramUniform3d with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", program, location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call ProgramUniform3dv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat){
            println!("Call ProgramUniform3f with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", program, location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call ProgramUniform3fv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint){
            println!("Call ProgramUniform3i with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", program, location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call ProgramUniform3iv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint){
            println!("Call ProgramUniform3ui with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", program, location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform3uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call ProgramUniform3uiv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble, v3: types::GLdouble){
            println!("Call ProgramUniform4d with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", program, location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call ProgramUniform4dv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat){
            println!("Call ProgramUniform4f with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", program, location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call ProgramUniform4fv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint){
            println!("Call ProgramUniform4i with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", program, location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call ProgramUniform4iv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint){
            println!("Call ProgramUniform4ui with program: {:?}, location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", program, location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniform4uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call ProgramUniform4uiv with program: {:?}, location: {:?}, count: {:?}, value: {:?}", program, location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix2dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix2fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2x3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix2x3dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix2x3fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2x4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix2x4dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix2x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix2x4fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix3dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix3fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3x2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix3x2dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix3x2fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3x4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix3x4dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix3x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix3x4fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix4dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix4fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4x2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix4x2dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix4x2fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4x3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call ProgramUniformMatrix4x3dv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProgramUniformMatrix4x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call ProgramUniformMatrix4x3fv with program: {:?}, location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", program, location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glProvokingVertex(mode: types::GLenum){
            println!("Call ProvokingVertex with mode: {:?}", mode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glPushDebugGroup(source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar){
            println!("Call PushDebugGroup with source: {:?}, id: {:?}, length: {:?}, message: {:?}", source, id, length, message);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glQueryCounter(id: types::GLuint, target: types::GLenum){
            println!("Call QueryCounter with id: {:?}, target: {:?}", id, target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glReadBuffer(src: types::GLenum){
            println!("Call ReadBuffer with src: {:?}", src);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void){
            println!("Call ReadPixels with x: {:?}, y: {:?}, width: {:?}, height: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", x, y, width, height, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glReadnPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, data: *mut __gl_imports::raw::c_void){
            println!("Call ReadnPixels with x: {:?}, y: {:?}, width: {:?}, height: {:?}, format: {:?}, type_: {:?}, bufSize: {:?}, data: {:?}", x, y, width, height, format, type_, bufSize, data);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glReleaseShaderCompiler(){
            println!("Call ReleaseShaderCompiler with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glRenderbufferStorage(target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call RenderbufferStorage with target: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", target, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glRenderbufferStorageMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call RenderbufferStorageMultisample with target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", target, samples, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glResumeTransformFeedback(){
            println!("Call ResumeTransformFeedback with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSampleCoverage(value: types::GLfloat, invert: types::GLboolean){
            println!("Call SampleCoverage with value: {:?}, invert: {:?}", value, invert);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSampleMaski(maskNumber: types::GLuint, mask: types::GLbitfield){
            println!("Call SampleMaski with maskNumber: {:?}, mask: {:?}", maskNumber, mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint){
            println!("Call SamplerParameterIiv with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLuint){
            println!("Call SamplerParameterIuiv with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameterf(sampler: types::GLuint, pname: types::GLenum, param: types::GLfloat){
            println!("Call SamplerParameterf with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLfloat){
            println!("Call SamplerParameterfv with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameteri(sampler: types::GLuint, pname: types::GLenum, param: types::GLint){
            println!("Call SamplerParameteri with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint){
            println!("Call SamplerParameteriv with sampler: {:?}, pname: {:?}, param: {:?}", sampler, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glScissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call Scissor with x: {:?}, y: {:?}, width: {:?}, height: {:?}", x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glScissorArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLint){
            println!("Call ScissorArrayv with first: {:?}, count: {:?}, v: {:?}", first, count, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glScissorIndexed(index: types::GLuint, left: types::GLint, bottom: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call ScissorIndexed with index: {:?}, left: {:?}, bottom: {:?}, width: {:?}, height: {:?}", index, left, bottom, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glScissorIndexedv(index: types::GLuint, v: *const types::GLint){
            println!("Call ScissorIndexedv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSecondaryColorP3ui(type_: types::GLenum, color: types::GLuint){
            println!("Call SecondaryColorP3ui with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glSecondaryColorP3uiv(type_: types::GLenum, color: *const types::GLuint){
            println!("Call SecondaryColorP3uiv with type_: {:?}, color: {:?}", type_, color);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glShaderBinary(count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei){
            println!("Call ShaderBinary with count: {:?}, shaders: {:?}, binaryformat: {:?}, binary: {:?}, length: {:?}", count, shaders, binaryformat, binary, length);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint){
            println!("Call ShaderSource");
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glShaderStorageBlockBinding(program: types::GLuint, storageBlockIndex: types::GLuint, storageBlockBinding: types::GLuint){
            println!("Call ShaderStorageBlockBinding with program: {:?}, storageBlockIndex: {:?}, storageBlockBinding: {:?}", program, storageBlockIndex, storageBlockBinding);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint){
            println!("Call StencilFunc with func: {:?}, ref_: {:?}, mask: {:?}", func, ref_, mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilFuncSeparate(face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint){
            println!("Call StencilFuncSeparate with face: {:?}, func: {:?}, ref_: {:?}, mask: {:?}", face, func, ref_, mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilMask(mask: types::GLuint){
            println!("Call StencilMask with mask: {:?}", mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilMaskSeparate(face: types::GLenum, mask: types::GLuint){
            println!("Call StencilMaskSeparate with face: {:?}, mask: {:?}", face, mask);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum){
            println!("Call StencilOp with fail: {:?}, zfail: {:?}, zpass: {:?}", fail, zfail, zpass);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glStencilOpSeparate(face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum){
            println!("Call StencilOpSeparate with face: {:?}, sfail: {:?}, dpfail: {:?}, dppass: {:?}", face, sfail, dpfail, dppass);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexBuffer(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint){
            println!("Call TexBuffer with target: {:?}, internalformat: {:?}, buffer: {:?}", target, internalformat, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexBufferRange(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call TexBufferRange with target: {:?}, internalformat: {:?}, buffer: {:?}, offset: {:?}, size: {:?}", target, internalformat, buffer, offset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP1ui(type_: types::GLenum, coords: types::GLuint){
            println!("Call TexCoordP1ui with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP1uiv(type_: types::GLenum, coords: *const types::GLuint){
            println!("Call TexCoordP1uiv with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP2ui(type_: types::GLenum, coords: types::GLuint){
            println!("Call TexCoordP2ui with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP2uiv(type_: types::GLenum, coords: *const types::GLuint){
            println!("Call TexCoordP2uiv with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP3ui(type_: types::GLenum, coords: types::GLuint){
            println!("Call TexCoordP3ui with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP3uiv(type_: types::GLenum, coords: *const types::GLuint){
            println!("Call TexCoordP3uiv with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP4ui(type_: types::GLenum, coords: types::GLuint){
            println!("Call TexCoordP4ui with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexCoordP4uiv(type_: types::GLenum, coords: *const types::GLuint){
            println!("Call TexCoordP4uiv with type_: {:?}, coords: {:?}", type_, coords);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexImage1D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, border: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, internalformat, width, border, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexImage2D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, border: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, internalformat, width, height, border, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexImage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TexImage2DMultisample with target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?}", target, samples, internalformat, width, height, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexImage3D with target: {:?}, level: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, border: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, internalformat, width, height, depth, border, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexImage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TexImage3DMultisample with target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?}", target, samples, internalformat, width, height, depth, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint){
            println!("Call TexParameterIiv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLuint){
            println!("Call TexParameterIuiv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat){
            println!("Call TexParameterf with target: {:?}, pname: {:?}, param: {:?}", target, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat){
            println!("Call TexParameterfv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint){
            println!("Call TexParameteri with target: {:?}, pname: {:?}, param: {:?}", target, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint){
            println!("Call TexParameteriv with target: {:?}, pname: {:?}, params: {:?}", target, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexStorage1D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei){
            println!("Call TexStorage1D with target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}", target, levels, internalformat, width);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexStorage2D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call TexStorage2D with target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", target, levels, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexStorage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TexStorage2DMultisample with target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?}", target, samples, internalformat, width, height, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexStorage3D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei){
            println!("Call TexStorage3D with target: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}", target, levels, internalformat, width, height, depth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexStorage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TexStorage3DMultisample with target: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?}", target, samples, internalformat, width, height, depth, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexSubImage1D with target: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, xoffset, width, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexSubImage2D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, xoffset, yoffset, width, height, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TexSubImage3D with target: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureBarrier(){
            println!("Call TextureBarrier with ", );
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureBuffer(texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint){
            println!("Call TextureBuffer with texture: {:?}, internalformat: {:?}, buffer: {:?}", texture, internalformat, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureBufferRange(texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call TextureBufferRange with texture: {:?}, internalformat: {:?}, buffer: {:?}, offset: {:?}, size: {:?}", texture, internalformat, buffer, offset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameterIiv(texture: types::GLuint, pname: types::GLenum, params: *const types::GLint){
            println!("Call TextureParameterIiv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameterIuiv(texture: types::GLuint, pname: types::GLenum, params: *const types::GLuint){
            println!("Call TextureParameterIuiv with texture: {:?}, pname: {:?}, params: {:?}", texture, pname, params);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameterf(texture: types::GLuint, pname: types::GLenum, param: types::GLfloat){
            println!("Call TextureParameterf with texture: {:?}, pname: {:?}, param: {:?}", texture, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameterfv(texture: types::GLuint, pname: types::GLenum, param: *const types::GLfloat){
            println!("Call TextureParameterfv with texture: {:?}, pname: {:?}, param: {:?}", texture, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameteri(texture: types::GLuint, pname: types::GLenum, param: types::GLint){
            println!("Call TextureParameteri with texture: {:?}, pname: {:?}, param: {:?}", texture, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureParameteriv(texture: types::GLuint, pname: types::GLenum, param: *const types::GLint){
            println!("Call TextureParameteriv with texture: {:?}, pname: {:?}, param: {:?}", texture, pname, param);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureStorage1D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei){
            println!("Call TextureStorage1D with texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}", texture, levels, internalformat, width);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureStorage2D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei){
            println!("Call TextureStorage2D with texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}", texture, levels, internalformat, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureStorage2DMultisample(texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TextureStorage2DMultisample with texture: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, fixedsamplelocations: {:?}", texture, samples, internalformat, width, height, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureStorage3D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei){
            println!("Call TextureStorage3D with texture: {:?}, levels: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}", texture, levels, internalformat, width, height, depth);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureStorage3DMultisample(texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean){
            println!("Call TextureStorage3DMultisample with texture: {:?}, samples: {:?}, internalformat: {:?}, width: {:?}, height: {:?}, depth: {:?}, fixedsamplelocations: {:?}", texture, samples, internalformat, width, height, depth, fixedsamplelocations);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TextureSubImage1D with texture: {:?}, level: {:?}, xoffset: {:?}, width: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", texture, level, xoffset, width, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TextureSubImage2D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, width: {:?}, height: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", texture, level, xoffset, yoffset, width, height, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void){
            println!("Call TextureSubImage3D with texture: {:?}, level: {:?}, xoffset: {:?}, yoffset: {:?}, zoffset: {:?}, width: {:?}, height: {:?}, depth: {:?}, format: {:?}, type_: {:?}, pixels: {:?}", texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTextureView(texture: types::GLuint, target: types::GLenum, origtexture: types::GLuint, internalformat: types::GLenum, minlevel: types::GLuint, numlevels: types::GLuint, minlayer: types::GLuint, numlayers: types::GLuint){
            println!("Call TextureView with texture: {:?}, target: {:?}, origtexture: {:?}, internalformat: {:?}, minlevel: {:?}, numlevels: {:?}, minlayer: {:?}, numlayers: {:?}", texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTransformFeedbackBufferBase(xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint){
            println!("Call TransformFeedbackBufferBase with xfb: {:?}, index: {:?}, buffer: {:?}", xfb, index, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTransformFeedbackBufferRange(xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr){
            println!("Call TransformFeedbackBufferRange with xfb: {:?}, index: {:?}, buffer: {:?}, offset: {:?}, size: {:?}", xfb, index, buffer, offset, size);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glTransformFeedbackVaryings(program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum){
            println!("Call TransformFeedbackVaryings with program: {:?}, count: {:?}, varyings: {:?}, bufferMode: {:?}", program, count, varyings, bufferMode);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1d(location: types::GLint, x: types::GLdouble){
            println!("Call Uniform1d with location: {:?}, x: {:?}", location, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call Uniform1dv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1f(location: types::GLint, v0: types::GLfloat){
            println!("Call Uniform1f with location: {:?}, v0: {:?}", location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call Uniform1fv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1i(location: types::GLint, v0: types::GLint){
            println!("Call Uniform1i with location: {:?}, v0: {:?}", location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call Uniform1iv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1ui(location: types::GLint, v0: types::GLuint){
            println!("Call Uniform1ui with location: {:?}, v0: {:?}", location, v0);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform1uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call Uniform1uiv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2d(location: types::GLint, x: types::GLdouble, y: types::GLdouble){
            println!("Call Uniform2d with location: {:?}, x: {:?}, y: {:?}", location, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call Uniform2dv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat){
            println!("Call Uniform2f with location: {:?}, v0: {:?}, v1: {:?}", location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call Uniform2fv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2i(location: types::GLint, v0: types::GLint, v1: types::GLint){
            println!("Call Uniform2i with location: {:?}, v0: {:?}, v1: {:?}", location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call Uniform2iv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint){
            println!("Call Uniform2ui with location: {:?}, v0: {:?}, v1: {:?}", location, v0, v1);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform2uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call Uniform2uiv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3d(location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble){
            println!("Call Uniform3d with location: {:?}, x: {:?}, y: {:?}, z: {:?}", location, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call Uniform3dv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat){
            println!("Call Uniform3f with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call Uniform3fv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint){
            println!("Call Uniform3i with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call Uniform3iv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint){
            println!("Call Uniform3ui with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}", location, v0, v1, v2);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform3uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call Uniform3uiv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4d(location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble){
            println!("Call Uniform4d with location: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", location, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble){
            println!("Call Uniform4dv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat){
            println!("Call Uniform4f with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat){
            println!("Call Uniform4fv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint){
            println!("Call Uniform4i with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint){
            println!("Call Uniform4iv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint){
            println!("Call Uniform4ui with location: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?}", location, v0, v1, v2, v3);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniform4uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint){
            println!("Call Uniform4uiv with location: {:?}, count: {:?}, value: {:?}", location, count, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformBlockBinding(program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint){
            println!("Call UniformBlockBinding with program: {:?}, uniformBlockIndex: {:?}, uniformBlockBinding: {:?}", program, uniformBlockIndex, uniformBlockBinding);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix2dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix2fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2x3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix2x3dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix2x3fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2x4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix2x4dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix2x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix2x4fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix3dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix3fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3x2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix3x2dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix3x2fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3x4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix3x4dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix3x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix3x4fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix4dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix4fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4x2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix4x2dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix4x2fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4x3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble){
            println!("Call UniformMatrix4x3dv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformMatrix4x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat){
            println!("Call UniformMatrix4x3fv with location: {:?}, count: {:?}, transpose: {:?}, value: {:?}", location, count, transpose, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUniformSubroutinesuiv(shadertype: types::GLenum, count: types::GLsizei, indices: *const types::GLuint){
            println!("Call UniformSubroutinesuiv with shadertype: {:?}, count: {:?}, indices: {:?}", shadertype, count, indices);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUnmapBuffer(target: types::GLenum){
            println!("Call UnmapBuffer with target: {:?}", target);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUnmapNamedBuffer(buffer: types::GLuint){
            println!("Call UnmapNamedBuffer with buffer: {:?}", buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUseProgram(program: types::GLuint){
            println!("Call UseProgram with program: {:?}", program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glUseProgramStages(pipeline: types::GLuint, stages: types::GLbitfield, program: types::GLuint){
            println!("Call UseProgramStages with pipeline: {:?}, stages: {:?}, program: {:?}", pipeline, stages, program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glValidateProgram(program: types::GLuint){
            println!("Call ValidateProgram with program: {:?}", program);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glValidateProgramPipeline(pipeline: types::GLuint){
            println!("Call ValidateProgramPipeline with pipeline: {:?}", pipeline);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayAttribBinding(vaobj: types::GLuint, attribindex: types::GLuint, bindingindex: types::GLuint){
            println!("Call VertexArrayAttribBinding with vaobj: {:?}, attribindex: {:?}, bindingindex: {:?}", vaobj, attribindex, bindingindex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayAttribFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint){
            println!("Call VertexArrayAttribFormat with vaobj: {:?}, attribindex: {:?}, size: {:?}, type_: {:?}, normalized: {:?}, relativeoffset: {:?}", vaobj, attribindex, size, type_, normalized, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayAttribIFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint){
            println!("Call VertexArrayAttribIFormat with vaobj: {:?}, attribindex: {:?}, size: {:?}, type_: {:?}, relativeoffset: {:?}", vaobj, attribindex, size, type_, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayAttribLFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint){
            println!("Call VertexArrayAttribLFormat with vaobj: {:?}, attribindex: {:?}, size: {:?}, type_: {:?}, relativeoffset: {:?}", vaobj, attribindex, size, type_, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayBindingDivisor(vaobj: types::GLuint, bindingindex: types::GLuint, divisor: types::GLuint){
            println!("Call VertexArrayBindingDivisor with vaobj: {:?}, bindingindex: {:?}, divisor: {:?}", vaobj, bindingindex, divisor);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayElementBuffer(vaobj: types::GLuint, buffer: types::GLuint){
            println!("Call VertexArrayElementBuffer with vaobj: {:?}, buffer: {:?}", vaobj, buffer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayVertexBuffer(vaobj: types::GLuint, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei){
            println!("Call VertexArrayVertexBuffer with vaobj: {:?}, bindingindex: {:?}, buffer: {:?}, offset: {:?}, stride: {:?}", vaobj, bindingindex, buffer, offset, stride);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexArrayVertexBuffers(vaobj: types::GLuint, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei){
            println!("Call VertexArrayVertexBuffers with vaobj: {:?}, first: {:?}, count: {:?}, buffers: {:?}, offsets: {:?}, strides: {:?}", vaobj, first, count, buffers, offsets, strides);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1d(index: types::GLuint, x: types::GLdouble){
            println!("Call VertexAttrib1d with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttrib1dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1f(index: types::GLuint, x: types::GLfloat){
            println!("Call VertexAttrib1f with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1fv(index: types::GLuint, v: *const types::GLfloat){
            println!("Call VertexAttrib1fv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1s(index: types::GLuint, x: types::GLshort){
            println!("Call VertexAttrib1s with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib1sv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttrib1sv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble){
            println!("Call VertexAttrib2d with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttrib2dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat){
            println!("Call VertexAttrib2f with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2fv(index: types::GLuint, v: *const types::GLfloat){
            println!("Call VertexAttrib2fv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2s(index: types::GLuint, x: types::GLshort, y: types::GLshort){
            println!("Call VertexAttrib2s with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib2sv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttrib2sv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble){
            println!("Call VertexAttrib3d with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttrib3dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat){
            println!("Call VertexAttrib3f with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3fv(index: types::GLuint, v: *const types::GLfloat){
            println!("Call VertexAttrib3fv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort){
            println!("Call VertexAttrib3s with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib3sv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttrib3sv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nbv(index: types::GLuint, v: *const types::GLbyte){
            println!("Call VertexAttrib4Nbv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Niv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttrib4Niv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nsv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttrib4Nsv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nub(index: types::GLuint, x: types::GLubyte, y: types::GLubyte, z: types::GLubyte, w: types::GLubyte){
            println!("Call VertexAttrib4Nub with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nubv(index: types::GLuint, v: *const types::GLubyte){
            println!("Call VertexAttrib4Nubv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nuiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttrib4Nuiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4Nusv(index: types::GLuint, v: *const types::GLushort){
            println!("Call VertexAttrib4Nusv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4bv(index: types::GLuint, v: *const types::GLbyte){
            println!("Call VertexAttrib4bv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble){
            println!("Call VertexAttrib4d with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttrib4dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat){
            println!("Call VertexAttrib4f with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4fv(index: types::GLuint, v: *const types::GLfloat){
            println!("Call VertexAttrib4fv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4iv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttrib4iv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort){
            println!("Call VertexAttrib4s with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4sv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttrib4sv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4ubv(index: types::GLuint, v: *const types::GLubyte){
            println!("Call VertexAttrib4ubv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4uiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttrib4uiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttrib4usv(index: types::GLuint, v: *const types::GLushort){
            println!("Call VertexAttrib4usv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribBinding(attribindex: types::GLuint, bindingindex: types::GLuint){
            println!("Call VertexAttribBinding with attribindex: {:?}, bindingindex: {:?}", attribindex, bindingindex);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribDivisor(index: types::GLuint, divisor: types::GLuint){
            println!("Call VertexAttribDivisor with index: {:?}, divisor: {:?}", index, divisor);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint){
            println!("Call VertexAttribFormat with attribindex: {:?}, size: {:?}, type_: {:?}, normalized: {:?}, relativeoffset: {:?}", attribindex, size, type_, normalized, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI1i(index: types::GLuint, x: types::GLint){
            println!("Call VertexAttribI1i with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI1iv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttribI1iv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI1ui(index: types::GLuint, x: types::GLuint){
            println!("Call VertexAttribI1ui with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI1uiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttribI1uiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI2i(index: types::GLuint, x: types::GLint, y: types::GLint){
            println!("Call VertexAttribI2i with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI2iv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttribI2iv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI2ui(index: types::GLuint, x: types::GLuint, y: types::GLuint){
            println!("Call VertexAttribI2ui with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI2uiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttribI2uiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI3i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint){
            println!("Call VertexAttribI3i with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI3iv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttribI3iv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI3ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint){
            println!("Call VertexAttribI3ui with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI3uiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttribI3uiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4bv(index: types::GLuint, v: *const types::GLbyte){
            println!("Call VertexAttribI4bv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint){
            println!("Call VertexAttribI4i with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4iv(index: types::GLuint, v: *const types::GLint){
            println!("Call VertexAttribI4iv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4sv(index: types::GLuint, v: *const types::GLshort){
            println!("Call VertexAttribI4sv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4ubv(index: types::GLuint, v: *const types::GLubyte){
            println!("Call VertexAttribI4ubv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint){
            println!("Call VertexAttribI4ui with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4uiv(index: types::GLuint, v: *const types::GLuint){
            println!("Call VertexAttribI4uiv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribI4usv(index: types::GLuint, v: *const types::GLushort){
            println!("Call VertexAttribI4usv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribIFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint){
            println!("Call VertexAttribIFormat with attribindex: {:?}, size: {:?}, type_: {:?}, relativeoffset: {:?}", attribindex, size, type_, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribIPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void){
            println!("Call VertexAttribIPointer with index: {:?}, size: {:?}, type_: {:?}, stride: {:?}, pointer: {:?}", index, size, type_, stride, pointer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL1d(index: types::GLuint, x: types::GLdouble){
            println!("Call VertexAttribL1d with index: {:?}, x: {:?}", index, x);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL1dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttribL1dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL2d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble){
            println!("Call VertexAttribL2d with index: {:?}, x: {:?}, y: {:?}", index, x, y);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL2dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttribL2dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL3d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble){
            println!("Call VertexAttribL3d with index: {:?}, x: {:?}, y: {:?}, z: {:?}", index, x, y, z);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL3dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttribL3dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL4d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble){
            println!("Call VertexAttribL4d with index: {:?}, x: {:?}, y: {:?}, z: {:?}, w: {:?}", index, x, y, z, w);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribL4dv(index: types::GLuint, v: *const types::GLdouble){
            println!("Call VertexAttribL4dv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribLFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint){
            println!("Call VertexAttribLFormat with attribindex: {:?}, size: {:?}, type_: {:?}, relativeoffset: {:?}", attribindex, size, type_, relativeoffset);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribLPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void){
            println!("Call VertexAttribLPointer with index: {:?}, size: {:?}, type_: {:?}, stride: {:?}, pointer: {:?}", index, size, type_, stride, pointer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP1ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint){
            println!("Call VertexAttribP1ui with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP1uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint){
            println!("Call VertexAttribP1uiv with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP2ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint){
            println!("Call VertexAttribP2ui with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP2uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint){
            println!("Call VertexAttribP2uiv with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP3ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint){
            println!("Call VertexAttribP3ui with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP3uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint){
            println!("Call VertexAttribP3uiv with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP4ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint){
            println!("Call VertexAttribP4ui with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribP4uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint){
            println!("Call VertexAttribP4uiv with index: {:?}, type_: {:?}, normalized: {:?}, value: {:?}", index, type_, normalized, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void){
            println!("Call VertexAttribPointer with index: {:?}, size: {:?}, type_: {:?}, normalized: {:?}, stride: {:?}, pointer: {:?}", index, size, type_, normalized, stride, pointer);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexBindingDivisor(bindingindex: types::GLuint, divisor: types::GLuint){
            println!("Call VertexBindingDivisor with bindingindex: {:?}, divisor: {:?}", bindingindex, divisor);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP2ui(type_: types::GLenum, value: types::GLuint){
            println!("Call VertexP2ui with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP2uiv(type_: types::GLenum, value: *const types::GLuint){
            println!("Call VertexP2uiv with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP3ui(type_: types::GLenum, value: types::GLuint){
            println!("Call VertexP3ui with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP3uiv(type_: types::GLenum, value: *const types::GLuint){
            println!("Call VertexP3uiv with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP4ui(type_: types::GLenum, value: types::GLuint){
            println!("Call VertexP4ui with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glVertexP4uiv(type_: types::GLenum, value: *const types::GLuint){
            println!("Call VertexP4uiv with type_: {:?}, value: {:?}", type_, value);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glViewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei){
            println!("Call Viewport with x: {}, y: {}, width: {}, height: {}", x, y, width, height);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glViewportArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLfloat){
            println!("Call ViewportArrayv with first: {:?}, count: {:?}, v: {:?}", first, count, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glViewportIndexedf(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, w: types::GLfloat, h: types::GLfloat){
            println!("Call ViewportIndexedf with index: {:?}, x: {:?}, y: {:?}, w: {:?}, h: {:?}", index, x, y, w, h);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glViewportIndexedfv(index: types::GLuint, v: *const types::GLfloat){
            println!("Call ViewportIndexedfv with index: {:?}, v: {:?}", index, v);
        }
    

        
        #[allow(non_camel_case_types, non_snake_case, unused_variables,dead_code)]
        pub unsafe extern "C" fn glWaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64){
            println!("Call WaitSync with sync: {:?}, flags: {:?}, timeout: {:?}", sync, flags, timeout);
        }
    
pub fn get_hook(identifier: &str) -> Result<*mut core::ffi::c_void, Box<dyn std::error::Error>> {
match identifier {

        "glActiveShaderProgram" => Ok(glActiveShaderProgram as *mut core::ffi::c_void),
    

        "glActiveTexture" => Ok(glActiveTexture as *mut core::ffi::c_void),
    

        "glAttachShader" => Ok(glAttachShader as *mut core::ffi::c_void),
    

        "glBeginConditionalRender" => Ok(glBeginConditionalRender as *mut core::ffi::c_void),
    

        "glBeginQuery" => Ok(glBeginQuery as *mut core::ffi::c_void),
    

        "glBeginQueryIndexed" => Ok(glBeginQueryIndexed as *mut core::ffi::c_void),
    

        "glBeginTransformFeedback" => Ok(glBeginTransformFeedback as *mut core::ffi::c_void),
    

        "glBindAttribLocation" => Ok(glBindAttribLocation as *mut core::ffi::c_void),
    

        "glBindBuffer" => Ok(glBindBuffer as *mut core::ffi::c_void),
    

        "glBindBufferBase" => Ok(glBindBufferBase as *mut core::ffi::c_void),
    

        "glBindBufferRange" => Ok(glBindBufferRange as *mut core::ffi::c_void),
    

        "glBindBuffersBase" => Ok(glBindBuffersBase as *mut core::ffi::c_void),
    

        "glBindBuffersRange" => Ok(glBindBuffersRange as *mut core::ffi::c_void),
    

        "glBindFragDataLocation" => Ok(glBindFragDataLocation as *mut core::ffi::c_void),
    

        "glBindFragDataLocationIndexed" => Ok(glBindFragDataLocationIndexed as *mut core::ffi::c_void),
    

        "glBindFramebuffer" => Ok(glBindFramebuffer as *mut core::ffi::c_void),
    

        "glBindImageTexture" => Ok(glBindImageTexture as *mut core::ffi::c_void),
    

        "glBindImageTextures" => Ok(glBindImageTextures as *mut core::ffi::c_void),
    

        "glBindProgramPipeline" => Ok(glBindProgramPipeline as *mut core::ffi::c_void),
    

        "glBindRenderbuffer" => Ok(glBindRenderbuffer as *mut core::ffi::c_void),
    

        "glBindSampler" => Ok(glBindSampler as *mut core::ffi::c_void),
    

        "glBindSamplers" => Ok(glBindSamplers as *mut core::ffi::c_void),
    

        "glBindTexture" => Ok(glBindTexture as *mut core::ffi::c_void),
    

        "glBindTextureUnit" => Ok(glBindTextureUnit as *mut core::ffi::c_void),
    

        "glBindTextures" => Ok(glBindTextures as *mut core::ffi::c_void),
    

        "glBindTransformFeedback" => Ok(glBindTransformFeedback as *mut core::ffi::c_void),
    

        "glBindVertexArray" => Ok(glBindVertexArray as *mut core::ffi::c_void),
    

        "glBindVertexBuffer" => Ok(glBindVertexBuffer as *mut core::ffi::c_void),
    

        "glBindVertexBuffers" => Ok(glBindVertexBuffers as *mut core::ffi::c_void),
    

        "glBlendColor" => Ok(glBlendColor as *mut core::ffi::c_void),
    

        "glBlendEquation" => Ok(glBlendEquation as *mut core::ffi::c_void),
    

        "glBlendEquationSeparate" => Ok(glBlendEquationSeparate as *mut core::ffi::c_void),
    

        "glBlendEquationSeparatei" => Ok(glBlendEquationSeparatei as *mut core::ffi::c_void),
    

        "glBlendEquationi" => Ok(glBlendEquationi as *mut core::ffi::c_void),
    

        "glBlendFunc" => Ok(glBlendFunc as *mut core::ffi::c_void),
    

        "glBlendFuncSeparate" => Ok(glBlendFuncSeparate as *mut core::ffi::c_void),
    

        "glBlendFuncSeparatei" => Ok(glBlendFuncSeparatei as *mut core::ffi::c_void),
    

        "glBlendFunci" => Ok(glBlendFunci as *mut core::ffi::c_void),
    

        "glBlitFramebuffer" => Ok(glBlitFramebuffer as *mut core::ffi::c_void),
    

        "glBlitNamedFramebuffer" => Ok(glBlitNamedFramebuffer as *mut core::ffi::c_void),
    

        "glBufferData" => Ok(glBufferData as *mut core::ffi::c_void),
    

        "glBufferStorage" => Ok(glBufferStorage as *mut core::ffi::c_void),
    

        "glBufferSubData" => Ok(glBufferSubData as *mut core::ffi::c_void),
    

        "glCheckFramebufferStatus" => Ok(glCheckFramebufferStatus as *mut core::ffi::c_void),
    

        "glCheckNamedFramebufferStatus" => Ok(glCheckNamedFramebufferStatus as *mut core::ffi::c_void),
    

        "glClampColor" => Ok(glClampColor as *mut core::ffi::c_void),
    

        "glClear" => Ok(glClear as *mut core::ffi::c_void),
    

        "glClearBufferData" => Ok(glClearBufferData as *mut core::ffi::c_void),
    

        "glClearBufferSubData" => Ok(glClearBufferSubData as *mut core::ffi::c_void),
    

        "glClearBufferfi" => Ok(glClearBufferfi as *mut core::ffi::c_void),
    

        "glClearBufferfv" => Ok(glClearBufferfv as *mut core::ffi::c_void),
    

        "glClearBufferiv" => Ok(glClearBufferiv as *mut core::ffi::c_void),
    

        "glClearBufferuiv" => Ok(glClearBufferuiv as *mut core::ffi::c_void),
    

        "glClearColor" => Ok(glClearColor as *mut core::ffi::c_void),
    

        "glClearDepth" => Ok(glClearDepth as *mut core::ffi::c_void),
    

        "glClearDepthf" => Ok(glClearDepthf as *mut core::ffi::c_void),
    

        "glClearNamedBufferData" => Ok(glClearNamedBufferData as *mut core::ffi::c_void),
    

        "glClearNamedBufferSubData" => Ok(glClearNamedBufferSubData as *mut core::ffi::c_void),
    

        "glClearNamedFramebufferfi" => Ok(glClearNamedFramebufferfi as *mut core::ffi::c_void),
    

        "glClearNamedFramebufferfv" => Ok(glClearNamedFramebufferfv as *mut core::ffi::c_void),
    

        "glClearNamedFramebufferiv" => Ok(glClearNamedFramebufferiv as *mut core::ffi::c_void),
    

        "glClearNamedFramebufferuiv" => Ok(glClearNamedFramebufferuiv as *mut core::ffi::c_void),
    

        "glClearStencil" => Ok(glClearStencil as *mut core::ffi::c_void),
    

        "glClearTexImage" => Ok(glClearTexImage as *mut core::ffi::c_void),
    

        "glClearTexSubImage" => Ok(glClearTexSubImage as *mut core::ffi::c_void),
    

        "glClientWaitSync" => Ok(glClientWaitSync as *mut core::ffi::c_void),
    

        "glClipControl" => Ok(glClipControl as *mut core::ffi::c_void),
    

        "glColorMask" => Ok(glColorMask as *mut core::ffi::c_void),
    

        "glColorMaski" => Ok(glColorMaski as *mut core::ffi::c_void),
    

        "glColorP3ui" => Ok(glColorP3ui as *mut core::ffi::c_void),
    

        "glColorP3uiv" => Ok(glColorP3uiv as *mut core::ffi::c_void),
    

        "glColorP4ui" => Ok(glColorP4ui as *mut core::ffi::c_void),
    

        "glColorP4uiv" => Ok(glColorP4uiv as *mut core::ffi::c_void),
    

        "glCompileShader" => Ok(glCompileShader as *mut core::ffi::c_void),
    

        "glCompressedTexImage1D" => Ok(glCompressedTexImage1D as *mut core::ffi::c_void),
    

        "glCompressedTexImage2D" => Ok(glCompressedTexImage2D as *mut core::ffi::c_void),
    

        "glCompressedTexImage3D" => Ok(glCompressedTexImage3D as *mut core::ffi::c_void),
    

        "glCompressedTexSubImage1D" => Ok(glCompressedTexSubImage1D as *mut core::ffi::c_void),
    

        "glCompressedTexSubImage2D" => Ok(glCompressedTexSubImage2D as *mut core::ffi::c_void),
    

        "glCompressedTexSubImage3D" => Ok(glCompressedTexSubImage3D as *mut core::ffi::c_void),
    

        "glCompressedTextureSubImage1D" => Ok(glCompressedTextureSubImage1D as *mut core::ffi::c_void),
    

        "glCompressedTextureSubImage2D" => Ok(glCompressedTextureSubImage2D as *mut core::ffi::c_void),
    

        "glCompressedTextureSubImage3D" => Ok(glCompressedTextureSubImage3D as *mut core::ffi::c_void),
    

        "glCopyBufferSubData" => Ok(glCopyBufferSubData as *mut core::ffi::c_void),
    

        "glCopyImageSubData" => Ok(glCopyImageSubData as *mut core::ffi::c_void),
    

        "glCopyNamedBufferSubData" => Ok(glCopyNamedBufferSubData as *mut core::ffi::c_void),
    

        "glCopyTexImage1D" => Ok(glCopyTexImage1D as *mut core::ffi::c_void),
    

        "glCopyTexImage2D" => Ok(glCopyTexImage2D as *mut core::ffi::c_void),
    

        "glCopyTexSubImage1D" => Ok(glCopyTexSubImage1D as *mut core::ffi::c_void),
    

        "glCopyTexSubImage2D" => Ok(glCopyTexSubImage2D as *mut core::ffi::c_void),
    

        "glCopyTexSubImage3D" => Ok(glCopyTexSubImage3D as *mut core::ffi::c_void),
    

        "glCopyTextureSubImage1D" => Ok(glCopyTextureSubImage1D as *mut core::ffi::c_void),
    

        "glCopyTextureSubImage2D" => Ok(glCopyTextureSubImage2D as *mut core::ffi::c_void),
    

        "glCopyTextureSubImage3D" => Ok(glCopyTextureSubImage3D as *mut core::ffi::c_void),
    

        "glCreateBuffers" => Ok(glCreateBuffers as *mut core::ffi::c_void),
    

        "glCreateFramebuffers" => Ok(glCreateFramebuffers as *mut core::ffi::c_void),
    

        "glCreateProgram" => Ok(glCreateProgram as *mut core::ffi::c_void),
    

        "glCreateProgramPipelines" => Ok(glCreateProgramPipelines as *mut core::ffi::c_void),
    

        "glCreateQueries" => Ok(glCreateQueries as *mut core::ffi::c_void),
    

        "glCreateRenderbuffers" => Ok(glCreateRenderbuffers as *mut core::ffi::c_void),
    

        "glCreateSamplers" => Ok(glCreateSamplers as *mut core::ffi::c_void),
    

        "glCreateShader" => Ok(glCreateShader as *mut core::ffi::c_void),
    

        "glCreateShaderProgramv" => Ok(glCreateShaderProgramv as *mut core::ffi::c_void),
    

        "glCreateTextures" => Ok(glCreateTextures as *mut core::ffi::c_void),
    

        "glCreateTransformFeedbacks" => Ok(glCreateTransformFeedbacks as *mut core::ffi::c_void),
    

        "glCreateVertexArrays" => Ok(glCreateVertexArrays as *mut core::ffi::c_void),
    

        "glCullFace" => Ok(glCullFace as *mut core::ffi::c_void),
    

        "glDebugMessageCallback" => Ok(glDebugMessageCallback as *mut core::ffi::c_void),
    

        "glDebugMessageControl" => Ok(glDebugMessageControl as *mut core::ffi::c_void),
    

        "glDebugMessageInsert" => Ok(glDebugMessageInsert as *mut core::ffi::c_void),
    

        "glDeleteBuffers" => Ok(glDeleteBuffers as *mut core::ffi::c_void),
    

        "glDeleteFramebuffers" => Ok(glDeleteFramebuffers as *mut core::ffi::c_void),
    

        "glDeleteProgram" => Ok(glDeleteProgram as *mut core::ffi::c_void),
    

        "glDeleteProgramPipelines" => Ok(glDeleteProgramPipelines as *mut core::ffi::c_void),
    

        "glDeleteQueries" => Ok(glDeleteQueries as *mut core::ffi::c_void),
    

        "glDeleteRenderbuffers" => Ok(glDeleteRenderbuffers as *mut core::ffi::c_void),
    

        "glDeleteSamplers" => Ok(glDeleteSamplers as *mut core::ffi::c_void),
    

        "glDeleteShader" => Ok(glDeleteShader as *mut core::ffi::c_void),
    

        "glDeleteSync" => Ok(glDeleteSync as *mut core::ffi::c_void),
    

        "glDeleteTextures" => Ok(glDeleteTextures as *mut core::ffi::c_void),
    

        "glDeleteTransformFeedbacks" => Ok(glDeleteTransformFeedbacks as *mut core::ffi::c_void),
    

        "glDeleteVertexArrays" => Ok(glDeleteVertexArrays as *mut core::ffi::c_void),
    

        "glDepthFunc" => Ok(glDepthFunc as *mut core::ffi::c_void),
    

        "glDepthMask" => Ok(glDepthMask as *mut core::ffi::c_void),
    

        "glDepthRange" => Ok(glDepthRange as *mut core::ffi::c_void),
    

        "glDepthRangeArrayv" => Ok(glDepthRangeArrayv as *mut core::ffi::c_void),
    

        "glDepthRangeIndexed" => Ok(glDepthRangeIndexed as *mut core::ffi::c_void),
    

        "glDepthRangef" => Ok(glDepthRangef as *mut core::ffi::c_void),
    

        "glDetachShader" => Ok(glDetachShader as *mut core::ffi::c_void),
    

        "glDisable" => Ok(glDisable as *mut core::ffi::c_void),
    

        "glDisableVertexArrayAttrib" => Ok(glDisableVertexArrayAttrib as *mut core::ffi::c_void),
    

        "glDisableVertexAttribArray" => Ok(glDisableVertexAttribArray as *mut core::ffi::c_void),
    

        "glDisablei" => Ok(glDisablei as *mut core::ffi::c_void),
    

        "glDispatchCompute" => Ok(glDispatchCompute as *mut core::ffi::c_void),
    

        "glDispatchComputeIndirect" => Ok(glDispatchComputeIndirect as *mut core::ffi::c_void),
    

        "glDrawArrays" => Ok(glDrawArrays as *mut core::ffi::c_void),
    

        "glDrawArraysIndirect" => Ok(glDrawArraysIndirect as *mut core::ffi::c_void),
    

        "glDrawArraysInstanced" => Ok(glDrawArraysInstanced as *mut core::ffi::c_void),
    

        "glDrawArraysInstancedBaseInstance" => Ok(glDrawArraysInstancedBaseInstance as *mut core::ffi::c_void),
    

        "glDrawBuffer" => Ok(glDrawBuffer as *mut core::ffi::c_void),
    

        "glDrawBuffers" => Ok(glDrawBuffers as *mut core::ffi::c_void),
    

        "glDrawElements" => Ok(glDrawElements as *mut core::ffi::c_void),
    

        "glDrawElementsBaseVertex" => Ok(glDrawElementsBaseVertex as *mut core::ffi::c_void),
    

        "glDrawElementsIndirect" => Ok(glDrawElementsIndirect as *mut core::ffi::c_void),
    

        "glDrawElementsInstanced" => Ok(glDrawElementsInstanced as *mut core::ffi::c_void),
    

        "glDrawElementsInstancedBaseInstance" => Ok(glDrawElementsInstancedBaseInstance as *mut core::ffi::c_void),
    

        "glDrawElementsInstancedBaseVertex" => Ok(glDrawElementsInstancedBaseVertex as *mut core::ffi::c_void),
    

        "glDrawElementsInstancedBaseVertexBaseInstance" => Ok(glDrawElementsInstancedBaseVertexBaseInstance as *mut core::ffi::c_void),
    

        "glDrawRangeElements" => Ok(glDrawRangeElements as *mut core::ffi::c_void),
    

        "glDrawRangeElementsBaseVertex" => Ok(glDrawRangeElementsBaseVertex as *mut core::ffi::c_void),
    

        "glDrawTransformFeedback" => Ok(glDrawTransformFeedback as *mut core::ffi::c_void),
    

        "glDrawTransformFeedbackInstanced" => Ok(glDrawTransformFeedbackInstanced as *mut core::ffi::c_void),
    

        "glDrawTransformFeedbackStream" => Ok(glDrawTransformFeedbackStream as *mut core::ffi::c_void),
    

        "glDrawTransformFeedbackStreamInstanced" => Ok(glDrawTransformFeedbackStreamInstanced as *mut core::ffi::c_void),
    

        "glEnable" => Ok(glEnable as *mut core::ffi::c_void),
    

        "glEnableVertexArrayAttrib" => Ok(glEnableVertexArrayAttrib as *mut core::ffi::c_void),
    

        "glEnableVertexAttribArray" => Ok(glEnableVertexAttribArray as *mut core::ffi::c_void),
    

        "glEnablei" => Ok(glEnablei as *mut core::ffi::c_void),
    

        "glEndConditionalRender" => Ok(glEndConditionalRender as *mut core::ffi::c_void),
    

        "glEndQuery" => Ok(glEndQuery as *mut core::ffi::c_void),
    

        "glEndQueryIndexed" => Ok(glEndQueryIndexed as *mut core::ffi::c_void),
    

        "glEndTransformFeedback" => Ok(glEndTransformFeedback as *mut core::ffi::c_void),
    

        "glFenceSync" => Ok(glFenceSync as *mut core::ffi::c_void),
    

        "glFinish" => Ok(glFinish as *mut core::ffi::c_void),
    

        "glFlush" => Ok(glFlush as *mut core::ffi::c_void),
    

        "glFlushMappedBufferRange" => Ok(glFlushMappedBufferRange as *mut core::ffi::c_void),
    

        "glFlushMappedNamedBufferRange" => Ok(glFlushMappedNamedBufferRange as *mut core::ffi::c_void),
    

        "glFramebufferParameteri" => Ok(glFramebufferParameteri as *mut core::ffi::c_void),
    

        "glFramebufferRenderbuffer" => Ok(glFramebufferRenderbuffer as *mut core::ffi::c_void),
    

        "glFramebufferTexture" => Ok(glFramebufferTexture as *mut core::ffi::c_void),
    

        "glFramebufferTexture1D" => Ok(glFramebufferTexture1D as *mut core::ffi::c_void),
    

        "glFramebufferTexture2D" => Ok(glFramebufferTexture2D as *mut core::ffi::c_void),
    

        "glFramebufferTexture3D" => Ok(glFramebufferTexture3D as *mut core::ffi::c_void),
    

        "glFramebufferTextureLayer" => Ok(glFramebufferTextureLayer as *mut core::ffi::c_void),
    

        "glFrontFace" => Ok(glFrontFace as *mut core::ffi::c_void),
    

        "glGenBuffers" => Ok(glGenBuffers as *mut core::ffi::c_void),
    

        "glGenFramebuffers" => Ok(glGenFramebuffers as *mut core::ffi::c_void),
    

        "glGenProgramPipelines" => Ok(glGenProgramPipelines as *mut core::ffi::c_void),
    

        "glGenQueries" => Ok(glGenQueries as *mut core::ffi::c_void),
    

        "glGenRenderbuffers" => Ok(glGenRenderbuffers as *mut core::ffi::c_void),
    

        "glGenSamplers" => Ok(glGenSamplers as *mut core::ffi::c_void),
    

        "glGenTextures" => Ok(glGenTextures as *mut core::ffi::c_void),
    

        "glGenTransformFeedbacks" => Ok(glGenTransformFeedbacks as *mut core::ffi::c_void),
    

        "glGenVertexArrays" => Ok(glGenVertexArrays as *mut core::ffi::c_void),
    

        "glGenerateMipmap" => Ok(glGenerateMipmap as *mut core::ffi::c_void),
    

        "glGenerateTextureMipmap" => Ok(glGenerateTextureMipmap as *mut core::ffi::c_void),
    

        "glGetActiveAtomicCounterBufferiv" => Ok(glGetActiveAtomicCounterBufferiv as *mut core::ffi::c_void),
    

        "glGetActiveAttrib" => Ok(glGetActiveAttrib as *mut core::ffi::c_void),
    

        "glGetActiveSubroutineName" => Ok(glGetActiveSubroutineName as *mut core::ffi::c_void),
    

        "glGetActiveSubroutineUniformName" => Ok(glGetActiveSubroutineUniformName as *mut core::ffi::c_void),
    

        "glGetActiveSubroutineUniformiv" => Ok(glGetActiveSubroutineUniformiv as *mut core::ffi::c_void),
    

        "glGetActiveUniform" => Ok(glGetActiveUniform as *mut core::ffi::c_void),
    

        "glGetActiveUniformBlockName" => Ok(glGetActiveUniformBlockName as *mut core::ffi::c_void),
    

        "glGetActiveUniformBlockiv" => Ok(glGetActiveUniformBlockiv as *mut core::ffi::c_void),
    

        "glGetActiveUniformName" => Ok(glGetActiveUniformName as *mut core::ffi::c_void),
    

        "glGetActiveUniformsiv" => Ok(glGetActiveUniformsiv as *mut core::ffi::c_void),
    

        "glGetAttachedShaders" => Ok(glGetAttachedShaders as *mut core::ffi::c_void),
    

        "glGetAttribLocation" => Ok(glGetAttribLocation as *mut core::ffi::c_void),
    

        "glGetBooleani_v" => Ok(glGetBooleani_v as *mut core::ffi::c_void),
    

        "glGetBooleanv" => Ok(glGetBooleanv as *mut core::ffi::c_void),
    

        "glGetBufferParameteri64v" => Ok(glGetBufferParameteri64v as *mut core::ffi::c_void),
    

        "glGetBufferParameteriv" => Ok(glGetBufferParameteriv as *mut core::ffi::c_void),
    

        "glGetBufferPointerv" => Ok(glGetBufferPointerv as *mut core::ffi::c_void),
    

        "glGetBufferSubData" => Ok(glGetBufferSubData as *mut core::ffi::c_void),
    

        "glGetCompressedTexImage" => Ok(glGetCompressedTexImage as *mut core::ffi::c_void),
    

        "glGetCompressedTextureImage" => Ok(glGetCompressedTextureImage as *mut core::ffi::c_void),
    

        "glGetCompressedTextureSubImage" => Ok(glGetCompressedTextureSubImage as *mut core::ffi::c_void),
    

        "glGetDebugMessageLog" => Ok(glGetDebugMessageLog as *mut core::ffi::c_void),
    

        "glGetDoublei_v" => Ok(glGetDoublei_v as *mut core::ffi::c_void),
    

        "glGetDoublev" => Ok(glGetDoublev as *mut core::ffi::c_void),
    

        "glGetError" => Ok(glGetError as *mut core::ffi::c_void),
    

        "glGetFloati_v" => Ok(glGetFloati_v as *mut core::ffi::c_void),
    

        "glGetFloatv" => Ok(glGetFloatv as *mut core::ffi::c_void),
    

        "glGetFragDataIndex" => Ok(glGetFragDataIndex as *mut core::ffi::c_void),
    

        "glGetFragDataLocation" => Ok(glGetFragDataLocation as *mut core::ffi::c_void),
    

        "glGetFramebufferAttachmentParameteriv" => Ok(glGetFramebufferAttachmentParameteriv as *mut core::ffi::c_void),
    

        "glGetFramebufferParameteriv" => Ok(glGetFramebufferParameteriv as *mut core::ffi::c_void),
    

        "glGetGraphicsResetStatus" => Ok(glGetGraphicsResetStatus as *mut core::ffi::c_void),
    

        "glGetInteger64i_v" => Ok(glGetInteger64i_v as *mut core::ffi::c_void),
    

        "glGetInteger64v" => Ok(glGetInteger64v as *mut core::ffi::c_void),
    

        "glGetIntegeri_v" => Ok(glGetIntegeri_v as *mut core::ffi::c_void),
    

        "glGetIntegerv" => Ok(glGetIntegerv as *mut core::ffi::c_void),
    

        "glGetInternalformati64v" => Ok(glGetInternalformati64v as *mut core::ffi::c_void),
    

        "glGetInternalformativ" => Ok(glGetInternalformativ as *mut core::ffi::c_void),
    

        "glGetMultisamplefv" => Ok(glGetMultisamplefv as *mut core::ffi::c_void),
    

        "glGetNamedBufferParameteri64v" => Ok(glGetNamedBufferParameteri64v as *mut core::ffi::c_void),
    

        "glGetNamedBufferParameteriv" => Ok(glGetNamedBufferParameteriv as *mut core::ffi::c_void),
    

        "glGetNamedBufferPointerv" => Ok(glGetNamedBufferPointerv as *mut core::ffi::c_void),
    

        "glGetNamedBufferSubData" => Ok(glGetNamedBufferSubData as *mut core::ffi::c_void),
    

        "glGetNamedFramebufferAttachmentParameteriv" => Ok(glGetNamedFramebufferAttachmentParameteriv as *mut core::ffi::c_void),
    

        "glGetNamedFramebufferParameteriv" => Ok(glGetNamedFramebufferParameteriv as *mut core::ffi::c_void),
    

        "glGetNamedRenderbufferParameteriv" => Ok(glGetNamedRenderbufferParameteriv as *mut core::ffi::c_void),
    

        "glGetObjectLabel" => Ok(glGetObjectLabel as *mut core::ffi::c_void),
    

        "glGetObjectPtrLabel" => Ok(glGetObjectPtrLabel as *mut core::ffi::c_void),
    

        "glGetPointerv" => Ok(glGetPointerv as *mut core::ffi::c_void),
    

        "glGetProgramBinary" => Ok(glGetProgramBinary as *mut core::ffi::c_void),
    

        "glGetProgramInfoLog" => Ok(glGetProgramInfoLog as *mut core::ffi::c_void),
    

        "glGetProgramInterfaceiv" => Ok(glGetProgramInterfaceiv as *mut core::ffi::c_void),
    

        "glGetProgramPipelineInfoLog" => Ok(glGetProgramPipelineInfoLog as *mut core::ffi::c_void),
    

        "glGetProgramPipelineiv" => Ok(glGetProgramPipelineiv as *mut core::ffi::c_void),
    

        "glGetProgramResourceIndex" => Ok(glGetProgramResourceIndex as *mut core::ffi::c_void),
    

        "glGetProgramResourceLocation" => Ok(glGetProgramResourceLocation as *mut core::ffi::c_void),
    

        "glGetProgramResourceLocationIndex" => Ok(glGetProgramResourceLocationIndex as *mut core::ffi::c_void),
    

        "glGetProgramResourceName" => Ok(glGetProgramResourceName as *mut core::ffi::c_void),
    

        "glGetProgramResourceiv" => Ok(glGetProgramResourceiv as *mut core::ffi::c_void),
    

        "glGetProgramStageiv" => Ok(glGetProgramStageiv as *mut core::ffi::c_void),
    

        "glGetProgramiv" => Ok(glGetProgramiv as *mut core::ffi::c_void),
    

        "glGetQueryBufferObjecti64v" => Ok(glGetQueryBufferObjecti64v as *mut core::ffi::c_void),
    

        "glGetQueryBufferObjectiv" => Ok(glGetQueryBufferObjectiv as *mut core::ffi::c_void),
    

        "glGetQueryBufferObjectui64v" => Ok(glGetQueryBufferObjectui64v as *mut core::ffi::c_void),
    

        "glGetQueryBufferObjectuiv" => Ok(glGetQueryBufferObjectuiv as *mut core::ffi::c_void),
    

        "glGetQueryIndexediv" => Ok(glGetQueryIndexediv as *mut core::ffi::c_void),
    

        "glGetQueryObjecti64v" => Ok(glGetQueryObjecti64v as *mut core::ffi::c_void),
    

        "glGetQueryObjectiv" => Ok(glGetQueryObjectiv as *mut core::ffi::c_void),
    

        "glGetQueryObjectui64v" => Ok(glGetQueryObjectui64v as *mut core::ffi::c_void),
    

        "glGetQueryObjectuiv" => Ok(glGetQueryObjectuiv as *mut core::ffi::c_void),
    

        "glGetQueryiv" => Ok(glGetQueryiv as *mut core::ffi::c_void),
    

        "glGetRenderbufferParameteriv" => Ok(glGetRenderbufferParameteriv as *mut core::ffi::c_void),
    

        "glGetSamplerParameterIiv" => Ok(glGetSamplerParameterIiv as *mut core::ffi::c_void),
    

        "glGetSamplerParameterIuiv" => Ok(glGetSamplerParameterIuiv as *mut core::ffi::c_void),
    

        "glGetSamplerParameterfv" => Ok(glGetSamplerParameterfv as *mut core::ffi::c_void),
    

        "glGetSamplerParameteriv" => Ok(glGetSamplerParameteriv as *mut core::ffi::c_void),
    

        "glGetShaderInfoLog" => Ok(glGetShaderInfoLog as *mut core::ffi::c_void),
    

        "glGetShaderPrecisionFormat" => Ok(glGetShaderPrecisionFormat as *mut core::ffi::c_void),
    

        "glGetShaderSource" => Ok(glGetShaderSource as *mut core::ffi::c_void),
    

        "glGetShaderiv" => Ok(glGetShaderiv as *mut core::ffi::c_void),
    

        "glGetString" => Ok(glGetString as *mut core::ffi::c_void),
    

        "glGetStringi" => Ok(glGetStringi as *mut core::ffi::c_void),
    

        "glGetSubroutineIndex" => Ok(glGetSubroutineIndex as *mut core::ffi::c_void),
    

        "glGetSubroutineUniformLocation" => Ok(glGetSubroutineUniformLocation as *mut core::ffi::c_void),
    

        "glGetSynciv" => Ok(glGetSynciv as *mut core::ffi::c_void),
    

        "glGetTexImage" => Ok(glGetTexImage as *mut core::ffi::c_void),
    

        "glGetTexLevelParameterfv" => Ok(glGetTexLevelParameterfv as *mut core::ffi::c_void),
    

        "glGetTexLevelParameteriv" => Ok(glGetTexLevelParameteriv as *mut core::ffi::c_void),
    

        "glGetTexParameterIiv" => Ok(glGetTexParameterIiv as *mut core::ffi::c_void),
    

        "glGetTexParameterIuiv" => Ok(glGetTexParameterIuiv as *mut core::ffi::c_void),
    

        "glGetTexParameterfv" => Ok(glGetTexParameterfv as *mut core::ffi::c_void),
    

        "glGetTexParameteriv" => Ok(glGetTexParameteriv as *mut core::ffi::c_void),
    

        "glGetTextureImage" => Ok(glGetTextureImage as *mut core::ffi::c_void),
    

        "glGetTextureLevelParameterfv" => Ok(glGetTextureLevelParameterfv as *mut core::ffi::c_void),
    

        "glGetTextureLevelParameteriv" => Ok(glGetTextureLevelParameteriv as *mut core::ffi::c_void),
    

        "glGetTextureParameterIiv" => Ok(glGetTextureParameterIiv as *mut core::ffi::c_void),
    

        "glGetTextureParameterIuiv" => Ok(glGetTextureParameterIuiv as *mut core::ffi::c_void),
    

        "glGetTextureParameterfv" => Ok(glGetTextureParameterfv as *mut core::ffi::c_void),
    

        "glGetTextureParameteriv" => Ok(glGetTextureParameteriv as *mut core::ffi::c_void),
    

        "glGetTextureSubImage" => Ok(glGetTextureSubImage as *mut core::ffi::c_void),
    

        "glGetTransformFeedbackVarying" => Ok(glGetTransformFeedbackVarying as *mut core::ffi::c_void),
    

        "glGetTransformFeedbacki64_v" => Ok(glGetTransformFeedbacki64_v as *mut core::ffi::c_void),
    

        "glGetTransformFeedbacki_v" => Ok(glGetTransformFeedbacki_v as *mut core::ffi::c_void),
    

        "glGetTransformFeedbackiv" => Ok(glGetTransformFeedbackiv as *mut core::ffi::c_void),
    

        "glGetUniformBlockIndex" => Ok(glGetUniformBlockIndex as *mut core::ffi::c_void),
    

        "glGetUniformIndices" => Ok(glGetUniformIndices as *mut core::ffi::c_void),
    

        "glGetUniformLocation" => Ok(glGetUniformLocation as *mut core::ffi::c_void),
    

        "glGetUniformSubroutineuiv" => Ok(glGetUniformSubroutineuiv as *mut core::ffi::c_void),
    

        "glGetUniformdv" => Ok(glGetUniformdv as *mut core::ffi::c_void),
    

        "glGetUniformfv" => Ok(glGetUniformfv as *mut core::ffi::c_void),
    

        "glGetUniformiv" => Ok(glGetUniformiv as *mut core::ffi::c_void),
    

        "glGetUniformuiv" => Ok(glGetUniformuiv as *mut core::ffi::c_void),
    

        "glGetVertexArrayIndexed64iv" => Ok(glGetVertexArrayIndexed64iv as *mut core::ffi::c_void),
    

        "glGetVertexArrayIndexediv" => Ok(glGetVertexArrayIndexediv as *mut core::ffi::c_void),
    

        "glGetVertexArrayiv" => Ok(glGetVertexArrayiv as *mut core::ffi::c_void),
    

        "glGetVertexAttribIiv" => Ok(glGetVertexAttribIiv as *mut core::ffi::c_void),
    

        "glGetVertexAttribIuiv" => Ok(glGetVertexAttribIuiv as *mut core::ffi::c_void),
    

        "glGetVertexAttribLdv" => Ok(glGetVertexAttribLdv as *mut core::ffi::c_void),
    

        "glGetVertexAttribPointerv" => Ok(glGetVertexAttribPointerv as *mut core::ffi::c_void),
    

        "glGetVertexAttribdv" => Ok(glGetVertexAttribdv as *mut core::ffi::c_void),
    

        "glGetVertexAttribfv" => Ok(glGetVertexAttribfv as *mut core::ffi::c_void),
    

        "glGetVertexAttribiv" => Ok(glGetVertexAttribiv as *mut core::ffi::c_void),
    

        "glGetnColorTable" => Ok(glGetnColorTable as *mut core::ffi::c_void),
    

        "glGetnCompressedTexImage" => Ok(glGetnCompressedTexImage as *mut core::ffi::c_void),
    

        "glGetnConvolutionFilter" => Ok(glGetnConvolutionFilter as *mut core::ffi::c_void),
    

        "glGetnHistogram" => Ok(glGetnHistogram as *mut core::ffi::c_void),
    

        "glGetnMapdv" => Ok(glGetnMapdv as *mut core::ffi::c_void),
    

        "glGetnMapfv" => Ok(glGetnMapfv as *mut core::ffi::c_void),
    

        "glGetnMapiv" => Ok(glGetnMapiv as *mut core::ffi::c_void),
    

        "glGetnMinmax" => Ok(glGetnMinmax as *mut core::ffi::c_void),
    

        "glGetnPixelMapfv" => Ok(glGetnPixelMapfv as *mut core::ffi::c_void),
    

        "glGetnPixelMapuiv" => Ok(glGetnPixelMapuiv as *mut core::ffi::c_void),
    

        "glGetnPixelMapusv" => Ok(glGetnPixelMapusv as *mut core::ffi::c_void),
    

        "glGetnPolygonStipple" => Ok(glGetnPolygonStipple as *mut core::ffi::c_void),
    

        "glGetnSeparableFilter" => Ok(glGetnSeparableFilter as *mut core::ffi::c_void),
    

        "glGetnTexImage" => Ok(glGetnTexImage as *mut core::ffi::c_void),
    

        "glGetnUniformdv" => Ok(glGetnUniformdv as *mut core::ffi::c_void),
    

        "glGetnUniformfv" => Ok(glGetnUniformfv as *mut core::ffi::c_void),
    

        "glGetnUniformiv" => Ok(glGetnUniformiv as *mut core::ffi::c_void),
    

        "glGetnUniformuiv" => Ok(glGetnUniformuiv as *mut core::ffi::c_void),
    

        "glHint" => Ok(glHint as *mut core::ffi::c_void),
    

        "glInvalidateBufferData" => Ok(glInvalidateBufferData as *mut core::ffi::c_void),
    

        "glInvalidateBufferSubData" => Ok(glInvalidateBufferSubData as *mut core::ffi::c_void),
    

        "glInvalidateFramebuffer" => Ok(glInvalidateFramebuffer as *mut core::ffi::c_void),
    

        "glInvalidateNamedFramebufferData" => Ok(glInvalidateNamedFramebufferData as *mut core::ffi::c_void),
    

        "glInvalidateNamedFramebufferSubData" => Ok(glInvalidateNamedFramebufferSubData as *mut core::ffi::c_void),
    

        "glInvalidateSubFramebuffer" => Ok(glInvalidateSubFramebuffer as *mut core::ffi::c_void),
    

        "glInvalidateTexImage" => Ok(glInvalidateTexImage as *mut core::ffi::c_void),
    

        "glInvalidateTexSubImage" => Ok(glInvalidateTexSubImage as *mut core::ffi::c_void),
    

        "glIsBuffer" => Ok(glIsBuffer as *mut core::ffi::c_void),
    

        "glIsEnabled" => Ok(glIsEnabled as *mut core::ffi::c_void),
    

        "glIsEnabledi" => Ok(glIsEnabledi as *mut core::ffi::c_void),
    

        "glIsFramebuffer" => Ok(glIsFramebuffer as *mut core::ffi::c_void),
    

        "glIsProgram" => Ok(glIsProgram as *mut core::ffi::c_void),
    

        "glIsProgramPipeline" => Ok(glIsProgramPipeline as *mut core::ffi::c_void),
    

        "glIsQuery" => Ok(glIsQuery as *mut core::ffi::c_void),
    

        "glIsRenderbuffer" => Ok(glIsRenderbuffer as *mut core::ffi::c_void),
    

        "glIsSampler" => Ok(glIsSampler as *mut core::ffi::c_void),
    

        "glIsShader" => Ok(glIsShader as *mut core::ffi::c_void),
    

        "glIsSync" => Ok(glIsSync as *mut core::ffi::c_void),
    

        "glIsTexture" => Ok(glIsTexture as *mut core::ffi::c_void),
    

        "glIsTransformFeedback" => Ok(glIsTransformFeedback as *mut core::ffi::c_void),
    

        "glIsVertexArray" => Ok(glIsVertexArray as *mut core::ffi::c_void),
    

        "glLineWidth" => Ok(glLineWidth as *mut core::ffi::c_void),
    

        "glLinkProgram" => Ok(glLinkProgram as *mut core::ffi::c_void),
    

        "glLogicOp" => Ok(glLogicOp as *mut core::ffi::c_void),
    

        "glMapBuffer" => Ok(glMapBuffer as *mut core::ffi::c_void),
    

        "glMapBufferRange" => Ok(glMapBufferRange as *mut core::ffi::c_void),
    

        "glMapNamedBuffer" => Ok(glMapNamedBuffer as *mut core::ffi::c_void),
    

        "glMapNamedBufferRange" => Ok(glMapNamedBufferRange as *mut core::ffi::c_void),
    

        "glMemoryBarrier" => Ok(glMemoryBarrier as *mut core::ffi::c_void),
    

        "glMemoryBarrierByRegion" => Ok(glMemoryBarrierByRegion as *mut core::ffi::c_void),
    

        "glMinSampleShading" => Ok(glMinSampleShading as *mut core::ffi::c_void),
    

        "glMultiDrawArrays" => Ok(glMultiDrawArrays as *mut core::ffi::c_void),
    

        "glMultiDrawArraysIndirect" => Ok(glMultiDrawArraysIndirect as *mut core::ffi::c_void),
    

        "glMultiDrawElements" => Ok(glMultiDrawElements as *mut core::ffi::c_void),
    

        "glMultiDrawElementsBaseVertex" => Ok(glMultiDrawElementsBaseVertex as *mut core::ffi::c_void),
    

        "glMultiDrawElementsIndirect" => Ok(glMultiDrawElementsIndirect as *mut core::ffi::c_void),
    

        "glMultiTexCoordP1ui" => Ok(glMultiTexCoordP1ui as *mut core::ffi::c_void),
    

        "glMultiTexCoordP1uiv" => Ok(glMultiTexCoordP1uiv as *mut core::ffi::c_void),
    

        "glMultiTexCoordP2ui" => Ok(glMultiTexCoordP2ui as *mut core::ffi::c_void),
    

        "glMultiTexCoordP2uiv" => Ok(glMultiTexCoordP2uiv as *mut core::ffi::c_void),
    

        "glMultiTexCoordP3ui" => Ok(glMultiTexCoordP3ui as *mut core::ffi::c_void),
    

        "glMultiTexCoordP3uiv" => Ok(glMultiTexCoordP3uiv as *mut core::ffi::c_void),
    

        "glMultiTexCoordP4ui" => Ok(glMultiTexCoordP4ui as *mut core::ffi::c_void),
    

        "glMultiTexCoordP4uiv" => Ok(glMultiTexCoordP4uiv as *mut core::ffi::c_void),
    

        "glNamedBufferData" => Ok(glNamedBufferData as *mut core::ffi::c_void),
    

        "glNamedBufferStorage" => Ok(glNamedBufferStorage as *mut core::ffi::c_void),
    

        "glNamedBufferSubData" => Ok(glNamedBufferSubData as *mut core::ffi::c_void),
    

        "glNamedFramebufferDrawBuffer" => Ok(glNamedFramebufferDrawBuffer as *mut core::ffi::c_void),
    

        "glNamedFramebufferDrawBuffers" => Ok(glNamedFramebufferDrawBuffers as *mut core::ffi::c_void),
    

        "glNamedFramebufferParameteri" => Ok(glNamedFramebufferParameteri as *mut core::ffi::c_void),
    

        "glNamedFramebufferReadBuffer" => Ok(glNamedFramebufferReadBuffer as *mut core::ffi::c_void),
    

        "glNamedFramebufferRenderbuffer" => Ok(glNamedFramebufferRenderbuffer as *mut core::ffi::c_void),
    

        "glNamedFramebufferTexture" => Ok(glNamedFramebufferTexture as *mut core::ffi::c_void),
    

        "glNamedFramebufferTextureLayer" => Ok(glNamedFramebufferTextureLayer as *mut core::ffi::c_void),
    

        "glNamedRenderbufferStorage" => Ok(glNamedRenderbufferStorage as *mut core::ffi::c_void),
    

        "glNamedRenderbufferStorageMultisample" => Ok(glNamedRenderbufferStorageMultisample as *mut core::ffi::c_void),
    

        "glNormalP3ui" => Ok(glNormalP3ui as *mut core::ffi::c_void),
    

        "glNormalP3uiv" => Ok(glNormalP3uiv as *mut core::ffi::c_void),
    

        "glObjectLabel" => Ok(glObjectLabel as *mut core::ffi::c_void),
    

        "glObjectPtrLabel" => Ok(glObjectPtrLabel as *mut core::ffi::c_void),
    

        "glPatchParameterfv" => Ok(glPatchParameterfv as *mut core::ffi::c_void),
    

        "glPatchParameteri" => Ok(glPatchParameteri as *mut core::ffi::c_void),
    

        "glPauseTransformFeedback" => Ok(glPauseTransformFeedback as *mut core::ffi::c_void),
    

        "glPixelStoref" => Ok(glPixelStoref as *mut core::ffi::c_void),
    

        "glPixelStorei" => Ok(glPixelStorei as *mut core::ffi::c_void),
    

        "glPointParameterf" => Ok(glPointParameterf as *mut core::ffi::c_void),
    

        "glPointParameterfv" => Ok(glPointParameterfv as *mut core::ffi::c_void),
    

        "glPointParameteri" => Ok(glPointParameteri as *mut core::ffi::c_void),
    

        "glPointParameteriv" => Ok(glPointParameteriv as *mut core::ffi::c_void),
    

        "glPointSize" => Ok(glPointSize as *mut core::ffi::c_void),
    

        "glPolygonMode" => Ok(glPolygonMode as *mut core::ffi::c_void),
    

        "glPolygonOffset" => Ok(glPolygonOffset as *mut core::ffi::c_void),
    

        "glPopDebugGroup" => Ok(glPopDebugGroup as *mut core::ffi::c_void),
    

        "glPrimitiveRestartIndex" => Ok(glPrimitiveRestartIndex as *mut core::ffi::c_void),
    

        "glProgramBinary" => Ok(glProgramBinary as *mut core::ffi::c_void),
    

        "glProgramParameteri" => Ok(glProgramParameteri as *mut core::ffi::c_void),
    

        "glProgramUniform1d" => Ok(glProgramUniform1d as *mut core::ffi::c_void),
    

        "glProgramUniform1dv" => Ok(glProgramUniform1dv as *mut core::ffi::c_void),
    

        "glProgramUniform1f" => Ok(glProgramUniform1f as *mut core::ffi::c_void),
    

        "glProgramUniform1fv" => Ok(glProgramUniform1fv as *mut core::ffi::c_void),
    

        "glProgramUniform1i" => Ok(glProgramUniform1i as *mut core::ffi::c_void),
    

        "glProgramUniform1iv" => Ok(glProgramUniform1iv as *mut core::ffi::c_void),
    

        "glProgramUniform1ui" => Ok(glProgramUniform1ui as *mut core::ffi::c_void),
    

        "glProgramUniform1uiv" => Ok(glProgramUniform1uiv as *mut core::ffi::c_void),
    

        "glProgramUniform2d" => Ok(glProgramUniform2d as *mut core::ffi::c_void),
    

        "glProgramUniform2dv" => Ok(glProgramUniform2dv as *mut core::ffi::c_void),
    

        "glProgramUniform2f" => Ok(glProgramUniform2f as *mut core::ffi::c_void),
    

        "glProgramUniform2fv" => Ok(glProgramUniform2fv as *mut core::ffi::c_void),
    

        "glProgramUniform2i" => Ok(glProgramUniform2i as *mut core::ffi::c_void),
    

        "glProgramUniform2iv" => Ok(glProgramUniform2iv as *mut core::ffi::c_void),
    

        "glProgramUniform2ui" => Ok(glProgramUniform2ui as *mut core::ffi::c_void),
    

        "glProgramUniform2uiv" => Ok(glProgramUniform2uiv as *mut core::ffi::c_void),
    

        "glProgramUniform3d" => Ok(glProgramUniform3d as *mut core::ffi::c_void),
    

        "glProgramUniform3dv" => Ok(glProgramUniform3dv as *mut core::ffi::c_void),
    

        "glProgramUniform3f" => Ok(glProgramUniform3f as *mut core::ffi::c_void),
    

        "glProgramUniform3fv" => Ok(glProgramUniform3fv as *mut core::ffi::c_void),
    

        "glProgramUniform3i" => Ok(glProgramUniform3i as *mut core::ffi::c_void),
    

        "glProgramUniform3iv" => Ok(glProgramUniform3iv as *mut core::ffi::c_void),
    

        "glProgramUniform3ui" => Ok(glProgramUniform3ui as *mut core::ffi::c_void),
    

        "glProgramUniform3uiv" => Ok(glProgramUniform3uiv as *mut core::ffi::c_void),
    

        "glProgramUniform4d" => Ok(glProgramUniform4d as *mut core::ffi::c_void),
    

        "glProgramUniform4dv" => Ok(glProgramUniform4dv as *mut core::ffi::c_void),
    

        "glProgramUniform4f" => Ok(glProgramUniform4f as *mut core::ffi::c_void),
    

        "glProgramUniform4fv" => Ok(glProgramUniform4fv as *mut core::ffi::c_void),
    

        "glProgramUniform4i" => Ok(glProgramUniform4i as *mut core::ffi::c_void),
    

        "glProgramUniform4iv" => Ok(glProgramUniform4iv as *mut core::ffi::c_void),
    

        "glProgramUniform4ui" => Ok(glProgramUniform4ui as *mut core::ffi::c_void),
    

        "glProgramUniform4uiv" => Ok(glProgramUniform4uiv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2dv" => Ok(glProgramUniformMatrix2dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2fv" => Ok(glProgramUniformMatrix2fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2x3dv" => Ok(glProgramUniformMatrix2x3dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2x3fv" => Ok(glProgramUniformMatrix2x3fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2x4dv" => Ok(glProgramUniformMatrix2x4dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix2x4fv" => Ok(glProgramUniformMatrix2x4fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3dv" => Ok(glProgramUniformMatrix3dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3fv" => Ok(glProgramUniformMatrix3fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3x2dv" => Ok(glProgramUniformMatrix3x2dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3x2fv" => Ok(glProgramUniformMatrix3x2fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3x4dv" => Ok(glProgramUniformMatrix3x4dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix3x4fv" => Ok(glProgramUniformMatrix3x4fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4dv" => Ok(glProgramUniformMatrix4dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4fv" => Ok(glProgramUniformMatrix4fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4x2dv" => Ok(glProgramUniformMatrix4x2dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4x2fv" => Ok(glProgramUniformMatrix4x2fv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4x3dv" => Ok(glProgramUniformMatrix4x3dv as *mut core::ffi::c_void),
    

        "glProgramUniformMatrix4x3fv" => Ok(glProgramUniformMatrix4x3fv as *mut core::ffi::c_void),
    

        "glProvokingVertex" => Ok(glProvokingVertex as *mut core::ffi::c_void),
    

        "glPushDebugGroup" => Ok(glPushDebugGroup as *mut core::ffi::c_void),
    

        "glQueryCounter" => Ok(glQueryCounter as *mut core::ffi::c_void),
    

        "glReadBuffer" => Ok(glReadBuffer as *mut core::ffi::c_void),
    

        "glReadPixels" => Ok(glReadPixels as *mut core::ffi::c_void),
    

        "glReadnPixels" => Ok(glReadnPixels as *mut core::ffi::c_void),
    

        "glReleaseShaderCompiler" => Ok(glReleaseShaderCompiler as *mut core::ffi::c_void),
    

        "glRenderbufferStorage" => Ok(glRenderbufferStorage as *mut core::ffi::c_void),
    

        "glRenderbufferStorageMultisample" => Ok(glRenderbufferStorageMultisample as *mut core::ffi::c_void),
    

        "glResumeTransformFeedback" => Ok(glResumeTransformFeedback as *mut core::ffi::c_void),
    

        "glSampleCoverage" => Ok(glSampleCoverage as *mut core::ffi::c_void),
    

        "glSampleMaski" => Ok(glSampleMaski as *mut core::ffi::c_void),
    

        "glSamplerParameterIiv" => Ok(glSamplerParameterIiv as *mut core::ffi::c_void),
    

        "glSamplerParameterIuiv" => Ok(glSamplerParameterIuiv as *mut core::ffi::c_void),
    

        "glSamplerParameterf" => Ok(glSamplerParameterf as *mut core::ffi::c_void),
    

        "glSamplerParameterfv" => Ok(glSamplerParameterfv as *mut core::ffi::c_void),
    

        "glSamplerParameteri" => Ok(glSamplerParameteri as *mut core::ffi::c_void),
    

        "glSamplerParameteriv" => Ok(glSamplerParameteriv as *mut core::ffi::c_void),
    

        "glScissor" => Ok(glScissor as *mut core::ffi::c_void),
    

        "glScissorArrayv" => Ok(glScissorArrayv as *mut core::ffi::c_void),
    

        "glScissorIndexed" => Ok(glScissorIndexed as *mut core::ffi::c_void),
    

        "glScissorIndexedv" => Ok(glScissorIndexedv as *mut core::ffi::c_void),
    

        "glSecondaryColorP3ui" => Ok(glSecondaryColorP3ui as *mut core::ffi::c_void),
    

        "glSecondaryColorP3uiv" => Ok(glSecondaryColorP3uiv as *mut core::ffi::c_void),
    

        "glShaderBinary" => Ok(glShaderBinary as *mut core::ffi::c_void),
    

        "glShaderSource" => Ok(glShaderSource as *mut core::ffi::c_void),
    

        "glShaderStorageBlockBinding" => Ok(glShaderStorageBlockBinding as *mut core::ffi::c_void),
    

        "glStencilFunc" => Ok(glStencilFunc as *mut core::ffi::c_void),
    

        "glStencilFuncSeparate" => Ok(glStencilFuncSeparate as *mut core::ffi::c_void),
    

        "glStencilMask" => Ok(glStencilMask as *mut core::ffi::c_void),
    

        "glStencilMaskSeparate" => Ok(glStencilMaskSeparate as *mut core::ffi::c_void),
    

        "glStencilOp" => Ok(glStencilOp as *mut core::ffi::c_void),
    

        "glStencilOpSeparate" => Ok(glStencilOpSeparate as *mut core::ffi::c_void),
    

        "glTexBuffer" => Ok(glTexBuffer as *mut core::ffi::c_void),
    

        "glTexBufferRange" => Ok(glTexBufferRange as *mut core::ffi::c_void),
    

        "glTexCoordP1ui" => Ok(glTexCoordP1ui as *mut core::ffi::c_void),
    

        "glTexCoordP1uiv" => Ok(glTexCoordP1uiv as *mut core::ffi::c_void),
    

        "glTexCoordP2ui" => Ok(glTexCoordP2ui as *mut core::ffi::c_void),
    

        "glTexCoordP2uiv" => Ok(glTexCoordP2uiv as *mut core::ffi::c_void),
    

        "glTexCoordP3ui" => Ok(glTexCoordP3ui as *mut core::ffi::c_void),
    

        "glTexCoordP3uiv" => Ok(glTexCoordP3uiv as *mut core::ffi::c_void),
    

        "glTexCoordP4ui" => Ok(glTexCoordP4ui as *mut core::ffi::c_void),
    

        "glTexCoordP4uiv" => Ok(glTexCoordP4uiv as *mut core::ffi::c_void),
    

        "glTexImage1D" => Ok(glTexImage1D as *mut core::ffi::c_void),
    

        "glTexImage2D" => Ok(glTexImage2D as *mut core::ffi::c_void),
    

        "glTexImage2DMultisample" => Ok(glTexImage2DMultisample as *mut core::ffi::c_void),
    

        "glTexImage3D" => Ok(glTexImage3D as *mut core::ffi::c_void),
    

        "glTexImage3DMultisample" => Ok(glTexImage3DMultisample as *mut core::ffi::c_void),
    

        "glTexParameterIiv" => Ok(glTexParameterIiv as *mut core::ffi::c_void),
    

        "glTexParameterIuiv" => Ok(glTexParameterIuiv as *mut core::ffi::c_void),
    

        "glTexParameterf" => Ok(glTexParameterf as *mut core::ffi::c_void),
    

        "glTexParameterfv" => Ok(glTexParameterfv as *mut core::ffi::c_void),
    

        "glTexParameteri" => Ok(glTexParameteri as *mut core::ffi::c_void),
    

        "glTexParameteriv" => Ok(glTexParameteriv as *mut core::ffi::c_void),
    

        "glTexStorage1D" => Ok(glTexStorage1D as *mut core::ffi::c_void),
    

        "glTexStorage2D" => Ok(glTexStorage2D as *mut core::ffi::c_void),
    

        "glTexStorage2DMultisample" => Ok(glTexStorage2DMultisample as *mut core::ffi::c_void),
    

        "glTexStorage3D" => Ok(glTexStorage3D as *mut core::ffi::c_void),
    

        "glTexStorage3DMultisample" => Ok(glTexStorage3DMultisample as *mut core::ffi::c_void),
    

        "glTexSubImage1D" => Ok(glTexSubImage1D as *mut core::ffi::c_void),
    

        "glTexSubImage2D" => Ok(glTexSubImage2D as *mut core::ffi::c_void),
    

        "glTexSubImage3D" => Ok(glTexSubImage3D as *mut core::ffi::c_void),
    

        "glTextureBarrier" => Ok(glTextureBarrier as *mut core::ffi::c_void),
    

        "glTextureBuffer" => Ok(glTextureBuffer as *mut core::ffi::c_void),
    

        "glTextureBufferRange" => Ok(glTextureBufferRange as *mut core::ffi::c_void),
    

        "glTextureParameterIiv" => Ok(glTextureParameterIiv as *mut core::ffi::c_void),
    

        "glTextureParameterIuiv" => Ok(glTextureParameterIuiv as *mut core::ffi::c_void),
    

        "glTextureParameterf" => Ok(glTextureParameterf as *mut core::ffi::c_void),
    

        "glTextureParameterfv" => Ok(glTextureParameterfv as *mut core::ffi::c_void),
    

        "glTextureParameteri" => Ok(glTextureParameteri as *mut core::ffi::c_void),
    

        "glTextureParameteriv" => Ok(glTextureParameteriv as *mut core::ffi::c_void),
    

        "glTextureStorage1D" => Ok(glTextureStorage1D as *mut core::ffi::c_void),
    

        "glTextureStorage2D" => Ok(glTextureStorage2D as *mut core::ffi::c_void),
    

        "glTextureStorage2DMultisample" => Ok(glTextureStorage2DMultisample as *mut core::ffi::c_void),
    

        "glTextureStorage3D" => Ok(glTextureStorage3D as *mut core::ffi::c_void),
    

        "glTextureStorage3DMultisample" => Ok(glTextureStorage3DMultisample as *mut core::ffi::c_void),
    

        "glTextureSubImage1D" => Ok(glTextureSubImage1D as *mut core::ffi::c_void),
    

        "glTextureSubImage2D" => Ok(glTextureSubImage2D as *mut core::ffi::c_void),
    

        "glTextureSubImage3D" => Ok(glTextureSubImage3D as *mut core::ffi::c_void),
    

        "glTextureView" => Ok(glTextureView as *mut core::ffi::c_void),
    

        "glTransformFeedbackBufferBase" => Ok(glTransformFeedbackBufferBase as *mut core::ffi::c_void),
    

        "glTransformFeedbackBufferRange" => Ok(glTransformFeedbackBufferRange as *mut core::ffi::c_void),
    

        "glTransformFeedbackVaryings" => Ok(glTransformFeedbackVaryings as *mut core::ffi::c_void),
    

        "glUniform1d" => Ok(glUniform1d as *mut core::ffi::c_void),
    

        "glUniform1dv" => Ok(glUniform1dv as *mut core::ffi::c_void),
    

        "glUniform1f" => Ok(glUniform1f as *mut core::ffi::c_void),
    

        "glUniform1fv" => Ok(glUniform1fv as *mut core::ffi::c_void),
    

        "glUniform1i" => Ok(glUniform1i as *mut core::ffi::c_void),
    

        "glUniform1iv" => Ok(glUniform1iv as *mut core::ffi::c_void),
    

        "glUniform1ui" => Ok(glUniform1ui as *mut core::ffi::c_void),
    

        "glUniform1uiv" => Ok(glUniform1uiv as *mut core::ffi::c_void),
    

        "glUniform2d" => Ok(glUniform2d as *mut core::ffi::c_void),
    

        "glUniform2dv" => Ok(glUniform2dv as *mut core::ffi::c_void),
    

        "glUniform2f" => Ok(glUniform2f as *mut core::ffi::c_void),
    

        "glUniform2fv" => Ok(glUniform2fv as *mut core::ffi::c_void),
    

        "glUniform2i" => Ok(glUniform2i as *mut core::ffi::c_void),
    

        "glUniform2iv" => Ok(glUniform2iv as *mut core::ffi::c_void),
    

        "glUniform2ui" => Ok(glUniform2ui as *mut core::ffi::c_void),
    

        "glUniform2uiv" => Ok(glUniform2uiv as *mut core::ffi::c_void),
    

        "glUniform3d" => Ok(glUniform3d as *mut core::ffi::c_void),
    

        "glUniform3dv" => Ok(glUniform3dv as *mut core::ffi::c_void),
    

        "glUniform3f" => Ok(glUniform3f as *mut core::ffi::c_void),
    

        "glUniform3fv" => Ok(glUniform3fv as *mut core::ffi::c_void),
    

        "glUniform3i" => Ok(glUniform3i as *mut core::ffi::c_void),
    

        "glUniform3iv" => Ok(glUniform3iv as *mut core::ffi::c_void),
    

        "glUniform3ui" => Ok(glUniform3ui as *mut core::ffi::c_void),
    

        "glUniform3uiv" => Ok(glUniform3uiv as *mut core::ffi::c_void),
    

        "glUniform4d" => Ok(glUniform4d as *mut core::ffi::c_void),
    

        "glUniform4dv" => Ok(glUniform4dv as *mut core::ffi::c_void),
    

        "glUniform4f" => Ok(glUniform4f as *mut core::ffi::c_void),
    

        "glUniform4fv" => Ok(glUniform4fv as *mut core::ffi::c_void),
    

        "glUniform4i" => Ok(glUniform4i as *mut core::ffi::c_void),
    

        "glUniform4iv" => Ok(glUniform4iv as *mut core::ffi::c_void),
    

        "glUniform4ui" => Ok(glUniform4ui as *mut core::ffi::c_void),
    

        "glUniform4uiv" => Ok(glUniform4uiv as *mut core::ffi::c_void),
    

        "glUniformBlockBinding" => Ok(glUniformBlockBinding as *mut core::ffi::c_void),
    

        "glUniformMatrix2dv" => Ok(glUniformMatrix2dv as *mut core::ffi::c_void),
    

        "glUniformMatrix2fv" => Ok(glUniformMatrix2fv as *mut core::ffi::c_void),
    

        "glUniformMatrix2x3dv" => Ok(glUniformMatrix2x3dv as *mut core::ffi::c_void),
    

        "glUniformMatrix2x3fv" => Ok(glUniformMatrix2x3fv as *mut core::ffi::c_void),
    

        "glUniformMatrix2x4dv" => Ok(glUniformMatrix2x4dv as *mut core::ffi::c_void),
    

        "glUniformMatrix2x4fv" => Ok(glUniformMatrix2x4fv as *mut core::ffi::c_void),
    

        "glUniformMatrix3dv" => Ok(glUniformMatrix3dv as *mut core::ffi::c_void),
    

        "glUniformMatrix3fv" => Ok(glUniformMatrix3fv as *mut core::ffi::c_void),
    

        "glUniformMatrix3x2dv" => Ok(glUniformMatrix3x2dv as *mut core::ffi::c_void),
    

        "glUniformMatrix3x2fv" => Ok(glUniformMatrix3x2fv as *mut core::ffi::c_void),
    

        "glUniformMatrix3x4dv" => Ok(glUniformMatrix3x4dv as *mut core::ffi::c_void),
    

        "glUniformMatrix3x4fv" => Ok(glUniformMatrix3x4fv as *mut core::ffi::c_void),
    

        "glUniformMatrix4dv" => Ok(glUniformMatrix4dv as *mut core::ffi::c_void),
    

        "glUniformMatrix4fv" => Ok(glUniformMatrix4fv as *mut core::ffi::c_void),
    

        "glUniformMatrix4x2dv" => Ok(glUniformMatrix4x2dv as *mut core::ffi::c_void),
    

        "glUniformMatrix4x2fv" => Ok(glUniformMatrix4x2fv as *mut core::ffi::c_void),
    

        "glUniformMatrix4x3dv" => Ok(glUniformMatrix4x3dv as *mut core::ffi::c_void),
    

        "glUniformMatrix4x3fv" => Ok(glUniformMatrix4x3fv as *mut core::ffi::c_void),
    

        "glUniformSubroutinesuiv" => Ok(glUniformSubroutinesuiv as *mut core::ffi::c_void),
    

        "glUnmapBuffer" => Ok(glUnmapBuffer as *mut core::ffi::c_void),
    

        "glUnmapNamedBuffer" => Ok(glUnmapNamedBuffer as *mut core::ffi::c_void),
    

        "glUseProgram" => Ok(glUseProgram as *mut core::ffi::c_void),
    

        "glUseProgramStages" => Ok(glUseProgramStages as *mut core::ffi::c_void),
    

        "glValidateProgram" => Ok(glValidateProgram as *mut core::ffi::c_void),
    

        "glValidateProgramPipeline" => Ok(glValidateProgramPipeline as *mut core::ffi::c_void),
    

        "glVertexArrayAttribBinding" => Ok(glVertexArrayAttribBinding as *mut core::ffi::c_void),
    

        "glVertexArrayAttribFormat" => Ok(glVertexArrayAttribFormat as *mut core::ffi::c_void),
    

        "glVertexArrayAttribIFormat" => Ok(glVertexArrayAttribIFormat as *mut core::ffi::c_void),
    

        "glVertexArrayAttribLFormat" => Ok(glVertexArrayAttribLFormat as *mut core::ffi::c_void),
    

        "glVertexArrayBindingDivisor" => Ok(glVertexArrayBindingDivisor as *mut core::ffi::c_void),
    

        "glVertexArrayElementBuffer" => Ok(glVertexArrayElementBuffer as *mut core::ffi::c_void),
    

        "glVertexArrayVertexBuffer" => Ok(glVertexArrayVertexBuffer as *mut core::ffi::c_void),
    

        "glVertexArrayVertexBuffers" => Ok(glVertexArrayVertexBuffers as *mut core::ffi::c_void),
    

        "glVertexAttrib1d" => Ok(glVertexAttrib1d as *mut core::ffi::c_void),
    

        "glVertexAttrib1dv" => Ok(glVertexAttrib1dv as *mut core::ffi::c_void),
    

        "glVertexAttrib1f" => Ok(glVertexAttrib1f as *mut core::ffi::c_void),
    

        "glVertexAttrib1fv" => Ok(glVertexAttrib1fv as *mut core::ffi::c_void),
    

        "glVertexAttrib1s" => Ok(glVertexAttrib1s as *mut core::ffi::c_void),
    

        "glVertexAttrib1sv" => Ok(glVertexAttrib1sv as *mut core::ffi::c_void),
    

        "glVertexAttrib2d" => Ok(glVertexAttrib2d as *mut core::ffi::c_void),
    

        "glVertexAttrib2dv" => Ok(glVertexAttrib2dv as *mut core::ffi::c_void),
    

        "glVertexAttrib2f" => Ok(glVertexAttrib2f as *mut core::ffi::c_void),
    

        "glVertexAttrib2fv" => Ok(glVertexAttrib2fv as *mut core::ffi::c_void),
    

        "glVertexAttrib2s" => Ok(glVertexAttrib2s as *mut core::ffi::c_void),
    

        "glVertexAttrib2sv" => Ok(glVertexAttrib2sv as *mut core::ffi::c_void),
    

        "glVertexAttrib3d" => Ok(glVertexAttrib3d as *mut core::ffi::c_void),
    

        "glVertexAttrib3dv" => Ok(glVertexAttrib3dv as *mut core::ffi::c_void),
    

        "glVertexAttrib3f" => Ok(glVertexAttrib3f as *mut core::ffi::c_void),
    

        "glVertexAttrib3fv" => Ok(glVertexAttrib3fv as *mut core::ffi::c_void),
    

        "glVertexAttrib3s" => Ok(glVertexAttrib3s as *mut core::ffi::c_void),
    

        "glVertexAttrib3sv" => Ok(glVertexAttrib3sv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nbv" => Ok(glVertexAttrib4Nbv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Niv" => Ok(glVertexAttrib4Niv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nsv" => Ok(glVertexAttrib4Nsv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nub" => Ok(glVertexAttrib4Nub as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nubv" => Ok(glVertexAttrib4Nubv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nuiv" => Ok(glVertexAttrib4Nuiv as *mut core::ffi::c_void),
    

        "glVertexAttrib4Nusv" => Ok(glVertexAttrib4Nusv as *mut core::ffi::c_void),
    

        "glVertexAttrib4bv" => Ok(glVertexAttrib4bv as *mut core::ffi::c_void),
    

        "glVertexAttrib4d" => Ok(glVertexAttrib4d as *mut core::ffi::c_void),
    

        "glVertexAttrib4dv" => Ok(glVertexAttrib4dv as *mut core::ffi::c_void),
    

        "glVertexAttrib4f" => Ok(glVertexAttrib4f as *mut core::ffi::c_void),
    

        "glVertexAttrib4fv" => Ok(glVertexAttrib4fv as *mut core::ffi::c_void),
    

        "glVertexAttrib4iv" => Ok(glVertexAttrib4iv as *mut core::ffi::c_void),
    

        "glVertexAttrib4s" => Ok(glVertexAttrib4s as *mut core::ffi::c_void),
    

        "glVertexAttrib4sv" => Ok(glVertexAttrib4sv as *mut core::ffi::c_void),
    

        "glVertexAttrib4ubv" => Ok(glVertexAttrib4ubv as *mut core::ffi::c_void),
    

        "glVertexAttrib4uiv" => Ok(glVertexAttrib4uiv as *mut core::ffi::c_void),
    

        "glVertexAttrib4usv" => Ok(glVertexAttrib4usv as *mut core::ffi::c_void),
    

        "glVertexAttribBinding" => Ok(glVertexAttribBinding as *mut core::ffi::c_void),
    

        "glVertexAttribDivisor" => Ok(glVertexAttribDivisor as *mut core::ffi::c_void),
    

        "glVertexAttribFormat" => Ok(glVertexAttribFormat as *mut core::ffi::c_void),
    

        "glVertexAttribI1i" => Ok(glVertexAttribI1i as *mut core::ffi::c_void),
    

        "glVertexAttribI1iv" => Ok(glVertexAttribI1iv as *mut core::ffi::c_void),
    

        "glVertexAttribI1ui" => Ok(glVertexAttribI1ui as *mut core::ffi::c_void),
    

        "glVertexAttribI1uiv" => Ok(glVertexAttribI1uiv as *mut core::ffi::c_void),
    

        "glVertexAttribI2i" => Ok(glVertexAttribI2i as *mut core::ffi::c_void),
    

        "glVertexAttribI2iv" => Ok(glVertexAttribI2iv as *mut core::ffi::c_void),
    

        "glVertexAttribI2ui" => Ok(glVertexAttribI2ui as *mut core::ffi::c_void),
    

        "glVertexAttribI2uiv" => Ok(glVertexAttribI2uiv as *mut core::ffi::c_void),
    

        "glVertexAttribI3i" => Ok(glVertexAttribI3i as *mut core::ffi::c_void),
    

        "glVertexAttribI3iv" => Ok(glVertexAttribI3iv as *mut core::ffi::c_void),
    

        "glVertexAttribI3ui" => Ok(glVertexAttribI3ui as *mut core::ffi::c_void),
    

        "glVertexAttribI3uiv" => Ok(glVertexAttribI3uiv as *mut core::ffi::c_void),
    

        "glVertexAttribI4bv" => Ok(glVertexAttribI4bv as *mut core::ffi::c_void),
    

        "glVertexAttribI4i" => Ok(glVertexAttribI4i as *mut core::ffi::c_void),
    

        "glVertexAttribI4iv" => Ok(glVertexAttribI4iv as *mut core::ffi::c_void),
    

        "glVertexAttribI4sv" => Ok(glVertexAttribI4sv as *mut core::ffi::c_void),
    

        "glVertexAttribI4ubv" => Ok(glVertexAttribI4ubv as *mut core::ffi::c_void),
    

        "glVertexAttribI4ui" => Ok(glVertexAttribI4ui as *mut core::ffi::c_void),
    

        "glVertexAttribI4uiv" => Ok(glVertexAttribI4uiv as *mut core::ffi::c_void),
    

        "glVertexAttribI4usv" => Ok(glVertexAttribI4usv as *mut core::ffi::c_void),
    

        "glVertexAttribIFormat" => Ok(glVertexAttribIFormat as *mut core::ffi::c_void),
    

        "glVertexAttribIPointer" => Ok(glVertexAttribIPointer as *mut core::ffi::c_void),
    

        "glVertexAttribL1d" => Ok(glVertexAttribL1d as *mut core::ffi::c_void),
    

        "glVertexAttribL1dv" => Ok(glVertexAttribL1dv as *mut core::ffi::c_void),
    

        "glVertexAttribL2d" => Ok(glVertexAttribL2d as *mut core::ffi::c_void),
    

        "glVertexAttribL2dv" => Ok(glVertexAttribL2dv as *mut core::ffi::c_void),
    

        "glVertexAttribL3d" => Ok(glVertexAttribL3d as *mut core::ffi::c_void),
    

        "glVertexAttribL3dv" => Ok(glVertexAttribL3dv as *mut core::ffi::c_void),
    

        "glVertexAttribL4d" => Ok(glVertexAttribL4d as *mut core::ffi::c_void),
    

        "glVertexAttribL4dv" => Ok(glVertexAttribL4dv as *mut core::ffi::c_void),
    

        "glVertexAttribLFormat" => Ok(glVertexAttribLFormat as *mut core::ffi::c_void),
    

        "glVertexAttribLPointer" => Ok(glVertexAttribLPointer as *mut core::ffi::c_void),
    

        "glVertexAttribP1ui" => Ok(glVertexAttribP1ui as *mut core::ffi::c_void),
    

        "glVertexAttribP1uiv" => Ok(glVertexAttribP1uiv as *mut core::ffi::c_void),
    

        "glVertexAttribP2ui" => Ok(glVertexAttribP2ui as *mut core::ffi::c_void),
    

        "glVertexAttribP2uiv" => Ok(glVertexAttribP2uiv as *mut core::ffi::c_void),
    

        "glVertexAttribP3ui" => Ok(glVertexAttribP3ui as *mut core::ffi::c_void),
    

        "glVertexAttribP3uiv" => Ok(glVertexAttribP3uiv as *mut core::ffi::c_void),
    

        "glVertexAttribP4ui" => Ok(glVertexAttribP4ui as *mut core::ffi::c_void),
    

        "glVertexAttribP4uiv" => Ok(glVertexAttribP4uiv as *mut core::ffi::c_void),
    

        "glVertexAttribPointer" => Ok(glVertexAttribPointer as *mut core::ffi::c_void),
    

        "glVertexBindingDivisor" => Ok(glVertexBindingDivisor as *mut core::ffi::c_void),
    

        "glVertexP2ui" => Ok(glVertexP2ui as *mut core::ffi::c_void),
    

        "glVertexP2uiv" => Ok(glVertexP2uiv as *mut core::ffi::c_void),
    

        "glVertexP3ui" => Ok(glVertexP3ui as *mut core::ffi::c_void),
    

        "glVertexP3uiv" => Ok(glVertexP3uiv as *mut core::ffi::c_void),
    

        "glVertexP4ui" => Ok(glVertexP4ui as *mut core::ffi::c_void),
    

        "glVertexP4uiv" => Ok(glVertexP4uiv as *mut core::ffi::c_void),
    

        "glViewport" => Ok(glViewport as *mut core::ffi::c_void),
    

        "glViewportArrayv" => Ok(glViewportArrayv as *mut core::ffi::c_void),
    

        "glViewportIndexedf" => Ok(glViewportIndexedf as *mut core::ffi::c_void),
    

        "glViewportIndexedfv" => Ok(glViewportIndexedfv as *mut core::ffi::c_void),
    

        "glWaitSync" => Ok(glWaitSync as *mut core::ffi::c_void),
    

    _ => Err(crate::errors::GLTraceError::HookNotFound(String::from(identifier)).into())
    

}
}

