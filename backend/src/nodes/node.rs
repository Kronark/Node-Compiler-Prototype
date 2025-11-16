use std::{collections::HashMap, fmt::Display, sync::Arc};
use crate::nodes::{instance::Instance, node_roots::NodeRoots, node_type::NodeType, pool::Pool};

// TODO: track what socket slots are used in separate data structure
pub struct Node
{
    pub is_compiler_node: bool,
    pub type_: Arc<NodeType>,
    pub roots: NodeRoots,
    pub id_pool: Pool,
    pub slot_pool: Pool,
    instances: HashMap<u32, Instance>
}

impl Node {
    pub fn new(icn: bool, t: Arc<NodeType>, r: NodeRoots, ip: Pool, sp: Pool, i: HashMap<u32, Instance>) -> Self {
        Self {
            is_compiler_node: icn,
            type_: t,
            roots: r,
            id_pool: ip,
            slot_pool: sp,
            instances: i,
        }
    }

    pub fn add_instance(&mut self, id: u32, instance: Instance) {
        self.instances.insert(id, instance);
    }

    pub fn get_instance(&mut self, id: u32) -> Option<&mut Instance> {
        self.instances.get_mut(&id)
    }

    pub fn remove_instance(&mut self, id: u32) -> Option<Instance> {
        self.instances.remove(&id)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", 
            if self.is_compiler_node { "♦" } else { "♣" },
            self.type_
        )?;

        write!(f, "{}", self.roots)?;

        if self.instances.is_empty() {
            writeln!(f, "no instances")?;
        } else {
            writeln!(f, "instances:")?;

            let mut i: Vec<_> = self.instances.iter().collect();
            i.sort_by_key(|(id, _)| *id);

            for (id, instance) in i {
                writeln!(f, "    • {} ──────────────────────────────", id)?;
                let i_str = instance.to_string();
                for line in i_str.lines() {
                    writeln!(f, "        {}", line)?;
                }
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! node {
    (
        $is_built_in:expr,
        type: $type:expr,
        roots: $roots:expr,
        id_pool: $id_pool:expr,
        slot_pool: $slot_pool:expr
        $(, $id:expr => $instance:expr )* $(,)?
    ) => {{
        $crate::nodes::node::Node::new(
            $is_built_in,
            $type,
            $roots,
            $id_pool,
            $slot_pool,
            [$( ($id, $instance) ),*].into_iter().collect::<std::collections::HashMap<_, _>>()
        )
    }};
}
