mod nodes;

fn main() {
    let s = out_socket!(
        slot: 0,
        default: data_value!(0xFF)
    );
    println!("{}", s);

    let i = rep_socket!(
        slot: 0,
        type: socket_type!(number),
        parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
        default: data_value!("hi!"),
        data_type!(identifier!("maths", "integer")),
        data_type!(identifier!("maths", "float"))
    );
    println!("{}", i);
}
