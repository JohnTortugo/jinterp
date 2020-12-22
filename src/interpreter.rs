use crate::spec;
use crate::bytecode;

pub struct Frame <'a> {
    pub class : &'a spec::ClassFile,
    pub locals : Vec<u64>,
    pub stack : Vec<u64>,
    pub method_idx : u64,
    pub bytecode_idx : u64,
    pub code_idx : u64,
}

pub struct Interpreter <'a> {
    frames : Vec<Frame<'a>>,
}

impl<'a> Interpreter <'a> {
    fn build_frame_for(startup_class : &'a spec::ClassFile, name : &str) -> Option<Frame<'a>> {
        for (pos, method) in startup_class.methods.iter().enumerate() {
            let candidate_name_bytes = &startup_class.constant_pool[ method.name_index as usize ].utf8().bytes;
            let candidate_name = String::from_utf8_lossy(candidate_name_bytes);

            if name == candidate_name {
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

                return Some(Frame { 
                    class : startup_class, 
                    locals : Vec::with_capacity(locals_size as usize), 
                    stack : Vec::with_capacity(stack_size as usize), 
                    method_idx : pos as u64,
                    bytecode_idx : 0,
                    code_idx : code_idx as u64,
                })
            }
        }

        return None
    }

    pub fn new(startup_class : &'a spec::ClassFile) -> Self {
        let mut frames = Vec::new();
        let cinit_frame = Interpreter::build_frame_for(startup_class, "<clinit>");
        let main_frame = Interpreter::build_frame_for(startup_class, "main");

        if main_frame.is_some() {
            frames.push(main_frame.unwrap());
        }
        else {
            panic!("Didn't find method main in the class.");
        }

        if cinit_frame.is_some() {
            frames.push(cinit_frame.unwrap());
        }

        Interpreter {
            frames,
        }
    }

    pub fn run(&mut self) -> bool {
        while !self.frames.is_empty() {
            let frame = self.frames.pop().unwrap();
            let method = &frame.class.methods[frame.method_idx as usize];
            let code_attr = method.attributes[frame.code_idx as usize].code.as_ref().unwrap();
            let mut idx = frame.bytecode_idx;

            println!("Popping one frame. Stack size {}. Locals {}. Code size {}", frame.stack.capacity(), frame.locals.capacity(), code_attr.code.len());

            loop {
                let instr = &code_attr.code[idx as usize];
                println!("{:?}", instr);

                match instr {
                    bytecode::Bytecode_Instruction::Iconst0 => {},
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
                    bytecode::Bytecode_Instruction::Putstatic(idx) => {},
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
}