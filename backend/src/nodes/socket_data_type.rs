pub enum SocketType
{
    Named,
    Number,
    Select,
    Switch,
    Text
}

pub struct Socket
{
    name: String,
    type_: SocketType,
    slot: u32,
    value: u8,
    parameters: u8,
    permitted: u8,
    connection: u8,
    is_outgoing: bool,
    is_repetition: bool
}

impl Socket
{
    pub fn new(n : String, t : SocketType, io : bool, ir : bool) -> Self
    {
        Self {
            name: n,
            type_: t,
            slot: 0,
            value: 0,
            parameters: 0,
            permitted: 0,
            connection: 0,
            is_outgoing: io,
            is_repetition: ir
        }
    }

    pub fn print(&self) { println!("Socket {}, DIRECTION REPETITION TYPE, SLOT", self.name.as_str()) }

    pub fn set_name(&mut self, new_name : String) { self.name = new_name }
    pub fn get_name(&self) -> &String { &self.name }

    pub fn get_type(&self) -> &SocketType { &self.type_ }

    pub fn set_slot(&self, new_slot : u32) { self.slot = new_slot }
    pub fn get_slot(&self) -> u32 { &self.slot }

    pub fn set_value() { todo!("Standardised value type missing.") }
    pub fn get_value() { todo!("Standardised value type missing.") }

    pub fn get_parameters() { todo!("Different parameter struct required for different socket types.")}

    pub fn is_permitted() { todo!("Compiler type representation missing.") }
    pub fn get_permitted() { todo!("Compiler type representation missing.") }

    pub fn set_connection() { todo!("Connection type missing.") }
    pub fn get_connection() { todo!("Connection type missing.") }

    pub fn is_outgoing(&self) -> bool { self.is_outgoing }

    pub fn is_repetition(&self) -> bool { self.is_repetition }
}
