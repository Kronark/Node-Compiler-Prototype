use std::{collections::HashMap, fmt::Display};
use crate::nodes::connection::Connection;

pub struct NodeRoots {
    connections: HashMap<u32, Vec<Connection>>
}

impl NodeRoots {
    pub fn new<I>(connections: I) -> Self
    where
        I: IntoIterator<Item = Connection>,
    {
        let mut map: HashMap<u32, Vec<Connection>> = HashMap::new();

        for connection in connections {
            map.entry(connection.instance_id)
                .or_insert_with(Vec::new)
                .push(connection);
        }

        Self { connections: map }
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.entry(connection.instance_id).or_insert_with(Vec::new).push(connection);
    }

    pub fn remove_connection(&mut self, instance_id: u32, socket_slot: u32) {
        if let Some(vec) = self.connections.get_mut(&instance_id) {
            vec.retain(|connection| connection.socket_slot != socket_slot);
            if vec.is_empty() {
                self.connections.remove(&instance_id);
            }
        }
    }

    pub fn remove_connections(&mut self, instance_id: u32) {
        self.connections.remove(&instance_id);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values().flatten()
    }

    pub fn count(&self) -> usize {
        self.connections.values().map(|vector| vector.len()).sum()
    }
}

impl Display for NodeRoots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.connections.is_empty() {
            writeln!(f, "no output connections")?;
        } else {
            writeln!(f, "output connections:")?;
            for connection in self.connections.values().flatten() {
                writeln!(f, "    • {}", connection)?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! node_roots {
    ($($connections:expr),+ $(,)?) => {{
        $crate::nodes::node_roots::NodeRoots::new([$( $connections ),+])
    }};
}
