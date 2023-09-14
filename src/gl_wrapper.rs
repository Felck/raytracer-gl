use gl::types::*;
use std;
use std::ffi::CStr;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Self {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id);
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut error = Vec::with_capacity(len as usize);
            unsafe {
                error.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character

                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }

            panic!("{}", String::from_utf8_lossy(&error));
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id);
            }
        }

        Self { id: program_id }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Self {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut error = Vec::with_capacity(len as usize);
            unsafe { error.set_len((len as usize) - 1) }; // subtract 1 to skip the trailing null character

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            panic!("{}", String::from_utf8_lossy(&error));
        }

        Self { id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ArrayBuffer {
    pub id: GLuint,
}

impl ArrayBuffer {
    pub fn new<T>(size: usize, data: *const T) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,   // target
                size as GLsizeiptr, // size of data in bytes
                data.cast(),        // pointer to data
                gl::STATIC_DRAW,    // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self { id }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct UniformBuffer {
    pub id: GLuint,
}

impl UniformBuffer {
    pub fn new<T>(index: GLuint, size: usize, data: *const T) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::UNIFORM_BUFFER, id);
            gl::BufferData(
                gl::UNIFORM_BUFFER, // target
                size as GLsizeiptr, // size of data in bytes
                data.cast(),        // pointer to data
                gl::STATIC_DRAW,    // usage
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, id);
        }

        Self { id }
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    pub fn new(index: u32, components: i32, stride: usize, vbo: GLuint) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                index,             // index of the generic vertex attribute ("layout (location = 0)")
                components,        // the number of components per generic vertex attribute
                gl::FLOAT,         // data type
                gl::FALSE,         // normalized (int-to-float conversion)
                stride as GLsizei, // stride (byte offset between consecutive attributes)
                std::ptr::null(),  // offset of the first component
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self { id }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
