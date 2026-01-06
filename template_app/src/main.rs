//! Template Engine - A minimal 3D rendering engine in Rust.

// ============================================================
// ========================= Modules ==========================
// ============================================================

mod app;

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_core::{TemplateResult, TemplateError};
use template_graphics::{Renderer, Shader, Mesh};
use nalgebra::{Matrix4, Vector3, Point3, Perspective3};
use winit::event_loop::EventLoop;
use tracing;
use std::time::Instant;
use app::TemplateApp;

// ============================================================
// ==================== Global Functions ======================
// ============================================================

/// Application entry point.
fn main() -> TemplateResult<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Template Engine v0.1.0");

    let event_loop = EventLoop::new().unwrap();
    let renderer = Renderer::new(800, 600, "Template Engine", &event_loop)?;
    
    let cube_vertices = create_cube_vertices();
    
    let mesh = Mesh::new(&renderer.gl, &cube_vertices);
    let shader = Shader::new(&renderer.gl, "basic.vert", "basic.frag")?;
    
    let projection = Perspective3::new(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);
    let view = create_view_matrix();
    let model = Matrix4::identity();
    let start_time = Instant::now();
    
    tracing::info!("Entering main event loop");
    run_event_loop(event_loop, renderer, mesh, shader, projection, view, model, start_time)
}

/// Create cube vertices with colors (36 vertices for 12 triangles, 6 floats per vertex).
fn create_cube_vertices() -> Vec<f32> {
    vec![
        // Front face (red-ish corners)
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  // Bottom left - red
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,  // Bottom right - yellow
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Top right - magenta
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Top right - magenta
        -0.5,  0.5,  0.5,  0.0, 1.0, 0.0,  // Top left - green
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  // Bottom left - red
        // Back face (blue-ish corners)
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Bottom left - blue
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  // Top left - cyan
         0.5,  0.5, -0.5,  1.0, 1.0, 1.0,  // Top right - white
         0.5,  0.5, -0.5,  1.0, 1.0, 1.0,  // Top right - white
         0.5, -0.5, -0.5,  0.5, 0.5, 0.5,  // Bottom right - gray
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Bottom left - blue
        // Left face
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Back bottom - blue
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  // Front bottom - red
        -0.5,  0.5,  0.5,  0.0, 1.0, 0.0,  // Front top - green
        -0.5,  0.5,  0.5,  0.0, 1.0, 0.0,  // Front top - green
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  // Back top - cyan
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Back bottom - blue
        // Right face
         0.5, -0.5, -0.5,  0.5, 0.5, 0.5,  // Back bottom - gray
         0.5,  0.5, -0.5,  1.0, 1.0, 1.0,  // Back top - white
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Front top - magenta
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Front top - magenta
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,  // Front bottom - yellow
         0.5, -0.5, -0.5,  0.5, 0.5, 0.5,  // Back bottom - gray
        // Top face
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  // Back left - cyan
        -0.5,  0.5,  0.5,  0.0, 1.0, 0.0,  // Front left - green
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Front right - magenta
         0.5,  0.5,  0.5,  1.0, 0.0, 1.0,  // Front right - magenta
         0.5,  0.5, -0.5,  1.0, 1.0, 1.0,  // Back right - white
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  // Back left - cyan
        // Bottom face
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Back left - blue
         0.5, -0.5, -0.5,  0.5, 0.5, 0.5,  // Back right - gray
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,  // Front right - yellow
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,  // Front right - yellow
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  // Front left - red
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  // Back left - blue
    ]
}

/// Create the view matrix for the camera.
fn create_view_matrix() -> Matrix4<f32> {
    Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    )
}

/// Run the main event loop.
fn run_event_loop(
    event_loop: EventLoop<()>,
    renderer: Renderer,
    mesh: Mesh,
    shader: Shader,
    projection: Perspective3<f32>,
    view: Matrix4<f32>,
    model: Matrix4<f32>,
    start_time: Instant,
) -> TemplateResult<()> {
    let mut app = TemplateApp {
        renderer,
        mesh,
        shader,
        projection,
        view,
        model,
        start_time,
    };
    
    event_loop.run_app(&mut app)
        .map_err(|e| TemplateError::WindowCreation(e.to_string()))?;
    
    Ok(())
}