use utils::bkdr_hash::{Hash, HashWith};

use super::*;

const FIELD_NAME_MAX_LEN: usize = 512;

pub type FieldID = u64;
pub type FieldName = Vec<u8>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FieldInfo {
    pub id: FieldID,
    pub name: FieldName,
    pub value_type: ValueType,
}

#[derive(Copy, Clone)]
pub struct AbstractFieldInfo {
    pub id: FieldID,
    pub value_type: ValueType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    Unknown,
    Float,
    Integer,
    Unsigned,
    Boolean,
    String,
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Float,
            1 => Self::Integer,
            2 => Self::Boolean,
            3 => Self::String,
            4 => Self::Unsigned,
            _ => Self::Unknown,
        }
    }
}

impl From<ValueType> for u8 {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Float => 0,
            ValueType::Integer => 1,
            ValueType::Boolean => 2,
            ValueType::String => 3,
            ValueType::Unsigned => 4,
            ValueType::Unknown => 5,
        }
    }
}

impl From<protos::models::FieldType> for ValueType {
    fn from(t: protos::models::FieldType) -> Self {
        match t {
            protos::models::FieldType::Float => ValueType::Float,
            protos::models::FieldType::Integer => ValueType::Integer,
            protos::models::FieldType::Unsigned => ValueType::Unsigned,
            protos::models::FieldType::Boolean => ValueType::Boolean,
            protos::models::FieldType::String => ValueType::String,
            _ => ValueType::Unknown,
        }
    }
}

impl FieldInfo {
    pub fn new(id: u64, name: Vec<u8>, value_type: ValueType) -> Self {
        FieldInfo { id, name, value_type }
    }

    pub fn to_abstract(&self) -> AbstractFieldInfo {
        AbstractFieldInfo { id: self.id, value_type: self.value_type }
    }

    pub fn cal_fid(name: &FieldName, sid: SeriesID) -> FieldID {
        let mut hash = Hash::from(sid);
        hash.hash_with(name).number()
    }
    pub fn update_id(&mut self, series_id: SeriesID) {
        self.id = Self::cal_fid(&self.name, series_id);
    }

    pub fn format_check(&self) -> Result<(), String> {
        if self.name.len() > FIELD_NAME_MAX_LEN {
            return Err(String::from("TagKey exceeds the FIELD_NAME_MAX_LEN"));
        }
        Ok(())
    }
    pub fn filed_id(&self) -> FieldID {
        self.id
    }
}

pub trait FieldInfoFromParts<T1, T2> {
    fn from_parts(name: T1, value_type: T2) -> Self;
}

impl FieldInfoFromParts<FieldName, ValueType> for FieldInfo {
    fn from_parts(name: FieldName, value_type: ValueType) -> Self {
        FieldInfo { id: 0, name, value_type }
    }
}

impl FieldInfoFromParts<&str, ValueType> for FieldInfo {
    fn from_parts(name: &str, value_type: ValueType) -> Self {
        FieldInfo { id: 0, name: name.as_bytes().to_vec(), value_type }
    }
}

#[cfg(test)]
mod tests_field_info {
    use crate::{FieldInfo, FieldInfoFromParts, ValueType};

    #[test]
    fn test_field_info_format_check() {
        let field_info = FieldInfo::from_parts(Vec::from("hello"), ValueType::Integer);
        field_info.format_check().unwrap();
    }
}
