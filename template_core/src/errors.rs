//! Core error types and utilities for the Template engine.

// ============================================================
// ========================= Imports ==========================
// ============================================================

use thiserror::Error;

// ============================================================
// ====================== Types & Enums ======================
// ============================================================

/// Main error type for Template engine operations.
#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("OpenGL error: {0}")]
    OpenGL(String),
    #[error("Shader compilation error: {0}")]
    ShaderCompilation(String),
    #[error("Window creation error: {0}")]
    WindowCreation(String),
}
