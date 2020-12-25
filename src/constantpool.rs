use std::fmt;
use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::bytecode;
use crate::spec;

pub enum ConstantPoolEntry {
    Class(String),
    Utf8(String),
    String(String),
    Unknown(String),
    NameAndType(CONSTANT_NameAndType),
    Integer(CONSTANT_Integer),
    Float(CONSTANT_Float),
    MethodRef(CONSTANT_Methodref),
    FieldRef(CONSTANT_Fieldref),
    InterfaceMethodRef(CONSTANT_InterfaceMethodref),
    Dynamic(CONSTANT_Dynamic),
    InvokeDynamic(CONSTANT_InvokeDynamic),
    MethodHandle(CONSTANT_MethodHandle),
}

impl ConstantPoolEntry {
    pub fn class(&self) -> String {
        match self {
            ConstantPoolEntry::Class(c) => c.to_string(),
            _ => panic!("This constant pool entry is not a Class."),
        }
    }

    pub fn utf8(&self) -> String {
        match self {
            ConstantPoolEntry::Utf8(c) => c.to_string(),
            _ => panic!("This constant pool entry is not an UTF8."),
        }
    }
}

pub struct CONSTANT_NameAndType {
    pub name_index : u16,
    pub descriptor_index : u16,
}

pub struct CONSTANT_Integer {
    pub bytes : u32,
}

pub struct CONSTANT_Float {
    pub bytes : u32,
}

pub struct CONSTANT_Fieldref {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_Methodref {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_InterfaceMethodref {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_Dynamic {
    pub bootstrap_method_attr_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_InvokeDynamic {
    pub bootstrap_method_attr_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_MethodHandle {
    pub reference_kind : u8,
    pub reference_index : u16,
}
