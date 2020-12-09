use std::fmt;
use std::str;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use byteorder::{ByteOrder, LittleEndian, BigEndian};


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

impl fmt::Display for ConstantPoolInfo {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantPoolInfo::Class(value) => write!(f, "{}", value),
            ConstantPoolInfo::NameAndType(value) => write!(f, "{}", value),
            ConstantPoolInfo::Utf8(value) => write!(f, "{}", value),
            ConstantPoolInfo::Integer(value) => write!(f, "{}", value),
            ConstantPoolInfo::Float(value) => write!(f, "{}", value),
            ConstantPoolInfo::MethodRef(value) => write!(f, "{}", value),
            ConstantPoolInfo::FieldRef(value) => write!(f, "{}", value),
            ConstantPoolInfo::InterfaceMethodRef(value) => write!(f, "{}", value),
            ConstantPoolInfo::Dynamic(value) => write!(f, "{}", value),
            ConstantPoolInfo::InvokeDynamic(value) => write!(f, "{}", value),
            ConstantPoolInfo::String(value) => write!(f, "{}", value),
            ConstantPoolInfo::MethodHandle(value) => write!(f, "{}", value),
            _ => write!(f, "Unknown\n"),
        }
    }
}

pub struct FieldInfo {
    access_flags : u16,
    name_index : u16,
    descriptor_index : u16,
    attributes_count : u16,
    attributes : Vec<AttributeInfo>
}

pub struct MethodInfo {
    access_flags : u16,
    name_index : u16,
    descriptor_index : u16,
    attributes_count : u16,
    attributes : Vec<AttributeInfo>,
}

pub struct AttributeInfo {
    attribute_name_index : u16,
    attribute_length : u32,
    info : Vec<u8>,
}

// The CONSTANT_Class_info structure is used to represent a class or an interface:
pub struct CONSTANT_Class_info {
    tag : u8,
    name_index : u16,
}

impl fmt::Display for CONSTANT_Class_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss1\n")
    }
}

// The CONSTANT_NameAndType_info structure is used to represent a field or method, without indicating which class or interface type it belongs to
pub struct CONSTANT_NameAndType_info {
    tag : u8,
    name_index : u16,
    descriptor_index : u16,
}

impl fmt::Display for CONSTANT_NameAndType_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss2\n")
    }
}


// The CONSTANT_Utf8_info structure is used to represent constant string values
pub struct CONSTANT_Utf8_info {
    tag : u8,
    length : u16,
    bytes : Vec<u8>,
}

impl fmt::Display for CONSTANT_Utf8_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &String::from_utf8_lossy(&self.bytes))
    }
}

// The CONSTANT_Integer_info and CONSTANT_Float_info structures represent 4-byte numeric (int and float) constants:
pub struct CONSTANT_Integer_info {
    tag : u8,
    bytes : u32,
}

impl fmt::Display for CONSTANT_Integer_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss3\n")
    }
}


pub struct CONSTANT_Float_info {
    tag : u8,
    bytes : u32,
}

impl fmt::Display for CONSTANT_Float_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss4\n")
    }
}

// Fields, methods, and interface methods are represented by similar structures:
pub struct CONSTANT_Fieldref_info {
    tag : u8,
    class_index : u16,
    name_and_type_index : u16,
}

impl fmt::Display for CONSTANT_Fieldref_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss5\n")
    }
}

pub struct CONSTANT_Methodref_info {
    tag : u8,
    class_index : u16,
    name_and_type_index : u16,
}

impl fmt::Display for CONSTANT_Methodref_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss6\n")
    }
}

pub struct CONSTANT_InterfaceMethodref_info {
    tag : u8,
    class_index : u16,
    name_and_type_index : u16,
}

impl fmt::Display for CONSTANT_InterfaceMethodref_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss7\n")
    }
}

pub struct CONSTANT_Dynamic_info {
    tag : u8,
    bootstrap_method_attr_index : u16,
    name_and_type_index : u16,
}

impl fmt::Display for CONSTANT_Dynamic_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss8\n")
    }
}

pub struct CONSTANT_InvokeDynamic_info {
    tag : u8,
    bootstrap_method_attr_index : u16,
    name_and_type_index : u16,
}

impl fmt::Display for CONSTANT_InvokeDynamic_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss9\n")
    }
}


pub struct CONSTANT_String_info {
    tag : u8,
    string_index : u16,
}

impl fmt::Display for CONSTANT_String_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss10\n")
    }
}

pub struct CONSTANT_MethodHandle_info {
    tag : u8,
    reference_kind : u8,
    reference_index : u16,
}

impl fmt::Display for CONSTANT_MethodHandle_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Classssss11\n")
    }
}


