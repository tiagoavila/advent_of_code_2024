pub struct Computer {
    pub register_a: usize,
    pub register_b: usize,
    pub register_c: usize,
    instruction_pointer: usize,
    instructions: Vec<usize>,
    pub output: Vec<usize>,
}

impl Computer {
    pub fn new(
        register_a: usize,
        register_b: usize,
        register_c: usize,
        instructions: String,
    ) -> Computer {
        Computer {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            instructions: instructions
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect(),
            output: Vec::new(),
        }
    }

    pub fn new_from_input(input: &mut Vec<String>) -> Computer {
        let register_a = Self::get_register_from_input(input[0].clone(), "A");
        let register_b = Self::get_register_from_input(input[1].clone(), "B");
        let register_c = Self::get_register_from_input(input[2].clone(), "C");
        let instructions = input[4].replace("Program: ", "");

        Self::new(register_a, register_b, register_c, instructions)
    }

    pub fn execute(&mut self) {
        loop {
            if self.instruction_pointer >= self.instructions.len() {
                break;
            }

            let opcode: usize = self.instructions[self.instruction_pointer];
            let operand: usize = self.instructions[self.instruction_pointer + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => self,
            };

            if opcode != 3 {
                self.instruction_pointer += 2;
            }
        }
    }

    pub fn print_output(&self) -> String {
        self.output
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn get_register_from_input(input: String, register_name: &str) -> usize {
        let register = format!("Register {}: ", register_name);
        input.replacen(&register, "", 1).parse::<usize>().unwrap()
    }

    /// The adv instruction (opcode 0) performs division.
    /// The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.
    /// The result of the division operation is truncated to an integer and then written to the A register.
    fn adv(&mut self, operand: usize) -> &mut Computer {
        self.register_a = self.do_division(operand);
        self
    }

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
    /// the instruction's literal operand, then stores the result in register B.
    fn bxl(&mut self, operand: usize) -> &mut Computer {
        self.register_b = self.register_b ^ operand;
        self
    }

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    fn bst(&mut self, operand: usize) -> &mut Computer {
        let operand: usize = self.get_combo_operand_value(operand);
        let modulo_8 = operand % 8;
        self.register_b = modulo_8;
        self
    }

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer
    /// to the value of its literal operand; if this instruction jumps, the instruction
    /// pointer is not increased by 2 after this instruction.
    fn jnz(&mut self, operand: usize) -> &mut Computer {
        if self.register_a != 0 {
            self.instruction_pointer = operand;
        } else {
            self.instruction_pointer += 2;
        }

        self
    }

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B. (For legacy reasons, this instruction reads an
    /// operand but ignores it.)
    fn bxc(&mut self, _: usize) -> &mut Computer {
        self.register_b = self.register_b ^ self.register_c;
        self
    }

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    /// then outputs that value. (If a program outputs multiple values,
    /// they are separated by commas.)
    fn out(&mut self, operand: usize) -> &mut Computer {
        let operand: usize = self.get_combo_operand_value(operand);
        let modulo_8 = operand % 8;
        self.output.push(modulo_8);
        self
    }

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that
    /// the result is stored in the B register. (The numerator is still read from the A
    /// register.)
    fn bdv(&mut self, operand: usize) -> &mut Computer {
        self.register_b = self.do_division(operand);
        self
    }

    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that
    /// the result is stored in the C register. (The numerator is still read from the A
    /// register.)
    fn cdv(&mut self, operand: usize) -> &mut Computer {
        self.register_c = self.do_division(operand);
        self
    }

    fn get_combo_operand_value(&self, operand: usize) -> usize {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => operand,
        }
    }

    fn do_division(&mut self, operand: usize) -> usize {
        let operand: usize = self.get_combo_operand_value(operand);
        let denominator = (1 << operand) as usize;
        let division = self.register_a / denominator;
        division
    }
}
