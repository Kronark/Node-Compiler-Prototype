mod nodes;

fn main() {
    let n = node!(
        true,
        type: node_type!(identifier!("project", "mathematics", "algebra", "add")),
        roots: node_roots!(
            connection!(42, 3, data_type!(identifier!("project", "numbers", "integer"))),
            connection!(33, 6, data_type!(identifier!("project", "numbers", "float")))
        ),
        1 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            out_socket!(
                slot: 0,
                default: data_value!(0xFF)
            ),
            rep_socket!(
                slot: 69,
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data_value!("hi!"),
                data_type!(identifier!("maths", "integer")),
                data_type!(identifier!("maths", "float"))
            ),
            in_socket!(
                slot: 42,
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data_value!("bye!"),
                data_type!(identifier!("text", "utf8")),
                data_type!(identifier!("text", "utx16"))
            )
        ),
        4 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            out_socket!(
                slot: 2,
                default: data_value!(0xFF)
            ),
            rep_socket!(
                slot: 70,
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data_value!("hi!"),
                data_type!(identifier!("maths", "integer")),
                data_type!(identifier!("maths", "float"))
            ),
            in_socket!(
                slot: 44,
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data_value!("bye!"),
                data_type!(identifier!("text", "utf8")),
                data_type!(identifier!("text", "utx16"))
            )
        ),
    );
    println!("{}", n);
}
