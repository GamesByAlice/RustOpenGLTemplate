//! Application event handler for the Template engine.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_graphics::{Renderer, Shader, Mesh};
use nalgebra::{Matrix4, Perspective3};
use std::time::Instant;
use tracing;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// Main application state and event handler.
pub struct TemplateApp {
    pub renderer: Renderer,
    pub mesh: Mesh,
    pub shader: Shader,
    pub projection: Perspective3<f32>,
    pub view: Matrix4<f32>,
    pub model: Matrix4<f32>,
    pub start_time: Instant,
}

impl winit::application::ApplicationHandler for TemplateApp {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
    
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(physical_size) => {
                handle_resize(&self.renderer, &mut self.projection, physical_size);
            }
            winit::event::WindowEvent::RedrawRequested => {
                let elapsed = self.start_time.elapsed().as_secs_f32();
                let rotation_x = Matrix4::from_axis_angle(&nalgebra::Vector3::x_axis(), elapsed * 0.5);
                let rotation_y = Matrix4::from_axis_angle(&nalgebra::Vector3::y_axis(), elapsed * 0.7);
                self.model = rotation_y * rotation_x;
                
                render_frame(&self.renderer, &self.mesh, &mut self.shader, &self.projection, &self.view, &self.model);
            }
            _ => {}
        }
    }
    
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.renderer.window.handle().request_redraw();
    }
}

// ============================================================
// ==================== Helper Functions ======================
// ============================================================

/// Handle window resize events.
fn handle_resize(
    renderer: &Renderer,
    projection: &mut Perspective3<f32>,
    physical_size: winit::dpi::PhysicalSize<u32>,
) {
    tracing::debug!("Window resized to {}x{}", physical_size.width, physical_size.height);
    
    renderer.resize(physical_size.width, physical_size.height);
    let aspect = physical_size.width as f32 / physical_size.height as f32;
    *projection = Perspective3::new(aspect, 45.0_f32.to_radians(), 0.1, 100.0);
}

/// Render a single frame.
fn render_frame(
    renderer: &Renderer,
    mesh: &Mesh,
    shader: &mut Shader,
    projection: &Perspective3<f32>,
    view: &Matrix4<f32>,
    model: &Matrix4<f32>,
) {
    renderer.clear();
    
    shader.bind(&renderer.gl);
    shader.set_matrix4(&renderer.gl, "projection", projection.as_matrix());
    shader.set_matrix4(&renderer.gl, "view", view);
    shader.set_matrix4(&renderer.gl, "model", model);
    
    mesh.draw(&renderer.gl);
    
    if let Err(e) = renderer.present() {
        tracing::error!("Render error: {}", e);
    }
}