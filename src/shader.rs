extern crate gl;
use std::error::Error;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::cell::Cell;

#[derive(Debug)]
pub struct ShaderProgram {
    id: Cell<GLuint>,
    vertex_filename: String,
    fragment_filename: String,
}

impl ShaderProgram {
    pub fn new(
        vertex_filename: &str,
        fragment_filename: &str,
    ) -> Result<ShaderProgram, Box<Error>> {
        let vertex_src: &str = &read_from_file(vertex_filename);
        let fragment_src: &str = &read_from_file(fragment_filename);

        let id = unsafe { create_shader_program(vertex_src, fragment_src)? };
        Ok(ShaderProgram {
            id: Cell::new(id),
            vertex_filename: vertex_filename.to_string(),
            fragment_filename: fragment_filename.to_string(),
        })
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id.get());
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn reload(&self) {
        println!("Reloading shader ({} + {})", self.vertex_filename, self.fragment_filename);
        let vertex_src: &str = &read_from_file(&self.vertex_filename);
        let fragment_src: &str = &read_from_file(&self.fragment_filename);
        let id = unsafe { create_shader_program(vertex_src, fragment_src).expect("Could not create shader program") };
        self.id.set(id);
    }
}

fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Could not read file");
    s
}

unsafe fn create_shader(src: &str, shader_type: GLuint) -> Result<GLuint, Box<Error>> {
    let vertex_shader = gl::CreateShader(shader_type);
    let c_str_vert = CString::new(src.as_bytes()).expect("Could not create vertex shader c string");
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            vertex_shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(
            format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).expect("Cannot read info_log")
            ).into(),
        );
    }
    Ok(vertex_shader)
}

unsafe fn create_shader_program(
    vertex_src: &str,
    fragment_src: &str,
) -> Result<GLuint, Box<Error>> {
    let vertex_shader = create_shader(vertex_src, gl::VERTEX_SHADER)?;
    let fragment_shader = create_shader(fragment_src, gl::FRAGMENT_SHADER)?;

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(
            format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            ).into(),
        );
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    Ok(shader_program)
}

unsafe fn update_shader_program(
    shader_program: GLuint,
    vertex_src: &str,
    fragment_src: &str,
) -> Result<(), Box<Error>> {
    let vertex_shader = create_shader(vertex_src, gl::VERTEX_SHADER)?;
    let fragment_shader = create_shader(fragment_src, gl::FRAGMENT_SHADER)?;

    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(
            format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            ).into(),
        );
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
    Ok(())
}
