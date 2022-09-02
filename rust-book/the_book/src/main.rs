//! # The Book
//!
//! `the_book` is a collection of all the chapters containing code examples and concepts from the rust book .

// mod hello_world;
// mod guessing_game;
// mod variables;
// mod data_types;
// mod functions;
// mod comments;
// mod control_flow;
// mod ownership;
// mod references;
// mod slice_type;
// mod structs;
// mod struct_by_example;
// mod method_syntax;
// mod enums;
// mod match_control_flow;
// mod if_let;
// mod vectors;
// mod strings;
// mod hashmaps;
// mod unrecoverable_errors;
// mod recoverable_errors;
// mod panic_or_not;
// mod generic_data_types;
// mod traits;
// mod references_and_lifetimes;
// mod closures;
// mod iterators;
// mod box_pointer;
// mod deref_trait;
// mod drop_trait;
// mod rc_smart_pointer;
mod refcell_smart_pointer;
fn main() {
    refcell_smart_pointer::run_refcell_smart_pointer();
}
