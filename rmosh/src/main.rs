use gc::GcRef;
use objects::{Object, Symbol};
extern crate num_derive;

pub mod alloc;
pub mod compiler;
pub mod equal;
pub mod fasl;
pub mod gc;
pub mod objects;
pub mod op;
pub mod procs;
pub mod vm2;

fn main() {}
