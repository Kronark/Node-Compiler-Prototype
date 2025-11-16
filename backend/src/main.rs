mod nodes;

fn main() {
    let ip = pool!();
    let sp = pool!();
    let n = node!(
        true,
        type: node_type!(identifier!("project", "mathematics", "algebra", "add")),
        roots: node_roots!(
            connection!(42, 3, data_type!(identifier!("project", "numbers", "integer"))),
            connection!(33, 6, data_type!(identifier!("project", "numbers", "float")))
        ),
        id_pool: ip,
        slot_pool: sp,
        1 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            0 => out_socket!(
                default: data!(data_type!(identifier!("data", "hex")), data_value!(0x86))
            ),
            42 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("hello!")),
                permitted: [
                    data_type!(identifier!("maths", "integer")),
                    data_type!(identifier!("maths", "float"))
                ]
            ),
            69 => in_socket!(
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("bye!")),
                permitted: [
                    data_type!(identifier!("text", "utf8")),
                    data_type!(identifier!("text", "utx16"))
                ]
            )
        ),
        4 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            2 => out_socket!(
                default: data!(data_type!(identifier!("data", "hex")), data_value!(0xFF))
            ),
            70 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("hi!")),
                actual: data!(data_type!(identifier!("text", "utf8")), data_value!("mornin!")),
                permitted: [
                    data_type!(identifier!("maths", "integer")),
                    data_type!(identifier!("maths", "float"))
                ]
            ),
            55 => in_socket!(
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("bye!")),
                permitted: [
                    data_type!(identifier!("text", "utf8")),
                    data_type!(identifier!("text", "utx16"))
                ]
            )
        ),
    );
    println!("{}", n);
}
