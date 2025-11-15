use crate::nodes::{node_type::NodeType, socket::Socket};

pub struct Instance
{
    pub type_: NodeType,
    pub sockets: Vec<Socket>
}
