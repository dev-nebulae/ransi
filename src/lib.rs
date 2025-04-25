//! Ransi - A rust library for ANSI escape codes
//! ============================================
//!
//! Ransi is a simple library for working with ANSI escape codes in Rust.
//!
//! It provides a set of functions to easily manage ANSI escape codes for
//! color, formatting, and cursor movement in terminal applications.
//!
//! Features:
//! - Color formatting (foreground and background)
//! - Text formatting (bold, underline, etc.)
//! - Cursor movement (up, down, left, right)
//! - Clearing the screen
//! - Clearing the line
//! - Saving and restoring cursor position
//! - Hiding and showing the cursor
//! - Getting terminal size

pub mod colors;
pub mod layout;
pub mod screen;
pub mod term;
pub mod cursor;
