extern crate liquid;
extern crate markdown;
extern crate walkdir;

// without this main.rs would have to use cobalt::cobalt
// with this approach you can explicitly say which part of a module is public and which not
pub use cobalt::build;

// modules
mod cobalt;
mod document;
