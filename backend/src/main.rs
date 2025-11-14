use crate::nodes::identifier::{Identifier, IdentifierComponent};
mod nodes;

fn main() {
    let idc = identifier_component!("project");
    let id2 = identifier!(idc.data(), "mathematics", "addition", "add-integers");
    println!("{}", id2);
}
