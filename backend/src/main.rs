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
            0 => out_socket!(
                default: data_value!(0xFF)
            ),
            42 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data_value!("hi!"),
                data_type!(identifier!("maths", "integer")),
                data_type!(identifier!("maths", "float"))
            ),
            69 => in_socket!(
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data_value!("bye!"),
                data_type!(identifier!("text", "utf8")),
                data_type!(identifier!("text", "utx16"))
            )
        ),
        4 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            2 => out_socket!(
                default: data_value!(0xFF)
            ),
            70 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data_value!("hi!"),
                data_type!(identifier!("maths", "integer")),
                data_type!(identifier!("maths", "float"))
            ),
            55 => in_socket!(
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
