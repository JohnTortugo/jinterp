use std::fmt;
use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::utils;
use crate::bytecode;
use crate::attributes;
use crate::constantpool;

#[derive(Debug)]
pub struct ClassDesc <'a> {
    pub magic : u32,
    pub name : String,
    pub minor_version : u16,
    pub major_version : u16,
    pub access_flags : u16,
    pub fields : Vec<Field>,
    pub methods : Vec<Method>,
    pub interfaces : Vec<u16>,
    pub parent_class_name : String,
    pub parent_class : Option<&'a ClassDesc<'a>>,
    pub attributes : Vec<attributes::AttributeInfo>,
    pub constant_pool : Vec<constantpool::ConstantPoolEntry>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags : u16,
    pub name : String,
    pub descriptor_index : u16,
    pub attributes : Vec<attributes::AttributeInfo>,
}

#[derive(Debug)]
pub struct Field {
    pub access_flags : u16,
    pub name : String,
    pub descriptor : String,
    pub value : Option<u64>,
    pub attributes : Vec<attributes::AttributeInfo>
}

impl<'a> ClassDesc<'a> {
    pub fn new<T: Read>(reader: &mut T) -> ClassDesc {
        let magic = utils::fetch_u32(reader);
        let miv = utils::fetch_u16(reader);
        let mav = utils::fetch_u16(reader);
        let constant_pool = ClassDesc::fetch_constant_pool(reader);
        let access_flags = utils::fetch_u16(reader);
        let this_class = utils::fetch_u16(reader);
        let class_name = constant_pool[this_class as usize].class();
        let parent_class = utils::fetch_u16(reader);
        let parent_class_name = constant_pool[parent_class as usize].class();
        let interfaces = ClassDesc::fetch_interfaces(reader);
        let fields = ClassDesc::fetch_fields(reader, &constant_pool);
        let methods = ClassDesc::fetch_methods(reader, &constant_pool);
        let attributes = attributes::AttributeInfo::fetch_attributes(reader, &constant_pool);

        ClassDesc {
            magic,
            minor_version: miv,
            major_version: mav,
            constant_pool,
            access_flags,
            name: class_name,
            parent_class_name,
            parent_class: None,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    fn fetch_constant_pool<T: Read>(reader: &mut T) -> Vec<constantpool::ConstantPoolEntry> {
        let cp_size = utils::fetch_u16(reader);
        let mut constant_pool = Vec::with_capacity((cp_size + 1) as usize);

        constant_pool.push(
            constantpool::ConstantPoolEntry::Unknown("Padding".to_string())
        );

        for _ in 1..cp_size {
            let tag = utils::fetch_bytes(reader, 1)[0] as u8;

            let constant_pool_entry = match tag {
                1  => { let length = utils::fetch_u16(reader); constantpool::ConstantPoolEntry::Utf8( String::from_utf8_lossy( &utils::fetch_bytes(reader, length as usize) ).to_string() ) } ,
                3  => constantpool::ConstantPoolEntry::Integer( constantpool::CONSTANT_Integer { bytes : utils::fetch_u32(reader)  } ),
                4  => constantpool::ConstantPoolEntry::Float( constantpool::CONSTANT_Float { bytes : utils::fetch_u32(reader) } ),
                7  => constantpool::ConstantPoolEntry::Class( utils::fetch_u16(reader).to_string() ),
                8  => constantpool::ConstantPoolEntry::String( utils::fetch_u16(reader).to_string() ),
                9  => constantpool::ConstantPoolEntry::FieldRef( constantpool::CONSTANT_Fieldref { class : utils::fetch_u16(reader).to_string(), name_and_type_index : utils::fetch_u16(reader), field : String::new(), descriptor : String::new() } ),
                10 => constantpool::ConstantPoolEntry::MethodRef( constantpool::CONSTANT_Methodref { class : utils::fetch_u16(reader).to_string(), name_and_type_index : utils::fetch_u16(reader), method : String::new(), descriptor : String::new() } ),
                11 => constantpool::ConstantPoolEntry::InterfaceMethodRef( constantpool::CONSTANT_InterfaceMethodref { class : utils::fetch_u16(reader).to_string(), name_and_type_index : utils::fetch_u16(reader), field_or_method : String::new(), descriptor : String::new()  } ),
                12 => constantpool::ConstantPoolEntry::NameAndType( constantpool::CONSTANT_NameAndType { name : utils::fetch_u16(reader).to_string(), descriptor : utils::fetch_u16(reader).to_string() } ),
                15 => constantpool::ConstantPoolEntry::MethodHandle( constantpool::CONSTANT_MethodHandle { reference_kind : utils::fetch_bytes(reader, 1)[0] as u8, reference_index : utils::fetch_u16(reader) } ),
                17 => constantpool::ConstantPoolEntry::Dynamic( constantpool::CONSTANT_Dynamic { bootstrap_method_attr_index : utils::fetch_u16(reader), name_and_type_index : utils::fetch_u16(reader), field : String::new(), descriptor : String::new() } ),
                18 => constantpool::ConstantPoolEntry::InvokeDynamic( constantpool::CONSTANT_InvokeDynamic { bootstrap_method_attr_index : utils::fetch_u16(reader), name_and_type_index : utils::fetch_u16(reader), method : String::new(), descriptor : String::new() } ),
                _  => constantpool::ConstantPoolEntry::Unknown( "Unknown".to_string() ),
            };

            constant_pool.push( constant_pool_entry );
        }

        let mut read_only_cp = constant_pool.clone();

        for cp_entry in &mut constant_pool {
            match cp_entry {
                constantpool::ConstantPoolEntry::Class(ref mut c) => {
                    let idx = c.parse::<usize>().unwrap(); 
                    *c = read_only_cp[idx].utf8();
                },
                constantpool::ConstantPoolEntry::String(ref mut c) => {
                    let idx = c.parse::<usize>().unwrap(); 
                    *c = read_only_cp[idx].utf8();
                },
                _ => {},
            }           
        }

        read_only_cp = constant_pool.clone();
        for cp_entry in &mut constant_pool {
            match cp_entry {
                constantpool::ConstantPoolEntry::NameAndType(ref mut c) => {
                    let name_idx = c.name.parse::<usize>().unwrap(); 
                    c.name = read_only_cp[name_idx].utf8();

                    let descriptor_idx = c.descriptor.parse::<usize>().unwrap(); 
                    c.descriptor = read_only_cp[descriptor_idx].utf8();
                },
                _ => {}
            }
        }

        read_only_cp = constant_pool.clone();
        for cp_entry in &mut constant_pool {
            match cp_entry {
                constantpool::ConstantPoolEntry::FieldRef(ref mut c) => {
                    let class_idx = c.class.parse::<usize>().unwrap(); 
                    let name_type_idx = c.name_and_type_index; 
                    let name_type = read_only_cp[name_type_idx as usize].name_and_type();

                    c.class = read_only_cp[class_idx].class();
                    c.field = name_type.name.clone();
                    c.descriptor = name_type.descriptor.clone();
                },
                constantpool::ConstantPoolEntry::MethodRef(ref mut c) => {
                    let idx = c.class.parse::<usize>().unwrap(); 
                    c.class = read_only_cp[idx].class();
                },
                constantpool::ConstantPoolEntry::InterfaceMethodRef(ref mut c) => {
                    let idx = c.class.parse::<usize>().unwrap(); 
                    c.class = read_only_cp[idx].class();
                },
                _ => {},
            }           
        }

        constant_pool
    }

    fn fetch_interfaces<T: Read>(reader: &mut T) -> Vec<u16> {
        let interfaces_count = utils::fetch_u16(reader);
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);

        for _ in 0..interfaces_count {
            let interface = utils::fetch_u16(reader);
            interfaces.push(interface);
        }

        interfaces
    }

    fn fetch_fields<T: Read>(reader: &mut T, constant_pool : &Vec<constantpool::ConstantPoolEntry>) -> Vec<Field> {
        let fields_count = utils::fetch_u16(reader);
        let mut fields = Vec::with_capacity(fields_count as usize);

        for _ in 0..fields_count {
            let access_flags = utils::fetch_u16(reader);
            let name_index = utils::fetch_u16(reader);
            let name = constant_pool[name_index as usize].utf8();
            let descriptor_index = utils::fetch_u16(reader);
            let descriptor = constant_pool[descriptor_index as usize].utf8();
            let attributes = attributes::AttributeInfo::fetch_attributes(reader, constant_pool);

            fields.push(
                Field {
                    access_flags,
                    name,
                    descriptor,
                    attributes,
                    value: None,
                }
            );
        }

        fields
    }

    fn fetch_methods<T: Read>(reader: &mut T, constant_pool : &Vec<constantpool::ConstantPoolEntry>) -> Vec<Method> {
        let methods_count = utils::fetch_u16(reader);
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let access_flags = utils::fetch_u16(reader);
            let name_index = utils::fetch_u16(reader);
            let name = constant_pool[name_index as usize].utf8();
            let descriptor_index = utils::fetch_u16(reader);
            let attributes = attributes::AttributeInfo::fetch_attributes(reader, &constant_pool);

            methods.push(
                Method {
                    access_flags,
                    name,
                    descriptor_index,
                    attributes,
                }
            );
        }

        methods
    }

