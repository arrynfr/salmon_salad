#[cfg(debug_assertions)]
pub fn is_debug() -> bool {true}

#[cfg(not(debug_assertions))]
pub fn is_debug() -> bool {false}

