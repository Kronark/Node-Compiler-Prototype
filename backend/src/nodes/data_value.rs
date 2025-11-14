use std::ops::{ Deref, DerefMut };
use std::iter::IntoIterator;

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

impl From<u32> for DataValue {
    fn from(num: u32) -> Self {
        Self {
            data: num.to_ne_bytes().to_vec(),
        }
    }
}

impl From<f32> for DataValue {
    fn from(num: f32) -> Self {
        Self {
            data: num.to_ne_bytes().to_vec(),
        }
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
