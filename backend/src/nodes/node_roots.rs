use std::{fmt::Display};
use crate::nodes::connection::Connection;

// TODO: figure out best way to delete node root connections

pub struct NodeRoots {
    connections: Vec<Connection>
}

impl NodeRoots {
    pub fn new(c : Vec<Connection>) -> Self {
        Self {
            connections: c
        }
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Connection> {
        self.connections.iter()
    }

    pub fn count(&self) -> usize {
        self.connections.len()
    }
}

impl Display for NodeRoots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "root connections:")?;
        for (index, connection) in self.connections.iter().enumerate() {
            if index > 0 {
                write!(f, ",")?;
            }
            writeln!(f, "\t{}", connection)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! node_roots {
    ($($connections:expr),+ $(,)?) => {{
        $crate::Connection::new([$( $connections ),+])
    }};
}
