pub struct Connection
{
    instance_id: u32,
    socket_slot: u32,
    type_: String
}

impl Connection
{
    pub fn new(i : u32, s : u32, t : String) -> Self
    {
        Self {
            instance_id: i,
            socket_slot: s,
            type_: t
        }
    }

    pub fn print(&self) { println!("Connection into {}, slot {}, type {}", self.instance_id, self.socket_slot, self.type_.as_str()) }

    pub fn set_instance_id(&mut self, new_instance : u32) { self.instance_id = new_instance }
    pub fn get_instance_id(&self) -> u32 { self.instance_id }

    pub fn set_socket_slot(&mut self, new_socket_slot : u32) { self.socket_slot = new_socket_slot }
    pub fn get_socket_slot(&self) -> u32 { self.socket_slot }

    pub fn set_type(&mut self, new_type : String) { self.type_ = new_type }
    pub fn get_type(&self) -> &String { &self.type_ }
}
