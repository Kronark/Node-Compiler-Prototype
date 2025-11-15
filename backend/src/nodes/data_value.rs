use std::fmt::Display;
use std::ops::{ Deref, DerefMut };
use std::iter::IntoIterator;

// OPTIMISE: implement interning for values to prevent duplicate results

pub struct DataValue {
    data: Vec<u8>
}

impl<const N:usize> From<&[u8;N]> for DataValue {
    fn from(value: &[u8;N]) -> Self {
        Self {
            data:value.to_vec()
        }
    }
}

impl<const N:usize> From<[u8;N]> for DataValue {
    fn from(value: [u8;N]) -> Self {
        Self {
            data:value.to_vec()
        }
    }
}

impl From<&[u8]> for DataValue {
    fn from(value: &[u8]) -> Self {
        Self {
            data:value.to_vec()
        }
    }
}

impl From<&str> for DataValue {
    fn from(s: &str) -> Self {
        Self {
            data: s.as_bytes().to_vec(),
        }
    }
}

impl From<Vec<u8>> for DataValue {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl DataValue {
    pub fn new<T: Into<DataValue>>(data: T) -> Self {
        data.into()
    }

    pub fn set<T: Into<DataValue>>(&mut self, data: T) {
        *self = data.into();
    }
}

impl AsRef<[u8]> for DataValue {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl IntoIterator for DataValue {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Deref for DataValue {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for DataValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, byte) in self.data.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! data_value {
    ($s:literal) => {{
        const fn is_hex_char(c: u8) -> bool {
            matches!(c,
                b'0'..=b'9'
                | b'a'..=b'f'
                | b'A'..=b'F'
            )
        }

        let s_bytes = $s.as_bytes();
        let mut looks_hex = true;
        let mut i = 0;
        while i < s_bytes.len() {
            let c = s_bytes[i];
            if c == b' ' || c == b'_' {
                i += 1;
                continue;
            }
            if !is_hex_char(c) {
                looks_hex = false;
                break;
            }
            i += 1;
        }

        if looks_hex {
            let cleaned: String = $s
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '_')
                .collect();

            assert!(
                cleaned.len() % 2 == 0,
                "hex string must have an even number of digits"
            );

            let bytes: Vec<u8> = cleaned
                .as_bytes()
                .chunks(2)
                .map(|p| {
                    let s = core::str::from_utf8(p).unwrap();
                    u8::from_str_radix(s, 16).unwrap()
                })
                .collect();

            $crate::DataValue::from(bytes)
        } else {
            $crate::DataValue::from($s)
        }
    }};

    ($($byte:expr),+ $(,)?) => {{
        let mut v = Vec::<u8>::new();
        $(
            v.push($byte as u8);
        )+
        $crate::DataValue::from(v)
    }};

    ($value:expr) => {
        $crate::DataValue::new($value)
    };
}
