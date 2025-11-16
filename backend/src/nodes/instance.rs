use std::{collections::HashMap, fmt::Display, sync::Arc};
use crate::nodes::{node_type::NodeType, socket::Socket};

pub struct Instance {
    pub type_: Arc<NodeType>,
    pub sockets: HashMap<u32, Socket>
}

impl Instance {
    pub fn new(type_: Arc<NodeType>, sockets: HashMap<u32, Socket>) -> Self {
        Self {
            type_,
            sockets
        }
    }

    pub fn node_type(&self) -> &NodeType {
        &self.type_
    }

    pub fn sockets(&self) -> &HashMap<u32, Socket> {
        &self.sockets
    }

    pub fn set_socket(&mut self, socket: Socket) {
        self.sockets.insert(socket.slot, socket);
    }

    pub fn get_socket(&mut self, slot: u32) -> Option<&mut Socket> {
        self.sockets.get_mut(&slot)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.type_)?;

        if self.sockets.is_empty() {
            writeln!(f, "no sockets")?;
            return Ok(());
        }

        let mut sockets: Vec<_> = self.sockets.values().collect();
        sockets.sort_by_key(|socket| socket.slot);

        writeln!(f, "sockets:")?;
        for socket in sockets {
            let socket_string = socket.to_string();
            for line in socket_string.lines() {
                writeln!(f, "    {}", line)?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! instance {
    ($type:expr $(, $socket:expr)* $(,)?) => {{
        $crate::nodes::instance::Instance::new(
            $type,
            [$( ($socket.slot, $socket) ),*].into_iter().collect()
        )
    }};
}
