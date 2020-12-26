use std::fs::File;
use std::io::BufReader;
use crate::spec;
use crate::constantpool;
use crate::bytecode;
use std::collections::HashMap;

pub struct Frame <'a> {
    pub class : &'a mut spec::ClassDesc<'a>,
    pub locals : Vec<u64>,
    pub stack : Vec<u64>,
    pub method_idx : u64,
    pub bytecode_idx : u64,
    pub code_idx : u64,
}

pub struct Interpreter <'a> {
    loaded_classes : HashMap<String, &'a mut spec::ClassDesc<'a>>,
    frames : Vec<Frame<'a>>,
}

impl<'a> Interpreter <'a> {
    fn build_frame_for(startup_class : &'a mut spec::ClassDesc<'a>, name : &str) -> Option<Frame<'a>> {
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

                //return Some(Frame { 
                //    class : startup_class, 
                //    locals : Vec::with_capacity(locals_size as usize), 
                //    stack : Vec::with_capacity(stack_size as usize), 
                //    method_idx : pos as u64,
                //    bytecode_idx : 0,
                //    code_idx : code_idx as u64,
                //})
            }
        }

        return None
    }

    pub fn new(startup_class : &'a mut spec::ClassDesc<'a>, filename : &String) -> Self {
        let mut frames = Vec::<Frame>::new();
        let main_frame = Interpreter::build_frame_for(startup_class, "main");
        let cinit_frame = Interpreter::build_frame_for(startup_class, "<clinit>");

        if main_frame.is_some() {
            frames.push(main_frame.unwrap());
        }
        else {
            panic!("Didn't find method main in the class.");
        }

 //       if cinit_frame.is_some() {
  //          frames.push(cinit_frame.unwrap());
   //     }

        let mut loaded_classes = HashMap::new();
        //loaded_classes.insert(startup_class.name.clone(), startup_class);

        Interpreter {
            frames,
            loaded_classes,
        }
    }

    pub fn run(&mut self) -> bool {
        while !self.frames.is_empty() {
            let mut frame = self.frames.pop().unwrap();
            let mut operand_stack = frame.stack;
            let mut locals = frame.locals;
            let class = frame.class;
            let constant_pool = &class.constant_pool;
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
                    bytecode::Bytecode_Instruction::Putstatic(idx) => {}, //self.putstatic(&constant_pool, *idx, class, &mut operand_stack),
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
        }

        true
    }

    fn putstatic(&self, constantpool : &Vec<constantpool::ConstantPoolEntry>, cp_field_index : u16, class : &mut spec::ClassDesc, operand_stack : &mut Vec<u64>) {
        let field_desc = &constantpool[cp_field_index as usize].field();

        for candidate_field in &mut class.fields {
            if candidate_field.name == field_desc.field && candidate_field.descriptor == field_desc.descriptor {
                println!("{:?}", candidate_field);
                candidate_field.value = operand_stack.pop();
                println!("{:?}", candidate_field);
            }
        }

    }
}