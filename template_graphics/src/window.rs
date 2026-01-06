//! Window creation and management using glutin and winit.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_core::{TemplateResult, TemplateError};
use glutin::{display::GetGlDisplay, prelude::*, surface::GlSurface, context::NotCurrentGlContext};
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use tracing::info;
use winit::event_loop::EventLoop;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// Window wrapper with OpenGL context and surface.
pub struct Window {
    handle: winit::window::Window,
    context: glutin::context::PossiblyCurrentContext,
    display: glutin::display::Display,
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

impl Window {
    /// Create a new window with the specified dimensions and title.
    pub fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> TemplateResult<Self> {
        info!("Creating window {}x{}", width, height);
        
        let (window, gl_config) = Self::create_window_and_config(width, height, title, event_loop)?;
        let display = gl_config.display();
        let context = Self::create_context(&window, &gl_config, &display)?;
        let surface = Self::create_surface(&window, &gl_config, &display, width, height)?;
        let context = Self::make_context_current(context, &surface)?;
        
        Self::configure_surface(&surface, &context)?;
        
        info!("Window created successfully");
        Ok(Self { handle: window, context, display, surface })
    }

    fn create_window_and_config(
        width: u32, 
        height: u32, 
        title: &str, 
        event_loop: &EventLoop<()>
    ) -> TemplateResult<(winit::window::Window, glutin::config::Config)> {
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));
        
        let template = glutin::config::ConfigTemplateBuilder::new()
            .with_alpha_size(8);
        
        let display_builder = glutin_winit::DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes));
        
        let (window, gl_config) = display_builder
            .build(event_loop, template, |mut configs| {
                configs.next().unwrap()
            })
            .map_err(|e| TemplateError::WindowCreation(e.to_string()))?;
        
        Ok((window.unwrap(), gl_config))
    }

    fn create_context(
        window: &winit::window::Window,
        gl_config: &glutin::config::Config,
        display: &glutin::display::Display
    ) -> TemplateResult<glutin::context::NotCurrentContext> {
        let window_handle = window.window_handle()
            .map_err(|e| TemplateError::WindowCreation(e.to_string()))?;
        
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .build(Some(window_handle.as_raw()));
        
        unsafe {
            display.create_context(gl_config, &context_attributes)
                .map_err(|e| TemplateError::WindowCreation(e.to_string()))
        }
    }

    fn create_surface(
        window: &winit::window::Window,
        gl_config: &glutin::config::Config,
        display: &glutin::display::Display,
        width: u32,
        height: u32
    ) -> TemplateResult<glutin::surface::Surface<glutin::surface::WindowSurface>> {
        let window_handle = window.window_handle()
            .map_err(|e| TemplateError::WindowCreation(e.to_string()))?;
        
        let surface_attributes = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
            .build(window_handle.as_raw(), NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap());
        
        unsafe {
            display.create_window_surface(gl_config, &surface_attributes)
                .map_err(|e| TemplateError::WindowCreation(e.to_string()))
        }
    }

    fn make_context_current(
        context: glutin::context::NotCurrentContext,
        surface: &glutin::surface::Surface<glutin::surface::WindowSurface>
    ) -> TemplateResult<glutin::context::PossiblyCurrentContext> {
        context.make_current(surface)
            .map_err(|e| TemplateError::WindowCreation(e.to_string()))
    }

    fn configure_surface(
        surface: &glutin::surface::Surface<glutin::surface::WindowSurface>,
        context: &glutin::context::PossiblyCurrentContext
    ) -> TemplateResult<()> {
        surface.set_swap_interval(context, glutin::surface::SwapInterval::Wait(NonZeroU32::MIN))
            .map_err(|e| TemplateError::WindowCreation(e.to_string()))
    }

    /// Swap the front and back buffers.
    pub fn swap_buffers(&self) -> TemplateResult<()> {
        self.surface.swap_buffers(&self.context)
            .map_err(|e| TemplateError::OpenGL(e.to_string()))?;
        self.handle.request_redraw();
        Ok(())
    }

    /// Get the address of an OpenGL function.
    pub fn get_proc_address(&self, addr: &std::ffi::CStr) -> *const std::ffi::c_void {
        self.display.get_proc_address(addr)
    }

    /// Get a reference to the underlying winit window.
    pub fn handle(&self) -> &winit::window::Window {
        &self.handle
    }
}