    pub fn flags_names(flags : u16) -> String {
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
        println!("{:<30} 0x{:X?}", "Magic number:", self.magic);
        println!("{:<30} {}.{}", "Version:", self.major_version, self.minor_version);
        println!("{:<30} {}", "Access Flags:", ClassDesc::flags_names(self.access_flags));
        println!("{:<30} {}", "This Class:", self.name);
        println!("{:<30} {}", "Super Class:", self.parent_class_name);

        if attributes {
            println!("Class Attributes:");

            for attribute in &self.attributes {
                attribute.print_info();
            }
        }

        if interfaces {
            println!("Interfaces:");

            for interface_entry in &self.interfaces {
                println!("\t {}", interface_entry);
            }
        }

        if constant_pool {
            println!("Constant Pool:");

            let mut i = 0;
            for constant_pool_entry in &self.constant_pool {
                println!("cp[{}] = {:?}", i, constant_pool_entry);
                i = i + 1;
            }
        }

        if fields {
            println!("Fields:");

            for field_entry in &self.fields {
                println!("\t {:?}", field_entry);
                if field_entry.attributes.len() > 0 {
                    println!("\tAttributes: ");

                    for attribute in &field_entry.attributes {
                        attribute.print_info();
                    }
                }
                println!();
            }
        }

        if methods {
            println!("Methods:");

            for method in &self.methods {
                println!("\tMethod name: {} {}", method.name, method.descriptor_index);
                println!("\tAccess flags: {}", ClassDesc::flags_names(method.access_flags));

                if method.attributes.len() > 0 {
                    println!("\tAttributes: ");

                    for attribute in &method.attributes {
                        attribute.print_info();
                    }
                }

                println!();
            }
        }
    }

