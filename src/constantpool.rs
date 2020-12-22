use std::fmt;
use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::bytecode;
use crate::spec;

pub enum ConstantPoolInfo {
    Class(CONSTANT_Class_info),
    NameAndType(CONSTANT_NameAndType_info),
    Utf8(CONSTANT_Utf8_info),
    Integer(CONSTANT_Integer_info),
    Float(CONSTANT_Float_info),
    MethodRef(CONSTANT_Methodref_info),
    FieldRef(CONSTANT_Fieldref_info),
    InterfaceMethodRef(CONSTANT_InterfaceMethodref_info),
    Dynamic(CONSTANT_Dynamic_info),
    InvokeDynamic(CONSTANT_InvokeDynamic_info),
    String(CONSTANT_String_info),
    MethodHandle(CONSTANT_MethodHandle_info),
    Unknown(String)
}

impl ConstantPoolInfo {
    pub fn class(&self) -> &CONSTANT_Class_info {
        match self {
            ConstantPoolInfo::Class(c) => c,
            _ => panic!("This constant pool entry is not a Class."),
        }
    }

    pub fn utf8(&self) -> &CONSTANT_Utf8_info {
        match self {
            ConstantPoolInfo::Utf8(c) => c,
            _ => panic!("This constant pool entry is not an UTF8."),
        }
    }
}

impl fmt::Display for ConstantPoolInfo {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantPoolInfo::Class(value) => write!(f, "{}", value.name_index),
            ConstantPoolInfo::Utf8(value) => write!(f, "{}", value),
            ConstantPoolInfo::String(value) => write!(f, "{}", value.string_index),
            _ => panic!("Unknown"),
        }
    }
}

// The CONSTANT_Class_info structure is used to represent a class or an interface:
pub struct CONSTANT_Class_info {
    pub name_index : u16,
}

// The CONSTANT_NameAndType_info structure is used to represent a field or method, without indicating which class or interface type it belongs to
pub struct CONSTANT_NameAndType_info {
    pub name_index : u16,
    pub descriptor_index : u16,
}

// The CONSTANT_Utf8_info structure is used to represent constant string values
pub struct CONSTANT_Utf8_info {
    pub bytes : Vec<u8>,
}

impl fmt::Display for CONSTANT_Utf8_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &String::from_utf8_lossy(&self.bytes))
    }
}

// The CONSTANT_Integer_info and CONSTANT_Float_info structures represent 4-byte numeric (int and float) constants:
pub struct CONSTANT_Integer_info {
    pub bytes : u32,
}

pub struct CONSTANT_Float_info {
    pub bytes : u32,
}

// Fields, methods, and interface methods are represented by similar structures:
pub struct CONSTANT_Fieldref_info {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_Methodref_info {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_InterfaceMethodref_info {
    pub class_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_Dynamic_info {
    pub bootstrap_method_attr_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_InvokeDynamic_info {
    pub bootstrap_method_attr_index : u16,
    pub name_and_type_index : u16,
}

pub struct CONSTANT_String_info {
    pub string_index : u16,
}

pub struct CONSTANT_MethodHandle_info {
    pub reference_kind : u8,
    pub reference_index : u16,
}
