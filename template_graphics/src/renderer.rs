//! Main renderer coordination.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_core::TemplateResult;
use crate::{Window, GlContext, GlContextBuilder};
use glow::HasContext;
use winit::event_loop::EventLoop;
use tracing;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// Main renderer that coordinates window, context, and rendering operations.
pub struct Renderer {
    pub window: Window,
    pub gl: GlContext,
}

impl Renderer {
    /// Create a new renderer with window and OpenGL context.
    /// 
    /// # Arguments
    /// * `width` - Window width in pixels
    /// * `height` - Window height in pixels  
    /// * `title` - Window title
    /// * `event_loop` - Winit event loop
    /// 
    /// # Returns
    /// A configured renderer ready for use
    pub fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> TemplateResult<Self> {
        tracing::info!("Initializing renderer {}x{}", width, height);
        
        let window = Window::new(width, height, title, event_loop)?;
        let gl = GlContextBuilder::new().build(&window)?;
        
        // Configure OpenGL state
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
            gl.clear_color(0.2, 0.3, 0.3, 1.0); // Dark teal background
        }
        
        tracing::info!("Renderer initialized successfully");
        
        Ok(Self { window, gl })
    }

    /// Clear the color and depth buffers.
    pub fn clear(&self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    /// Present the rendered frame.
    pub fn present(&self) -> TemplateResult<()> {
        self.window.swap_buffers()
    }

    /// Update viewport when window is resized.
    /// 
    /// # Arguments
    /// * `width` - New viewport width
    /// * `height` - New viewport height
    pub fn resize(&self, width: u32, height: u32) {
        tracing::debug!("Resizing viewport to {}x{}", width, height);
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}