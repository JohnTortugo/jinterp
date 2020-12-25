use std::fmt;
use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::bytecode;
use crate::spec;
use crate::attributes;
use crate::constantpool;

#[derive(Clone)]
pub struct AttributeInfo {
    pub name : String,
    pub source_file : Option<String>,
    pub bootstrap_methods : Option<Vec<BootstrapMethods_attribute>>,
    pub inner_classes : Option<Vec<InnerClasses_attribute>>,
    pub code : Option<Code_attribute>,
}

#[derive(Clone)]
pub struct BootstrapMethods_attribute {
    pub bootstrap_method_ref : u16,
    pub bootstrap_arguments : Vec<u16>,
}

#[derive(Clone)]
pub struct InnerClasses_attribute {
    pub inner_class_info : String,
    pub outer_class_info : Option<String>,
    pub inner_name : Option<String>,
    pub inner_class_access_flags : u16,
}

#[derive(Clone)]
pub struct ExceptionTable_entry {
    pub start_pc : u16,
    pub end_pc : u16,
    pub handler_pc : u16,
    pub catch_type : u16,
}

#[derive(Clone)]
pub struct Code_attribute {
    pub max_stack : u16,
    pub max_locals : u16,
    pub code : Vec<bytecode::Bytecode_Instruction>,
    pub exception_table : Vec<ExceptionTable_entry>,
    pub attributes : Vec<AttributeInfo>,
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
                println!("\t\t{} {}", "Flags:", spec::ClassFile::flags_names(inner_class.inner_class_access_flags));
            }
        }

        if self.bootstrap_methods.is_some() {
            for bootstrap_method in self.bootstrap_methods.as_deref().unwrap() {
                println!("\t\t{} {}", "Bootstrap Method Index:", bootstrap_method.bootstrap_method_ref);
                println!("\t\t{} {:?}", "Bootstrap Method Arguments:", bootstrap_method.bootstrap_arguments);
            }
        }

        if self.code.is_some() {
            let bytecode = self.code.as_ref().unwrap();
            println!("\t\tStack={}, Locals={}", bytecode.max_stack, bytecode.max_locals);

            for instruction in &bytecode.code {
                println!("\t\t\t{:?}", instruction);
            }
        }
    }

    pub fn build_attribute_info(constant_pool : &Vec<constantpool::ConstantPoolEntry>, attribute_name_index : u16, info : Vec<u8>) -> AttributeInfo {
        let mut cursor = Cursor::new(info);
        let constant_pool_attr_name_entry = constant_pool[attribute_name_index as usize].utf8();
        let name = constant_pool_attr_name_entry;
        let mut source_file = None;
        let mut inner_classes = None;
        let mut bootstrap_methods = None;
        let mut code = None;

        if name == "SourceFile" {
            let sourcefile_index = cursor.read_u16::<BigEndian>().unwrap();
            let name2 = constant_pool[sourcefile_index as usize].utf8() ;
            source_file = Some(name2);
        }
        else if name == "InnerClasses" {
            let number_of_classes = cursor.read_u16::<BigEndian>().unwrap();
            let mut classes = Vec::with_capacity(number_of_classes as usize);

            for _ in 0..number_of_classes {
                let inner_class_idx = cursor.read_u16::<BigEndian>().unwrap();
                let inner_class_info = constant_pool[inner_class_idx as usize].class();

                let outer_class_idx = cursor.read_u16::<BigEndian>().unwrap();
                let outer_class_info =  if outer_class_idx != 0 
                                            { 
                                                let outer_class_info_class_idx = constant_pool[outer_class_idx as usize].class();
                                                Some( outer_class_info_class_idx )
                                            }
                                        else
                                            { None };

                let inner_name_idx = cursor.read_u16::<BigEndian>().unwrap();
                let inner_name =    if inner_name_idx != 0 
                                        { 
                                            Some( constant_pool[inner_name_idx as usize].utf8() )
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

            for _ in 0..number_of_bootstrap_methods {
                let bootstrap_method_ref = cursor.read_u16::<BigEndian>().unwrap();
                let num_bootstrap_arguments = cursor.read_u16::<BigEndian>().unwrap();
                let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);

                for _ in 0..num_bootstrap_arguments {
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
            let max_stack = cursor.read_u16::<BigEndian>().unwrap();
            let max_locals = cursor.read_u16::<BigEndian>().unwrap();
            let code_length = cursor.read_u32::<BigEndian>().unwrap();
            let mut bytes = Vec::with_capacity(code_length as usize);
            
            //cursor.take(code_length as u64).read(&mut bytecodes);
            for _ in 0..code_length {
                bytes.push(cursor.read_u8().unwrap());
            }

            let exception_table_length = cursor.read_u16::<BigEndian>().unwrap();
            let mut exception_table = Vec::with_capacity(exception_table_length as usize);
            for _ in 0..exception_table_length {
                let start_pc = cursor.read_u16::<BigEndian>().unwrap();
                let end_pc = cursor.read_u16::<BigEndian>().unwrap();
                let handler_pc = cursor.read_u16::<BigEndian>().unwrap();
                let catch_type = cursor.read_u16::<BigEndian>().unwrap();

                exception_table.push(
                    ExceptionTable_entry {
                        start_pc,
                        end_pc,
                        handler_pc,
                        catch_type,
                    }
                );
            }

            let attributes_count = cursor.read_u16::<BigEndian>().unwrap();
            let mut attributes = Vec::with_capacity(attributes_count as usize);
            for _ in 0..attributes_count {
                let attribute_name_index = cursor.read_u16::<BigEndian>().unwrap();
                let attribute_length = cursor.read_u32::<BigEndian>().unwrap();
                let mut info = Vec::with_capacity(code_length as usize);

                for _ in 0..attribute_length {
                    info.push(cursor.read_u8().unwrap());
                }

                attributes.push(
                    AttributeInfo::build_attribute_info(&constant_pool, attribute_name_index, info)
                );
            }

            code = Some(
                Code_attribute {
                    max_stack,
                    max_locals,
                    code : spec::ClassFile::parse_bytecode(bytes),
                    exception_table,
                    attributes
                }
            );
        }
        else if name == "LineNumberTable" {
            //println!("LineNumberTable attribute");
        }
        else if name == "StackMapTable" {
            //println!("StackMapTable attribute");
        }
        else {
            panic!("Unknown attribute name: {}", name);
        }

        AttributeInfo {
            name : name.to_string(),
            source_file,
            bootstrap_methods,
            inner_classes,
            code,
        }
    }
}