mod nodes;

fn main() {
    let mut ip = pool!();
    let mut sp = pool!();

    let i1 = ip.reserve(1);
    let s1 = sp.reserve(0);
    let s2 = sp.reserve(42);
    let s3 = sp.reserve(69);
    let i2 = ip.reserve(2);
    let s4 = sp.reserve(2);
    let s5 = sp.reserve(79);
    let s6 = sp.reserve(33);

    let n = node!(
        false,
        type: node_type!(identifier!("project", "mathematics", "algebra", "add")),
        roots: node_roots!(
            connection!(42, 3, data_type!(identifier!("project", "numbers", "integer"))),
            connection!(33, 6, data_type!(identifier!("project", "numbers", "float")))
        ),
        id_pool: ip,
        slot_pool: sp,
        i1 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            s1 => out_socket!(
                default: data!(data_type!(identifier!("data", "hex")), data_value!(0x86))
            ),
            s2 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("hello!")),
                permitted: [
                    data_type!(identifier!("maths", "integer")),
                    data_type!(identifier!("maths", "float"))
                ]
            ),
            s3 => in_socket!(
                type: socket_type!(color),
                parameters: socket_parameters!(color),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("bye!")),
                permitted: [
                    data_type!(identifier!("text", "utf8")),
                    data_type!(identifier!("text", "utx16"))
                ]
            )
        ),
        i2 => instance!(
            node_type!(identifier!("project", "x64", "add", "add-registers")),
            s4 => out_socket!(
                default: data!(data_type!(identifier!("data", "hex")), data_value!(0xFF))
            ),
            s5 => rep_socket!(
                type: socket_type!(number),
                parameters: socket_parameters!(number, min: "-50", max: "50", step: "0.5"),
                default: data!(data_type!(identifier!("text", "utf8")), data_value!("hi!")),
                actual: data!(data_type!(identifier!("text", "utf8")), data_value!("mornin!")),
                permitted: [
                    data_type!(identifier!("maths", "integer")),
                    data_type!(identifier!("maths", "float"))
                ]
            ),
            s6 => in_socket!(
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