pub struct ClassFile {
    pub magic : u32,
    pub minor_version : u16,
    pub major_version : u16,
    pub constant_pool_count : u16,
    pub constant_pool : Vec<ConstantPoolInfo>,
    pub access_flags : u16,
    pub this_class : u16,
    pub super_class : u16,
    pub interfaces_count : u16,
    pub interfaces : Vec<u16>,
    pub fields_count : u16,
    pub fields : Vec<FieldInfo>,
    pub methods_count : u16,
    pub methods : Vec<MethodInfo>,
    pub attributes_count : u16,
    pub attributes : Vec<AttributeInfo>
}

impl ClassFile {
    pub fn load<T: Read>(reader: &mut T) -> ClassFile {
        let mut load_part = |size| {
            let mut buf = Vec::with_capacity(size);

            let mut part_reader = reader.take(size as u64);

            part_reader.read_to_end(&mut buf).unwrap();

            buf
        };

        let magic = BigEndian::read_u32(&load_part(4));
        let miv = BigEndian::read_u16(&load_part(2));
        let mav = BigEndian::read_u16(&load_part(2));
        let cp_size = BigEndian::read_u16(&load_part(2));
        let mut constant_pool = Vec::with_capacity(cp_size as usize);

        for i in 1..cp_size {
            let tag = load_part(1)[0] as u8;

            let constant_pool_entry = match tag {
                9=> ConstantPoolInfo::FieldRef( CONSTANT_Fieldref_info { tag: tag, class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                10 => ConstantPoolInfo::MethodRef( CONSTANT_Methodref_info { tag: tag, class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                11 => ConstantPoolInfo::InterfaceMethodRef( CONSTANT_InterfaceMethodref_info { tag: tag, class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                7  => ConstantPoolInfo::Class( CONSTANT_Class_info { tag: tag, name_index : BigEndian::read_u16(&load_part(2))  } ),
                12  => ConstantPoolInfo::NameAndType( CONSTANT_NameAndType_info { tag: tag, name_index : BigEndian::read_u16(&load_part(2)), descriptor_index : BigEndian::read_u16(&load_part(2)) } ),
                1  => { let length = BigEndian::read_u16(&load_part(2)); ConstantPoolInfo::Utf8( CONSTANT_Utf8_info { tag: tag, length : length, bytes : load_part(length as usize) } ) } ,
                3  => ConstantPoolInfo::Integer( CONSTANT_Integer_info { tag: tag, bytes : BigEndian::read_u32(&load_part(4))  } ),
                4  => ConstantPoolInfo::Float( CONSTANT_Float_info { tag: tag, bytes : BigEndian::read_u32(&load_part(4))  } ),
                17  => ConstantPoolInfo::Dynamic( CONSTANT_Dynamic_info { tag: tag, bootstrap_method_attr_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2)) } ),
                18  => ConstantPoolInfo::InvokeDynamic( CONSTANT_InvokeDynamic_info { tag: tag, bootstrap_method_attr_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2)) } ),
                8  => ConstantPoolInfo::String( CONSTANT_String_info { tag: tag, string_index : BigEndian::read_u16(&load_part(2))  } ),
                15  => ConstantPoolInfo::MethodHandle( CONSTANT_MethodHandle_info { tag: tag, reference_kind : load_part(1)[0] as u8, reference_index : BigEndian::read_u16(&load_part(2)) } ),
                _  => ConstantPoolInfo::Unknown( "Unknown".to_string() ),
            };

            constant_pool.push( constant_pool_entry );
        }

        let access_flags = BigEndian::read_u16(&load_part(2));
        let this_class = BigEndian::read_u16(&load_part(2));
        let super_class = BigEndian::read_u16(&load_part(2));
        let interfaces_count = BigEndian::read_u16(&load_part(2));
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);

        for i in 0..interfaces_count {
            let interface_info = BigEndian::read_u16(&load_part(2));
            interfaces.push(interface_info);
        }

        let fields_count = BigEndian::read_u16(&load_part(2));
        let mut fields = Vec::with_capacity(fields_count as usize);

        for i in 0..fields_count {
            let access_flags = BigEndian::read_u16(&load_part(2));
            let name_index = BigEndian::read_u16(&load_part(2));
            let descriptor_index = BigEndian::read_u16(&load_part(2));
            let attributes_count = BigEndian::read_u16(&load_part(2));
            let mut attributes = Vec::with_capacity(attributes_count as usize);

            for j in 0..attributes_count {
                let attribute_name_index = BigEndian::read_u16(&load_part(2));
                let attribute_length = BigEndian::read_u32(&load_part(4));
                let info = load_part(attribute_length as usize);

                attributes.push(
                    AttributeInfo {
                        attribute_name_index : attribute_name_index,
                        attribute_length : attribute_length,
                        info : info,
                    }
                );
            }

            fields.push(
                FieldInfo {
                    access_flags : access_flags,
                    name_index : name_index,
                    descriptor_index : descriptor_index,
                    attributes_count : attributes_count,
                    attributes : attributes,
                }
            );
        }

        let methods_count = BigEndian::read_u16(&load_part(2));
        let mut methods = Vec::with_capacity(methods_count as usize);

        for i in 0..methods_count {
            let access_flags = BigEndian::read_u16(&load_part(2));
            let name_index = BigEndian::read_u16(&load_part(2));
            let descriptor_index = BigEndian::read_u16(&load_part(2));
            let attributes_count = BigEndian::read_u16(&load_part(2));
            let mut attributes = Vec::with_capacity(attributes_count as usize);

            for j in 0..attributes_count {
                let attribute_name_index = BigEndian::read_u16(&load_part(2));
                let attribute_length = BigEndian::read_u32(&load_part(4));
                let info = load_part(attribute_length as usize);

                attributes.push(
                    AttributeInfo {
                        attribute_name_index : attribute_name_index,
                        attribute_length : attribute_length,
                        info : info,
                    }
                );
            }

            methods.push(
                MethodInfo {
                    access_flags : access_flags,
                    name_index : name_index,
                    descriptor_index : descriptor_index,
                    attributes_count : attributes_count,
                    attributes : attributes,
                }
            );
        }

        let attributes_count = BigEndian::read_u16(&load_part(2));
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for j in 0..attributes_count {
            let attribute_name_index = BigEndian::read_u16(&load_part(2));
            let attribute_length = BigEndian::read_u32(&load_part(4));
            let info = load_part(attribute_length as usize);

            attributes.push(
                AttributeInfo {
                    attribute_name_index : attribute_name_index,
                    attribute_length : attribute_length,
                    info : info,
                }
            );
        }

        ClassFile {
            magic: magic,
            minor_version: miv,
            major_version: mav,
            constant_pool_count: cp_size,
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces_count: interfaces_count,
            interfaces : interfaces,
            fields_count: fields_count,
            fields : fields,
            methods_count: methods_count,
            methods : methods,
            attributes_count: attributes_count,
            attributes : attributes,
        }
    }
}

impl ClassFile {
    pub fn print(self : Self) {
        println!("Magic number: 0x{:X?}", self.magic);
        println!("Version: {}.{}", self.major_version, self.minor_version);

        println!("# Constant Pool: {}", self.constant_pool.len());
        for constant_pool_entry in &self.constant_pool {
            match constant_pool_entry {
                ConstantPoolInfo::Class(value) => { 
                    println!("\tClass:");
                    println!("\t\tname_index: {}", value.name_index);
                },
                ConstantPoolInfo::NameAndType(value) => {
                    println!("\tNameAndtype:");
                    println!("\t\tname_index: {} \t # {}", value.name_index, self.constant_pool[value.name_index as usize]);
                    println!("\t\tdescriptor_index: {} \t # {}", value.descriptor_index, self.constant_pool[value.descriptor_index as usize]);
                },
                ConstantPoolInfo::Utf8(value) => {
                    println!("\tUTF-8:");
                    println!("\t\tlength: {}", value.length);
                    println!("\t\tbytes: {}", value);
                },
                ConstantPoolInfo::Integer(value) => {
                    println!("\tInteger:");
                    println!("\t\tbytes: {}", value.bytes);
                },
                ConstantPoolInfo::Float(value) => {
                    println!("\tFloat:");
                    println!("\t\tbytes: {}", value.bytes);
                },
                ConstantPoolInfo::MethodRef(value) => {
                    println!("\tMethodRef:");
                    println!("\t\tclass_index: {}", value.class_index);
                    println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                },
                ConstantPoolInfo::FieldRef(value) => {
                    println!("\tFieldRef:");
                    println!("\t\tclass_index: {}", value.class_index);
                    println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                },
                ConstantPoolInfo::InterfaceMethodRef(value) => {
                    println!("\tInterfaceMethodRef:");
                    println!("\t\tclass_index: {}", value.class_index);
                    println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                },
                ConstantPoolInfo::Dynamic(value) => {
                    println!("\tDynamic:");
                    println!("\t\tbootstrap_method_attr_index: {}", value.bootstrap_method_attr_index);
                    println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                },
                ConstantPoolInfo::InvokeDynamic(value) => {
                    println!("\tInvokeDynamic:");
                    println!("\t\tbootstrap_method_attr_index: {}", value.bootstrap_method_attr_index);
                    println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                },
                ConstantPoolInfo::String(value) => {
                    println!("\tString:");
                    println!("\t\tstring_index: {} \t # {}", value.string_index, self.constant_pool[value.string_index as usize]);
                },
                ConstantPoolInfo::MethodHandle(value) => {
                    println!("\tMethodHandle:");
                    println!("\t\treference_kind: {}", value.reference_kind);
                    println!("\t\treference_index: {}", value.reference_index);
                },
                _ => println!("Unknown"),
            }
        }


        println!("# Fields: {}", self.fields_count); 
        println!("# Interfaces: {}", self.interfaces_count);
        println!("# Methods: {}", self.methods_count);
        println!("# Class Attributes: {}", self.attributes_count);

    }
}