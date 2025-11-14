use core::fmt;
use std::{collections::HashMap, fmt::Display, sync::{Arc, Mutex, Weak, OnceLock}};

// ========== Globals ==========

static IDENTIFIER_COMPONENT_REGISTRY: OnceLock<Arc<IdentifierComponentRegistry>> = OnceLock::new();

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

// ========== Identifier Component Registry ==========

#[derive(Debug, Default)]
pub struct IdentifierComponentRegistry {
    registry: Mutex<HashMap<String, Weak<str>>>
}

impl IdentifierComponentRegistry {
    pub fn new() -> Self {
        Self { registry: Mutex::new(HashMap::new()) }
    }

    pub fn intern(&self, s: &str) -> Result<Arc<str>, IdentifierComponentError> {
        if s.is_empty() {
            return Err(IdentifierComponentError::Empty)
        }

        if let Some(c) = s.chars().find(|&c| !matches!(c, 'a'..='z' | '-')) {
            return Err(IdentifierComponentError::InvalidCharacter(c))
        }

        let mut registry = self.registry.lock().unwrap();

        if let Some(existing_weak) = registry.get(s) {
            if let Some(existing_arc) = existing_weak.upgrade() {
                return Ok(existing_arc);
            }
        }

        let arc_str: Arc<str> = Arc::from(s);
        registry.insert(s.to_string(), Arc::downgrade(&arc_str));
        Ok(arc_str)
    }
}

pub fn identifier_component_registry() -> &'static Arc<IdentifierComponentRegistry> {
    IDENTIFIER_COMPONENT_REGISTRY.get_or_init(|| Arc::new(IdentifierComponentRegistry::new()))
}

// ========== Identifier Component ==========

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct IdentifierComponent {
    inner: Arc<str>
}

impl IdentifierComponent {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, IdentifierComponentError> {
        Ok(Self {
            inner: identifier_component_registry().intern(s.as_ref())?
        })
    }

    pub fn data(&self) -> &str {
        &self.inner
    }
}

impl Display for IdentifierComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[macro_export]
macro_rules! identifier_component {
    ( $x:expr ) => {{
        $crate::IdentifierComponent::new( $x ).expect("Invalid character(s)!")
    }};
}

// ========== Identifier ==========

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
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
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.components.iter().map(|c| c.data()).collect::<Vec<_>>().join(":"))
    }
}

#[macro_export]
macro_rules! identifier {
    ( $( $x:expr ),+ $(,)? ) => {{
        $crate::Identifier::new([ $( $x ),+ ]).expect("Invalid identifier component(s)!")
    }};
}