    pub fn parse_bytecode(bytes : Vec<u8>) -> Vec<bytecode::Bytecode_Instruction> {
        let mut cursor = Cursor::new(&bytes);
        let mut bytecodes = Vec::new();

        while (cursor.position() as usize) < bytes.len() {
            let opcode = cursor.read_u8().unwrap();
            let instruction = match opcode {
                2 => bytecode::Bytecode_Instruction::Iconstm1,
                3 => bytecode::Bytecode_Instruction::Iconst0,
                4 => bytecode::Bytecode_Instruction::Iconst1,
                5 => bytecode::Bytecode_Instruction::Iconst2,
                6 => bytecode::Bytecode_Instruction::Iconst3,
                7 => bytecode::Bytecode_Instruction::Iconst4,
                8 => bytecode::Bytecode_Instruction::Iconst5,

                9 => bytecode::Bytecode_Instruction::Lconst0,
                10 => bytecode::Bytecode_Instruction::Lconst1,
                11 => bytecode::Bytecode_Instruction::Fconst0,
                12 => bytecode::Bytecode_Instruction::Fconst1,
                13 => bytecode::Bytecode_Instruction::Fconst2,

                16 => {
                    let index = cursor.read_i8().unwrap();
                    bytecode::Bytecode_Instruction::Bipush(index)
                },
                17 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Sipush(index)
                },
                18 => {
                    let index = cursor.read_u8().unwrap();
                    bytecode::Bytecode_Instruction::Ldc(index)
                },

                21 => {
                    let index = cursor.read_u8().unwrap();
                    bytecode::Bytecode_Instruction::Iload(index)
                },
                26 => bytecode::Bytecode_Instruction::Iload0,
                27 => bytecode::Bytecode_Instruction::Iload1,
                28 => bytecode::Bytecode_Instruction::Iload2,
                29 => bytecode::Bytecode_Instruction::Iload3,

                42 => bytecode::Bytecode_Instruction::Aload0,
                43 => bytecode::Bytecode_Instruction::Aload1,
                44 => bytecode::Bytecode_Instruction::Aload2,
                45 => bytecode::Bytecode_Instruction::Aload3,

                54 => {
                    let index = cursor.read_u8().unwrap();
                    bytecode::Bytecode_Instruction::Istore(index)
                },

                59 => bytecode::Bytecode_Instruction::Istore0,
                60 => bytecode::Bytecode_Instruction::Istore1,
                61 => bytecode::Bytecode_Instruction::Istore2,
                62 => bytecode::Bytecode_Instruction::Istore3,

                75 => bytecode::Bytecode_Instruction::Astore0,
                76 => bytecode::Bytecode_Instruction::Astore1,
                77 => bytecode::Bytecode_Instruction::Astore2,
                78 => bytecode::Bytecode_Instruction::Astore3,
                83 => bytecode::Bytecode_Instruction::Aastore,

                89 => bytecode::Bytecode_Instruction::Dup,
                96 => bytecode::Bytecode_Instruction::Iadd,

                132 => {
                    let index = cursor.read_u8().unwrap();
                    let value = cursor.read_i8().unwrap();
                    bytecode::Bytecode_Instruction::Iinc{index, value}
                },

                153 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Ifeq(index)
                },
                154 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Ifne(index)
                },
                155 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Iflt(index)
                },
                156 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Ifge(index)
                },
                157 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Ifgt(index)
                },
                158 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Ifle(index)
                },
                159 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmpeq(index)
                },
                160 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmpne(index)
                },
                161 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmplt(index)
                },
                162 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmpge(index)
                },
                163 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmpgt(index)
                },
                164 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::IfIcmple(index)
                },

                167 => {
                    let index = cursor.read_i16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Goto(index)
                },


                172 => bytecode::Bytecode_Instruction::Ireturn,
                177 => bytecode::Bytecode_Instruction::Return,
                178 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Getstatic(index)
                },
                179 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Putstatic(index)
                },

                182 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Invokevirtual(index)
                },
                183 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Invokespecial(index)
                },
                184 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Invokestatic(index)
                },
                186 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    let _ = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Invokedynamic(index)
                },
                187 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::New(index)
                },
                189 => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    bytecode::Bytecode_Instruction::Anewarray(index)
                },
                _ => panic!("Unrecognized opcode {}", opcode),
            };

            bytecodes.push(instruction);
        }

        return bytecodes;
    }
}