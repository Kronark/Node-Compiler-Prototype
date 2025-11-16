use core::fmt;
use std::{fmt::Display, sync::OnceLock};
use parking_lot::{RwLock};
use lasso::{Rodeo, Spur};

// ========== Globals ==========

static IDENTIFIER_COMPONENT_INTERNER: OnceLock<IdentifierComponentInterner> = OnceLock::new();

// ========== Errors ==========

#[derive(Debug)]
pub enum IdentifierComponentError {
    Empty,
    InvalidCharacter(char)
}

#[derive(Debug)]
pub enum IdentifierError {
    Empty,
    InvalidComponent(IdentifierComponentError)
}

// ========== Identifier Component Interner ==========

pub type IdentifierComponentReference = Spur;

pub struct IdentifierComponentInterner {
    data: RwLock<Rodeo>
}

impl IdentifierComponentInterner {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(Rodeo::default())
        }
    }

    pub fn intern(&self, datum: &str) -> Result<IdentifierComponentReference, IdentifierComponentError> {
        if datum.is_empty() {
            return Err(IdentifierComponentError::Empty)
        }

        // bytes is faster here since it skips decoding utf8
        if let Some(&byte) = datum.as_bytes().iter().find(|&&byte|
            !(b'a'..=b'z').contains(&byte) &&
            !(b'0'..=b'9').contains(&byte) &&
            byte != b'-'
        ) {
            return Err(IdentifierComponentError::InvalidCharacter(byte as char))
        }

        Ok(self.data.write().get_or_intern(datum))
    }

    pub fn resolve(&'static self, symbol: IdentifierComponentReference) -> &'static str {
        let data = self.data.read();
        let datum: &str = data.resolve(&symbol);

        // invariant of Interner: Once a string is interned it won't be modified
        unsafe { &*(datum as *const str) }
    }
}

pub fn identifier_component_interner() -> &'static IdentifierComponentInterner {
    IDENTIFIER_COMPONENT_INTERNER.get_or_init(|| IdentifierComponentInterner::new())
}

// ========== Identifier Component ==========

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdentifierComponent {
    data: IdentifierComponentReference
}

impl IdentifierComponent {
    pub fn new(data: &str) -> Result<Self, IdentifierComponentError> {
        Ok(Self {
            data: identifier_component_interner().intern(data)?
        })
    }

    pub fn data(&self) -> &'static str {
        identifier_component_interner().resolve(self.data)
    }
}

impl Display for IdentifierComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data())
    }
}

#[macro_export]
macro_rules! identifier_component {
    ($string:expr) => {{
        $crate::IdentifierComponent::new($string).expect("Invalid character(s)!")
    }};
}

// ========== Identifier ==========

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    components: Vec<IdentifierComponent>
}

impl Identifier {
    pub fn new<I, T>(parts: I) -> Result<Self, IdentifierError>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>
    {
        let mut components = Vec::new();
        for part in parts {
            let component = IdentifierComponent::new(part.as_ref()).map_err(IdentifierError::InvalidComponent)?;
            components.push(component);
        }

        if components.is_empty() {
            return Err(IdentifierError::Empty);
        }

        Ok(Self { components })
    }

    pub fn as_str(&self) -> String {
        self.components.iter().map(|c| c.data()).collect::<Vec<_>>().join(":")
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.components.iter().map(|c| c.data()).collect::<Vec<_>>().join(":"))
    }
}

#[macro_export]
macro_rules! identifier {
    ($($strings:expr),+ $(,)?) => {{
        $crate::nodes::identifier::Identifier::new([$( $strings ),+]).expect("Invalid identifier component(s)!")
    }};
}
