//! Mesh rendering with vertex buffer management.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use crate::GlContext;
use glow::HasContext;
use bytemuck;
use tracing;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// A renderable mesh with vertex data.
pub struct Mesh {
    vao: glow::VertexArray,
    #[allow(dead_code)]
    vbo: glow::Buffer,
    vertex_count: i32,
}

impl Mesh {
    /// Create a new mesh from vertex data with positions and colors.
    /// 
    /// # Arguments
    /// * `gl` - OpenGL context
    /// * `vertices` - Vertex data (6 floats per vertex: x, y, z, r, g, b)
    /// 
    /// # Returns
    /// A new mesh ready for rendering
    pub fn new(gl: &GlContext, vertices: &[f32]) -> Self {
        tracing::debug!("Creating mesh with {} vertices", vertices.len() / 6);
        
        unsafe {
            // Create OpenGL objects
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            
            // Bind VAO to capture vertex attribute state
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            
            // Upload vertex data to GPU
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            
            // Configure vertex attributes
            // Position (location 0): 3 floats starting at offset 0
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 24, 0);
            
            // Color (location 1): 3 floats starting at offset 12 (3 * 4 bytes)
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 24, 12);
            
            // Unbind VAO to prevent accidental modification
            gl.bind_vertex_array(None);
            
            tracing::debug!("Mesh created successfully");
            
            Self {
                vao,
                vbo, // Kept alive for RAII cleanup
                vertex_count: Self::calculate_vertex_count(vertices),
            }
        }
    }

    /// Render the mesh using triangles.
    /// 
    /// # Arguments
    /// * `gl` - OpenGL context for rendering
    pub fn draw(&self, gl: &GlContext) {
        tracing::trace!("Drawing mesh with {} vertices", self.vertex_count);
        
        unsafe {
            // Bind VAO containing vertex attribute configuration
            gl.bind_vertex_array(Some(self.vao));
            
            // Issue draw call
            gl.draw_arrays(glow::TRIANGLES, 0, self.vertex_count);
            
            // Clean up binding
            gl.bind_vertex_array(None);
        }
    }

    /// Calculate vertex count from raw vertex data with position and color.
    /// 
    /// # Arguments
    /// * `vertices` - Raw vertex data (6 floats per vertex: x, y, z, r, g, b)
    /// 
    /// # Returns
    /// Number of vertices
    pub fn calculate_vertex_count(vertices: &[f32]) -> i32 {
        (vertices.len() / 6) as i32
    }
}
