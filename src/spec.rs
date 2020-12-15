use std::fmt;
use std::str;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::borrow::Borrow;
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};
use std::io::Cursor;

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
    fn class(&self) -> &CONSTANT_Class_info {
        match self {
            ConstantPoolInfo::Class(c) => c,
            _ => panic!("This constant pool entry is not a Class."),
        }
    }

    fn utf8(&self) -> &CONSTANT_Utf8_info {
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
            ConstantPoolInfo::MethodHandle(value) => write!(f, "{}", value.reference_kind),
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

// The CONSTANT_Class_info structure is used to represent a class or an interface:
pub struct CONSTANT_Class_info {
    tag : u8,
    name_index : u16,
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

pub struct AttributeInfo {
    pub name : String,

    pub source_file : Option<String>,
    pub bootstrap_methods : Option<Vec<BootstrapMethods_attribute>>,
    pub inner_classes : Option<Vec<InnerClasses_attribute>>,
}

pub struct BootstrapMethods_attribute {
    pub bootstrap_method_ref : u16,
    pub bootstrap_arguments : Vec<u16>,
}

pub struct InnerClasses_attribute {
    pub inner_class_info : String,
    pub outer_class_info : Option<String>,
    pub inner_name : Option<String>,
    pub inner_class_access_flags : u16,
}

impl AttributeInfo {
    pub fn print_info(&self) {
        println!("\t{}", self.name);

        if self.source_file.is_some() {
            let x = self.source_file.as_deref().unwrap();
            println!("\t\t{} {}", "Name:", x);
        }

        if self.inner_classes.is_some() {
            for inner_class in self.inner_classes.as_deref().unwrap() {
                println!("\t\t{} {}", "Inner Class:", inner_class.inner_class_info);
                println!("\t\t{} {}", "Outer Class:", if inner_class.outer_class_info.is_some() { inner_class.outer_class_info.as_deref().unwrap() } else { "" } );
                println!("\t\t{} {}", "Inner Name:", if inner_class.inner_name.is_some() { inner_class.inner_name.as_deref().unwrap() } else { "" } );
                println!("\t\t{} {}", "Flags:", ClassFile::flags_names(inner_class.inner_class_access_flags));
            }
        }

        if self.bootstrap_methods.is_some() {
            for bootstrap_method in self.bootstrap_methods.as_deref().unwrap() {
                println!("\t\t{} {}", "Bootstrap Method Index:", bootstrap_method.bootstrap_method_ref);
                println!("\t\t{} {:?}", "Bootstrap Method Arguments:", bootstrap_method.bootstrap_arguments);
            }
        }
    }

    pub fn build_attribute_info(constant_pool : &Vec<ConstantPoolInfo>, attribute_name_index : u16, info : Vec<u8>) -> AttributeInfo {
        let mut cursor = Cursor::new(info);
        let constant_pool_attr_name_entry = constant_pool[attribute_name_index as usize].utf8();
        let name = String::from_utf8_lossy(&constant_pool_attr_name_entry.bytes);
        let mut source_file = None;
        let mut inner_classes = None;
        let mut bootstrap_methods = None;

        if name == "SourceFile" {
            let sourcefile_index = cursor.read_u16::<BigEndian>().unwrap();
            let name2 = String::from_utf8_lossy(&constant_pool[sourcefile_index as usize].utf8().bytes) ;
            source_file = Some(name2.into_owned());
        }
        else if name == "InnerClasses" {
            let number_of_classes = cursor.read_u16::<BigEndian>().unwrap();
            let mut classes = Vec::with_capacity(number_of_classes as usize);

            for i in 0..number_of_classes {
                let inner_class_idx = cursor.read_u16::<BigEndian>().unwrap();
                let inner_class_name_idx = constant_pool[inner_class_idx as usize].class().name_index;
                let inner_class_info = String::from_utf8_lossy(&constant_pool[inner_class_name_idx as usize].utf8().bytes) ;

                let outer_class_idx = cursor.read_u16::<BigEndian>().unwrap();
                let outer_class_info =  if outer_class_idx != 0 
                                            { 
                                                let outer_class_info_class_idx = constant_pool[outer_class_idx as usize].class().name_index;
                                                Some( String::from_utf8_lossy(&constant_pool[outer_class_info_class_idx as usize].utf8().bytes).into_owned() )
                                            }
                                        else
                                            { None };

                let inner_name_idx = cursor.read_u16::<BigEndian>().unwrap();
                let inner_name =    if inner_name_idx != 0 
                                        { 
                                            Some( String::from_utf8_lossy(&constant_pool[inner_name_idx as usize].utf8().bytes).into_owned() )
                                        }
                                    else
                                        { None };

                classes.push(
                    InnerClasses_attribute {
                        inner_class_info : inner_class_info.to_string(),
                        outer_class_info : outer_class_info,
                        inner_name : inner_name,
                        inner_class_access_flags : cursor.read_u16::<BigEndian>().unwrap()
                    }
                );
            }

            inner_classes = Some(classes);
        }
        else if name == "BootstrapMethods" {
            let number_of_bootstrap_methods = cursor.read_u16::<BigEndian>().unwrap();
            let mut bs_methods = Vec::with_capacity(number_of_bootstrap_methods as usize);

            for i in 0..number_of_bootstrap_methods {
                let bootstrap_method_ref = cursor.read_u16::<BigEndian>().unwrap();
                let num_bootstrap_arguments = cursor.read_u16::<BigEndian>().unwrap();
                let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);

                for i in 0..num_bootstrap_arguments {
                    bootstrap_arguments.push( cursor.read_u16::<BigEndian>().unwrap() );
                }

                bs_methods.push(
                    BootstrapMethods_attribute {
                        bootstrap_method_ref,
                        bootstrap_arguments,
                    }
                );
            }

            bootstrap_methods = Some(bs_methods);
        }
        else if name == "Code" {
            println!("Code attribute");
        }
        else {
            panic!("Unknown attribute name: {}", name);
        }

        AttributeInfo {
            name : name.to_string(),
            source_file,
            bootstrap_methods,
            inner_classes,
        }
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
        let mut constant_pool = Vec::with_capacity((cp_size + 1) as usize);

        constant_pool.push(
            ConstantPoolInfo::Unknown("Padding".to_string())
        );

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
                    AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
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
                    AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
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
                AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
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


    fn flags_names(flags : u16) -> String {
        let mut names = String::new();

        if (flags & 0x0001) == 0x0001 { names.push_str(",ACC_PUBLIC") }
        if (flags & 0x0002) == 0x0002 { names.push_str(",ACC_PRIVATE") }
        if (flags & 0x0004) == 0x0004 { names.push_str(",ACC_PROTECTED") }
        if (flags & 0x0008) == 0x0008 { names.push_str(",ACC_STATIC") }
        if (flags & 0x0010) == 0x0010 { names.push_str(",ACC_FINAL") }
        if (flags & 0x0020) == 0x0020 { names.push_str(",ACC_SUPER") }
        if (flags & 0x0200) == 0x0200 { names.push_str(",ACC_INTERFACE") }
        if (flags & 0x0400) == 0x0400 { names.push_str(",ACC_ABSTRACT") }
        if (flags & 0x0040) == 0x0040 { names.push_str(",ACC_VOLATILE") }
        if (flags & 0x0080) == 0x0080 { names.push_str(",ACC_TRANSIENT") }
        if (flags & 0x1000) == 0x1000 { names.push_str(",ACC_SYNTHETIC") }
        if (flags & 0x2000) == 0x2000 { names.push_str(",ACC_ANNOTATION") }
        if (flags & 0x4000) == 0x4000 { names.push_str(",ACC_ENUM") }
        if (flags & 0x8000) == 0x8000 { names.push_str(",ACC_MODULE") }


        if (flags & 0x0020) == 0x0020 { names.push_str(",ACC_SYNCHRONIZED") } // duplicated
        if (flags & 0x0040) == 0x0040 { names.push_str(",ACC_BRIDGE") }     // duplicated
        if (flags & 0x0080) == 0x0080 { names.push_str(",ACC_VARARGS") }// duplicated
        if (flags & 0x0100) == 0x0100 { names.push_str(",ACC_NATIVE") }
        if (flags & 0x0800) == 0x0800 { names.push_str(",ACC_STRICT") }

        let x: &[_] = &[','];
        return names.trim_matches(x).to_string();
    }

    pub fn print(self : Self, attributes : bool, constant_pool : bool, interfaces : bool, fields : bool, methods : bool) {
        let mut class = self.constant_pool[self.this_class as usize].class();
        let mut super_class = self.constant_pool[self.super_class as usize].class();

        println!("{:<30} 0x{:X?}", "Magic number:", self.magic);
        println!("{:<30} {}.{}", "Version:", self.major_version, self.minor_version);
        println!("{:<30} {}", "Access Flags:", ClassFile::flags_names(self.access_flags));
        println!("{:<30} {}", "This Class:", self.constant_pool[class.name_index as usize]);
        println!("{:<30} {}", "Super Class:", self.constant_pool[super_class.name_index as usize]);

        if attributes {
            println!("Class Attributes:");

            for attribute in &self.attributes {
                attribute.print_info();
            }
        }

        if interfaces {
            println!("# Interfaces: {}", self.interfaces_count);

            for interface_entry in &self.interfaces {
                println!("\t {}", self.constant_pool[*interface_entry as usize]);
            }
        }

        if constant_pool {
            println!("# Constant Pool: {:X?}", self.constant_pool.len());

            let mut i = 0;
            for constant_pool_entry in &self.constant_pool {
                println!("cp[{}] = ", i);
                i = i + 1;
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
        }

        if fields {
            println!("# Fields: {}", self.fields_count);

            for field_entry in &self.fields {
                print!("\t Access flags: {}, Name: {}, descritor_index: {}", 
                    ClassFile::flags_names(field_entry.access_flags),
                    self.constant_pool[field_entry.name_index as usize],
                    field_entry.descriptor_index,
                );
                for attribute in &field_entry.attributes {
                    attribute.print_info();
                }
                println!();
            }
        }

        if methods {
            println!("# Methods: {}", self.methods_count);

            for method in &self.methods {
                println!();
                println!("\tMethod name: {} {}", self.constant_pool[method.name_index as usize], self.constant_pool[method.descriptor_index as usize]);
                println!("\tAccess flags: {}", ClassFile::flags_names(method.access_flags));

                if method.attributes_count > 0 {
                    println!("\tAttributes: ");

                    for attribute in &method.attributes {
                        attribute.print_info();
                    }
                }
            }
        }
    }

}