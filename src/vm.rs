use crate::opcodes::OpCode;
use crate::value::{Value, ValueType, Val};

pub(crate) struct VM {
    ip: usize,
    stack_pointer: usize,
    stack: Vec<Value>,
    memory: Vec<u8>,
}

static DEBUG: bool = true;

impl VM {
    pub(crate) fn new() -> Self {
        VM {
            ip: 0,
            stack_pointer: 0,
            stack: vec![],
            memory: vec![],
        }
    }

    pub(crate) fn interpret(&mut self, bytes: &[u8]) -> &Value {
        // copy program into memory
        for b in bytes {
            self.memory.push(b.clone());
        }

        while self.ip < bytes.len() {
            if DEBUG {
                // todo!("dissemble val")
            }

            let b = self.memory[self.ip]
                .try_into()
                .expect(&format!("Invalid OpCode {} at instruction {}", bytes[self.ip], self.ip));

            self.ip += 1;
            match b {
                OpCode::Return => {
                    if DEBUG {
                        // todo!("dissemble val")
                    }
                    return self.peek();
                }
                OpCode::Int => self.add_int(bytes),
                OpCode::Negate => self.negate(),
                OpCode::Add => self.add(),
                OpCode::Subtract => self.subtract(),
                OpCode::Multiply => self.multiply(),
                OpCode::Divide => self.divide(),
                OpCode::Float => self.add_float(bytes),
            }
        }

        todo!("error");
    }

    fn add_int(&mut self, bytes: &[u8]) {
        let byte1 = bytes[self.ip];
        let byte2 = bytes[self.ip + 1];
        let byte3 = bytes[self.ip + 2];
        let byte4 = bytes[self.ip + 3];

        let num = i32::from_be_bytes([byte1, byte2, byte3, byte4]);

        self.ip += 4;

        let val = Value {
            value_type: ValueType::Int,
            val: Val { i: num}
        };

        self.push(val);
    }

    fn add_float(&mut self, bytes: &[u8]) {
        let byte1 = bytes[self.ip];
        let byte2 = bytes[self.ip + 1];
        let byte3 = bytes[self.ip + 2];
        let byte4 = bytes[self.ip + 3];
        let byte5 = bytes[self.ip + 4];
        let byte6 = bytes[self.ip + 5];
        let byte7 = bytes[self.ip + 6];
        let byte8 = bytes[self.ip + 7];

        let num = f64::from_be_bytes([byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8]);

        self.ip += 8;

        let val = Value {
            value_type: ValueType::Float,
            val: Val { f: num }
        };

        self.push(val);
    }

    fn negate(&mut self) {
        let val = self.pop();
        match val.value_type {
            ValueType::Int => {
                let num = unsafe { val.val.i };
                self.push(Value {
                    value_type: ValueType::Int,
                    val: Val { i: -num },
                })
            }
            ValueType::Float => {
                let num = unsafe { val.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: -num },
                })
            }
            _ => todo!("error"),
        }
    }

    fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();

        match (&a.value_type, &b.value_type) {
            (ValueType::Int, ValueType::Int) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Int,
                    val: Val { i: num1 + num2 },
                })
            },
            (ValueType::Int, ValueType::Float) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 as f64 + num2 }
                })
            }
            (ValueType::Float, ValueType::Int) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 + num2 as f64 }
                })
            }
            (ValueType::Float, ValueType::Float) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 + num2 }
                })
            }
        }
    }

    fn subtract(&mut self) {
        let b = self.pop();
        let a = self.pop();

        match (&a.value_type, &b.value_type) {
            (ValueType::Int, ValueType::Int) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Int,
                    val: Val { i: num1 - num2 },
                })
            },
            (ValueType::Int, ValueType::Float) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 as f64 - num2 }
                })
            }
            (ValueType::Float, ValueType::Int) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 - num2 as f64 }
                })
            }
            (ValueType::Float, ValueType::Float) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 - num2 }
                })
            }
        }
    }

    fn multiply(&mut self) {
        let b = self.pop();
        let a = self.pop();

        match (&a.value_type, &b.value_type) {
            (ValueType::Int, ValueType::Int) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Int,
                    val: Val { i: num1 * num2 },
                })
            },
            (ValueType::Int, ValueType::Float) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 as f64 * num2 }
                })
            }
            (ValueType::Float, ValueType::Int) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 * num2 as f64 }
                })
            }
            (ValueType::Float, ValueType::Float) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 * num2 }
                })
            }
        }
    }

    fn divide(&mut self) {
        let b = self.pop();
        let a = self.pop();

        match (&a.value_type, &b.value_type) {
            (ValueType::Int, ValueType::Int) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Int,
                    val: Val { i: num1 / num2 },
                })
            },
            (ValueType::Int, ValueType::Float) => {
                let num1 = unsafe { a.val.i };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 as f64 / num2 }
                })
            }
            (ValueType::Float, ValueType::Int) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.i };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 / num2 as f64 }
                })
            }
            (ValueType::Float, ValueType::Float) => {
                let num1 = unsafe { a.val.f };
                let num2 = unsafe { b.val.f };
                self.push(Value {
                    value_type: ValueType::Float,
                    val: Val { f: num1 / num2 }
                })
            }
        }
    }

    fn push(&mut self, constant: Value) {
        if self.stack.len() >= 255 {
            panic!("Stack overflow");
        }
        self.stack.push(constant);
    }

    fn pop(&mut self) -> Value {
        if self.stack.len() == 0 {
            panic!("Stack underflow");
        }
        self.stack.pop().unwrap()
    }

    fn peek(&self) -> &Value {
        &self.stack.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let instructions = [
            1, // Constant
            0,
            0,
            0,
            10,
            1, // Constant
            0,
            0,
            0,
            5,
            3, // Add
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        unsafe { assert_eq!(val.val.i, 15) }
    }
    #[test]
    fn test_sub() {
        let instructions = [
            1, // Constant
            0,
            0,
            0,
            10,
            1, // Constant
            0,
            0,
            0,
            5,
            4, // Subtract
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        unsafe { assert_eq!(val.val.i, 5) }
    }

    #[test]
    fn test_mul() {
        let instructions = [
            1, // Constant
            0,
            0,
            0,
            10,
            1, // Constant
            0,
            0,
            0,
            5,
            5, // Multiply
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        unsafe { assert_eq!(val.val.i, 50) }
    }

    #[test]
    fn test_div() {
        let instructions = [
            1, // Constant
            0,
            0,
            0,
            10,
            1, // Constant
            0,
            0,
            0,
            5,
            6, // Divide
            0, // Return
        ];

        let mut vm = VM::new();
        let val = vm.interpret(&instructions);
        unsafe { assert_eq!(val.val.i, 2) }
    }

    #[test]
    fn test_interpret_source() {
        let s = r#"
        10 + 10
        "#;
    }
}
