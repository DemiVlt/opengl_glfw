use std::{
    ffi::{CStr, CString},
    fs,
    path::Path,
    ptr,
};

unsafe fn log_shader_complation_err(shader: u32, name: &str) {
    let mut success = true as _;

    let mut info_log = vec![0; 512];
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != true as _ {
        gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr().cast());
        println!("ERROR::SHADER::{}::COMPILATION_FAILED", name);
        println!("{}", String::from_utf8(info_log).unwrap());
    }
}

pub struct Shader {
    id: u32,
}

#[allow(dead_code)]
impl Shader {
    pub unsafe fn new(vertex_path: &Path, fragment_path: &Path) -> Self {
        // retrieve vertex & shader source code
        let (Ok(vertex_shader_bytes), Ok(fragment_shader_bytes)) =
            (fs::read(vertex_path), fs::read(fragment_path))
        else {
            panic!("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ");
        };

        let (Ok(vertex_shader_source), Ok(fragment_shader_source)) = (
            CString::new(vertex_shader_bytes),
            CString::new(fragment_shader_bytes),
        ) else {
            panic!("CString::new failed");
        };

        // compile vertex_shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        log_shader_complation_err(vertex_shader, "VERTEX");

        // compile fragment_shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(fragment_shader);
        log_shader_complation_err(fragment_shader, "FRAGMENT");

        // init shader program
        let id = gl::CreateProgram();

        gl::AttachShader(id, vertex_shader);
        gl::AttachShader(id, fragment_shader);
        gl::LinkProgram(id);

        let mut success = true as _;
        let info_log = CString::new([0; 511]).unwrap();
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

        if success != true as _ {
            gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_ptr().cast_mut());
            println!("ERROR::PROGRAM::SHADER_PROGRAM::LINKING_FAILED");
            panic!("{}", info_log.to_str().expect("CStr::to_str failed"));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        Self { id }
    }

    pub unsafe fn activate(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform1i(location, value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform1f(location, value);
    }
}
