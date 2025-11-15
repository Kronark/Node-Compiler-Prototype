use crate::nodes::data_value::DataValue;
mod nodes;

fn main() {
    let v = data_value!("hello world!");
    println!("{}", v);
}
