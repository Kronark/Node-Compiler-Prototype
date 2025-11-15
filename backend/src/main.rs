use crate::nodes::{data_value::DataValue, socket_parameters::SocketParameters};
mod nodes;

fn main() {
    let v = data_value!("hello world!");
    println!("{}", v);

    let p1 = socket_parameters!(named);
    println!("{}", p1);

    let p2 = socket_parameters!(number, min: "0", max: "69", step: "0.5");
    println!("{}", p2);

    let p3 = socket_parameters!(color);
    println!("{}", p3);

    let p4 = socket_parameters!(switch);
    println!("{}", p4);

    let p5 = socket_parameters!(text, min: "1", max: "20", valid: "");
    println!("{}", p5);

    let p6 = socket_parameters!(select, "option 1", "option 2", "option 3");
    println!("{}", p6);
}
