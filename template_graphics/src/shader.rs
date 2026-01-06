//! Shader program compilation and uniform management.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_core::{TemplateResult, TemplateError};
use crate::GlContext;
use glow::HasContext;
use nalgebra::Matrix4;
use std::collections::HashMap;
use tracing;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// Compiled shader program with uniform caching.
pub struct Shader {
    program: glow::Program,
    uniforms: HashMap<String, glow::UniformLocation>,
}

impl Shader {
    /// Create and compile a new shader program from vertex and fragment shader files.
    /// 
    /// # Arguments
    /// * `gl` - OpenGL context
    /// * `vertex_path` - Path to vertex shader file (relative to resources/shaders/)
    /// * `fragment_path` - Path to fragment shader file (relative to resources/shaders/)
    pub fn new(gl: &GlContext, vertex_path: &str, fragment_path: &str) -> TemplateResult<Self> {
        tracing::info!("Compiling shader program: {} + {}", vertex_path, fragment_path);
        
        let vertex_source = std::fs::read_to_string(format!("resources/shaders/{}", vertex_path))?;
        let fragment_source = std::fs::read_to_string(format!("resources/shaders/{}", fragment_path))?;
        
        let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, &vertex_source)?;
        let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, &fragment_source)?;
        
        let program = Self::link_program(gl, vertex_shader, fragment_shader)?;
        
        tracing::info!("Shader program compiled successfully");
        
        Ok(Self {
            program,
            uniforms: HashMap::new(),
        })
    }

    fn compile_shader(gl: &GlContext, shader_type: u32, source: &str) -> TemplateResult<glow::Shader> {
        let shader = unsafe { gl.create_shader(shader_type) }
            .map_err(|e| TemplateError::ShaderCompilation(e))?;
        
        unsafe {
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
            
            if !gl.get_shader_compile_status(shader) {
                let error = gl.get_shader_info_log(shader);
                return Err(TemplateError::ShaderCompilation(error));
            }
        }
        
        Ok(shader)
    }

    fn link_program(gl: &GlContext, vertex_shader: glow::Shader, fragment_shader: glow::Shader) -> TemplateResult<glow::Program> {
        let program = unsafe { gl.create_program() }
            .map_err(|e| TemplateError::ShaderCompilation(e))?;
        
        unsafe {
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            
            if !gl.get_program_link_status(program) {
                let error = gl.get_program_info_log(program);
                return Err(TemplateError::ShaderCompilation(error));
            }
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
        }
        
        Ok(program)
    }

    /// Bind this shader program for rendering.
    pub fn bind(&self, gl: &GlContext) {
        tracing::trace!("Binding shader program");
        unsafe { gl.use_program(Some(self.program)); }
    }

    /// Set a 4x4 matrix uniform.
    pub fn set_matrix4(&mut self, gl: &GlContext, name: &str, matrix: &Matrix4<f32>) {
        tracing::trace!("Setting matrix uniform: {}", name);
        let location = self.get_uniform_location(gl, name);
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(&location), false, matrix.as_slice());
        }
    }

    fn get_uniform_location(&mut self, gl: &GlContext, name: &str) -> glow::UniformLocation {
        *self.uniforms.entry(name.to_string()).or_insert_with(|| {
            unsafe { gl.get_uniform_location(self.program, name).unwrap() }
        })
    }
}
