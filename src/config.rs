//! Config variables that control certain code pathways at compile time.  
//! This might be replaced by something else in the future

#[cfg(debug_assertions)]
pub const IS_DEBUG: bool = true;

#[cfg(not(debug_assertions))]
pub const IS_DEBUG: bool = false;