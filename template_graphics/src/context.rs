//! OpenGL context creation and management.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use template_core::TemplateResult;
use std::{ffi::CString, sync::Arc};
use tracing::info;
use crate::Window;

// ============================================================
// ========================== Types ===========================
// ============================================================

/// Shared OpenGL context type.
pub type GlContext = Arc<glow::Context>;

// ============================================================
// ===================== Structs & Impls ======================
// ============================================================

/// Builder for creating OpenGL contexts.
pub struct GlContextBuilder;

impl GlContextBuilder {
    /// Create a new context builder.
    pub fn new() -> Self {
        Self
    }

    /// Build an OpenGL context for the given window.
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn build(self, window: &Window) -> TemplateResult<GlContext> {
        info!("Creating OpenGL Context...");
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let s = CString::new(s).expect("failed to construct C string");
                window.get_proc_address(&s)
            })
        };
        info!("OpenGL Context created successfully");
        Ok(Arc::new(gl))
    }
}