use std::fmt;
use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::bytecode;
use crate::attributes;
use crate::constantpool;

pub struct ClassFile {
    pub magic : u32,
    pub minor_version : u16,
    pub major_version : u16,
    pub constant_pool : Vec<constantpool::ConstantPoolInfo>,
    pub access_flags : u16,
    pub this_class : u16,
    pub super_class : u16,
    pub interfaces : Vec<u16>,
    pub fields : Vec<FieldInfo>,
    pub methods : Vec<MethodInfo>,
    pub attributes : Vec<attributes::AttributeInfo>
}

pub struct MethodInfo {
    pub access_flags : u16,
    pub name_index : u16,
    pub descriptor_index : u16,
    pub attributes : Vec<attributes::AttributeInfo>,
}

pub struct FieldInfo {
    pub access_flags : u16,
    pub name_index : u16,
    pub descriptor_index : u16,
    pub attributes : Vec<attributes::AttributeInfo>
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
            constantpool::ConstantPoolInfo::Unknown("Padding".to_string())
        );

        for _ in 1..cp_size {
            let tag = load_part(1)[0] as u8;

            let constant_pool_entry = match tag {
                9=> constantpool::ConstantPoolInfo::FieldRef( constantpool::CONSTANT_Fieldref_info { class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                10 => constantpool::ConstantPoolInfo::MethodRef( constantpool::CONSTANT_Methodref_info { class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                11 => constantpool::ConstantPoolInfo::InterfaceMethodRef( constantpool::CONSTANT_InterfaceMethodref_info { class_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2))  } ),
                7  => constantpool::ConstantPoolInfo::Class( constantpool::CONSTANT_Class_info { name_index : BigEndian::read_u16(&load_part(2))  } ),
                12  => constantpool::ConstantPoolInfo::NameAndType( constantpool::CONSTANT_NameAndType_info { name_index : BigEndian::read_u16(&load_part(2)), descriptor_index : BigEndian::read_u16(&load_part(2)) } ),
                1  => { let length = BigEndian::read_u16(&load_part(2)); constantpool::ConstantPoolInfo::Utf8( constantpool::CONSTANT_Utf8_info { bytes : load_part(length as usize) } ) } ,
                3  => constantpool::ConstantPoolInfo::Integer( constantpool::CONSTANT_Integer_info { bytes : BigEndian::read_u32(&load_part(4))  } ),
                4  => constantpool::ConstantPoolInfo::Float( constantpool::CONSTANT_Float_info { bytes : BigEndian::read_u32(&load_part(4))  } ),
                17  => constantpool::ConstantPoolInfo::Dynamic( constantpool::CONSTANT_Dynamic_info { bootstrap_method_attr_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2)) } ),
                18  => constantpool::ConstantPoolInfo::InvokeDynamic( constantpool::CONSTANT_InvokeDynamic_info { bootstrap_method_attr_index : BigEndian::read_u16(&load_part(2)), name_and_type_index : BigEndian::read_u16(&load_part(2)) } ),
                8  => constantpool::ConstantPoolInfo::String( constantpool::CONSTANT_String_info { string_index : BigEndian::read_u16(&load_part(2))  } ),
                15  => constantpool::ConstantPoolInfo::MethodHandle( constantpool::CONSTANT_MethodHandle_info { reference_kind : load_part(1)[0] as u8, reference_index : BigEndian::read_u16(&load_part(2)) } ),
                _  => constantpool::ConstantPoolInfo::Unknown( "Unknown".to_string() ),
            };

            constant_pool.push( constant_pool_entry );
        }

        let access_flags = BigEndian::read_u16(&load_part(2));
        let this_class = BigEndian::read_u16(&load_part(2));
        let super_class = BigEndian::read_u16(&load_part(2));
        let interfaces_count = BigEndian::read_u16(&load_part(2));
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);

        for _ in 0..interfaces_count {
            let interface_info = BigEndian::read_u16(&load_part(2));
            interfaces.push(interface_info);
        }

        let fields_count = BigEndian::read_u16(&load_part(2));
        let mut fields = Vec::with_capacity(fields_count as usize);

        for _ in 0..fields_count {
            let access_flags = BigEndian::read_u16(&load_part(2));
            let name_index = BigEndian::read_u16(&load_part(2));
            let descriptor_index = BigEndian::read_u16(&load_part(2));
            let attributes_count = BigEndian::read_u16(&load_part(2));
            let mut attributes = Vec::with_capacity(attributes_count as usize);

            for _ in 0..attributes_count {
                let attribute_name_index = BigEndian::read_u16(&load_part(2));
                let attribute_length = BigEndian::read_u32(&load_part(4));
                let info = load_part(attribute_length as usize);

                attributes.push(
                    attributes::AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
                );
            }

            fields.push(
                FieldInfo {
                    access_flags : access_flags,
                    name_index : name_index,
                    descriptor_index : descriptor_index,
                    attributes : attributes,
                }
            );
        }

        let methods_count = BigEndian::read_u16(&load_part(2));
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let access_flags = BigEndian::read_u16(&load_part(2));
            let name_index = BigEndian::read_u16(&load_part(2));
            let descriptor_index = BigEndian::read_u16(&load_part(2));
            let attributes_count = BigEndian::read_u16(&load_part(2));
            let mut attributes = Vec::with_capacity(attributes_count as usize);

            for _ in 0..attributes_count {
                let attribute_name_index = BigEndian::read_u16(&load_part(2));
                let attribute_length = BigEndian::read_u32(&load_part(4));
                let info = load_part(attribute_length as usize);
                let attr = attributes::AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info);

                attributes.push(attr);
            }

            methods.push(
                MethodInfo {
                    access_flags : access_flags,
                    name_index : name_index,
                    descriptor_index : descriptor_index,
                    attributes : attributes,
                }
            );
        }

        let attributes_count = BigEndian::read_u16(&load_part(2));
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_name_index = BigEndian::read_u16(&load_part(2));
            let attribute_length = BigEndian::read_u32(&load_part(4));
            let info = load_part(attribute_length as usize);

            attributes.push(
                attributes::AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
            );
        }

        ClassFile {
            magic: magic,
            minor_version: miv,
            major_version: mav,
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces : interfaces,
            fields : fields,
            methods : methods,
            attributes : attributes,
        }
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
        let class = self.constant_pool[self.this_class as usize].class();
        let super_class = self.constant_pool[self.super_class as usize].class();

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
            println!("Interfaces:");

            for interface_entry in &self.interfaces {
                println!("\t {}", self.constant_pool[*interface_entry as usize]);
            }
        }

        if constant_pool {
            println!("Constant Pool:");

            let mut i = 0;
            for constant_pool_entry in &self.constant_pool {
                println!("cp[{}] = ", i);
                i = i + 1;
                match constant_pool_entry {
                    constantpool::ConstantPoolInfo::Class(value) => { 
                        println!("\tClass:");
                        println!("\t\tname_index: {}", value.name_index);
                    },
                    constantpool::ConstantPoolInfo::NameAndType(value) => {
                        println!("\tNameAndtype:");
                        println!("\t\tname_index: {} \t # {}", value.name_index, self.constant_pool[value.name_index as usize]);
                        println!("\t\tdescriptor_index: {} \t # {}", value.descriptor_index, self.constant_pool[value.descriptor_index as usize]);
                    },
                    constantpool::ConstantPoolInfo::Utf8(value) => {
                        println!("\tUTF-8:");
                        println!("\t\tbytes: {}", value);
                    },
                    constantpool::ConstantPoolInfo::Integer(value) => {
                        println!("\tInteger:");
                        println!("\t\tbytes: {}", value.bytes);
                    },
                    constantpool::ConstantPoolInfo::Float(value) => {
                        println!("\tFloat:");
                        println!("\t\tbytes: {}", value.bytes);
                    },
                    constantpool::ConstantPoolInfo::MethodRef(value) => {
                        println!("\tMethodRef:");
                        println!("\t\tclass_index: {}", value.class_index);
                        println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                    },
                    constantpool::ConstantPoolInfo::FieldRef(value) => {
                        println!("\tFieldRef:");
                        println!("\t\tclass_index: {}", value.class_index);
                        println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                    },
                    constantpool::ConstantPoolInfo::InterfaceMethodRef(value) => {
                        println!("\tInterfaceMethodRef:");
                        println!("\t\tclass_index: {}", value.class_index);
                        println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                    },
                    constantpool::ConstantPoolInfo::Dynamic(value) => {
                        println!("\tDynamic:");
                        println!("\t\tbootstrap_method_attr_index: {}", value.bootstrap_method_attr_index);
                        println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                    },
                    constantpool::ConstantPoolInfo::InvokeDynamic(value) => {
                        println!("\tInvokeDynamic:");
                        println!("\t\tbootstrap_method_attr_index: {}", value.bootstrap_method_attr_index);
                        println!("\t\tname_and_type_index: {}", value.name_and_type_index);
                    },
                    constantpool::ConstantPoolInfo::String(value) => {
                        println!("\tString:");
                        println!("\t\tstring_index: {} \t # {}", value.string_index, self.constant_pool[value.string_index as usize]);
                    },
                    constantpool::ConstantPoolInfo::MethodHandle(value) => {
                        println!("\tMethodHandle:");
                        println!("\t\treference_kind: {}", value.reference_kind);
                        println!("\t\treference_index: {}", value.reference_index);
                    },
                    _ => println!("Unknown"),
                }
            }
        }

        if fields {
            println!("Fields:");

            for field_entry in &self.fields {
                println!("\t Access flags: {}, Name: {}, descritor_index: {}", 
                    ClassFile::flags_names(field_entry.access_flags),
                    self.constant_pool[field_entry.name_index as usize],
                    field_entry.descriptor_index,
                );
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
                println!("\tMethod name: {} {}", self.constant_pool[method.name_index as usize], self.constant_pool[method.descriptor_index as usize]);
                println!("\tAccess flags: {}", ClassFile::flags_names(method.access_flags));

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