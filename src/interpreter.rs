use std::fs::File;
use std::io::BufReader;
use crate::spec;
use crate::constantpool;
use crate::bytecode;
use std::collections::HashMap;

pub struct Frame {
    pub class_name : String,
    pub locals : Vec<u64>,
    pub stack : Vec<u64>,
    pub method_idx : u64,
    pub bytecode_idx : u64,
    pub code_idx : u64,
}

pub struct Interpreter <'a> {
    loaded_classes : HashMap<String, &'a mut spec::ClassDesc<'a>>,
    frames : Vec<Frame>,
}

impl<'a> Interpreter <'a> {
    fn build_frame_for(startup_class : &spec::ClassDesc<'a>, name : &str) -> Option<Frame> {
        for (pos, method) in startup_class.methods.iter().enumerate() {
            if name == method.name {
                let mut locals_size = 0;
                let mut stack_size = 0;
                let mut code_idx = 0;

                for (pos, attr) in method.attributes.iter().enumerate() {
                    if attr.code.is_some() {
                        code_idx = pos;
                        locals_size = attr.code.as_ref().unwrap().max_locals;
                        stack_size = attr.code.as_ref().unwrap().max_stack;
                    }
                }

                let frame = Frame { 
                    class_name : startup_class.name.clone(),
                    locals : Vec::with_capacity(locals_size as usize), 
                    stack : Vec::with_capacity(stack_size as usize), 
                    method_idx : pos as u64,
                    bytecode_idx : 0,
                    code_idx : code_idx as u64,
                };

                return Some(frame)
            }
        }

        return None
    }

    pub fn new(startup_class : &'a mut spec::ClassDesc<'a>) -> Self {
        let main_frame = Interpreter::build_frame_for(&startup_class, "main");

        if main_frame.is_some() {
            let mut frames = Vec::<Frame>::new();

            frames.push(main_frame.unwrap());

            let cinit_frame = Interpreter::build_frame_for(&startup_class, "<clinit>");

            if cinit_frame.is_some() {
                frames.push(cinit_frame.unwrap());
            }

            let mut loaded_classes = HashMap::new();
            loaded_classes.insert(startup_class.name.clone(), startup_class);

            Interpreter {
                frames,
                loaded_classes,
            }
        }
        else {
            panic!("Didn't find method main in the class.");
        }
    }

    pub fn run(&mut self) -> bool {
        while !self.frames.is_empty() {
            let mut frame = &mut self.frames.pop().unwrap();
            let mut operand_stack = &mut frame.stack;
            let mut locals = &frame.locals;
            let class = self.loaded_classes.remove(&frame.class_name).unwrap();
            let method = &class.methods[frame.method_idx as usize];
            let code_attr = method.attributes[frame.code_idx as usize].code.as_ref().unwrap();
            let mut idx = frame.bytecode_idx;

            println!("Popping one frame. Stack size {}. Locals {}. Code size {}", operand_stack.capacity(), locals.capacity(), code_attr.code.len());

            loop {
                let instr = &code_attr.code[idx as usize];
                println!("{:?}", instr);

                match instr {
                    bytecode::Bytecode_Instruction::Iconst0 => { operand_stack.push(0); },
                    bytecode::Bytecode_Instruction::Iconst1 => { operand_stack.push(1); },
                    bytecode::Bytecode_Instruction::Iconst2 => { operand_stack.push(2); },
                    bytecode::Bytecode_Instruction::Iconst3 => { operand_stack.push(3); },
                    bytecode::Bytecode_Instruction::Iconst4 => { operand_stack.push(4); },
                    bytecode::Bytecode_Instruction::Iconst5 => { operand_stack.push(5); },

                    bytecode::Bytecode_Instruction::Astore1 => {},
                    bytecode::Bytecode_Instruction::Istore2 => {},
                    bytecode::Bytecode_Instruction::Iload2 => {},
                    bytecode::Bytecode_Instruction::Dup => {},
                    bytecode::Bytecode_Instruction::Iadd => {},
                    bytecode::Bytecode_Instruction::Iconst0 => {},
                    bytecode::Bytecode_Instruction::Aload1 => {},
                    bytecode::Bytecode_Instruction::Iinc{index, value} => {},
                    bytecode::Bytecode_Instruction::New(idx) => {},
                    bytecode::Bytecode_Instruction::Goto(idx) => {},
                    bytecode::Bytecode_Instruction::Putstatic(idx) => { self.putstatic(&class.constant_pool[*idx as usize].field(), &mut class.fields, &mut operand_stack) },
                    bytecode::Bytecode_Instruction::Getstatic(idx) => {},
                    bytecode::Bytecode_Instruction::Invokespecial(idx) => {},
                    bytecode::Bytecode_Instruction::Ldc(idx) => {},
                    bytecode::Bytecode_Instruction::IfIcmpge(idx) => {},
                    bytecode::Bytecode_Instruction::Invokevirtual(idx) => {},
                    bytecode::Bytecode_Instruction::Invokedynamic(idx) => {},
                    bytecode::Bytecode_Instruction::Return => { break; },
                    _ => println!("Unknown instruction {:?}", instr),
                }

                idx = idx + 1;
            }

            self.loaded_classes.insert(frame.class_name.clone(), class);
        }

        true
    }

    fn putstatic(&self, field_desc : &constantpool::CONSTANT_Fieldref, fields : &mut Vec<spec::Field>, operand_stack : &mut Vec<u64>) {
        for candidate_field in fields {
            if candidate_field.name == field_desc.field && candidate_field.descriptor == field_desc.descriptor {
                println!("{:?}", candidate_field);
                candidate_field.value = operand_stack.pop();
                println!("{:?}", candidate_field);
            }
        }

    }
}