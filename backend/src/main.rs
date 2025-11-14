use crate::nodes::identifier::{Identifier, IdentifierComponent};
mod nodes;

fn main() {
    let a = identifier_component!("project-o");
    let b = identifier!(a.data(), "mathematics", "algebra", "math");
    println!("{}", b)
}
