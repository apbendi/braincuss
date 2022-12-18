pub struct Machine {
    memory: Vec<u8>,
    pointer: usize,
    program: Vec<char>,
    pc: usize,
}

impl Machine {
    pub fn with_memory_size_and_program(size: usize, program: &String) -> Self {
        Machine {
            memory: vec![0; size],
            pointer: 0,
            program: program.chars().collect(),
            pc: 0,
        }
    }

    pub fn read_memory(&self) -> u8 {
        self.memory[self.pointer]
    }

    pub fn write_memory(&mut self, c: u8) {
        self.memory[self.pointer] = c;
    }

    pub fn increment_memory(&mut self) {
        if self.memory[self.pointer] == 255 {
            self.memory[self.pointer] = 0;
        } else {
            self.memory[self.pointer] += 1;
        }
    }

    pub fn decrement_memory(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.memory[self.pointer] = 255
        } else {
            self.memory[self.pointer] -= 1;
        }
    }

    pub fn advance_pointer(&mut self) {
        self.pointer += 1;
        if self.pointer == self.memory.len() {
            self.pointer = 0;
        }
    }

    pub fn reverse_pointer(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.memory.len() - 1;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    pub fn jump(&mut self, destination: usize) {
        self.pc = destination;
    }

    pub fn instruction(&self) -> char {
        self.program[self.pc]
    }

    pub fn end_of_program(&self) -> bool {
        self.pc >= self.program.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() -> Machine {
        Machine::with_memory_size_and_program(50_000, &String::from("+-"))
    }

    #[test]
    fn init_machine() {
        let machine = set_up();
        assert_eq!(machine.memory.len(), 50_000);
        assert_eq!(machine.program[0], '+');
        assert_eq!(machine.program[1], '-');
        assert_eq!(machine.pc, 0);
        assert_eq!(machine.pointer, 0);
    }

    #[test]
    fn advance_pointer_test() {
        let mut machine = set_up();

        machine.advance_pointer();
        assert_eq!(machine.pointer, 1);

        machine.advance_pointer();
        assert_eq!(machine.pointer, 2);

        // test memory wraps around on
        for _ in 1..(machine.memory.len() - 1) {
            machine.advance_pointer();
        }

        assert_eq!(machine.pointer, 0);
    }

    #[test]
    fn reverse_pointer_test() {
        let mut machine = set_up();

        machine.reverse_pointer();
        assert_eq!(machine.pointer, 49_999);

        machine.reverse_pointer();
        assert_eq!(machine.pointer, 49_998);
    }
    #[test]
    fn increment_memory_test() {
        let mut machine = set_up();

        machine.increment_memory();
        assert_eq!(machine.memory[0], 1);

        machine.increment_memory();
        assert_eq!(machine.memory[0], 2);

        // test overflow on increment
        for _ in 1..255 {
            machine.increment_memory();
        }
        assert_eq!(machine.memory[0], 0);
    }

    #[test]
    fn decrement_memory_test() {
        let mut machine = set_up();

        machine.decrement_memory();
        assert_eq!(machine.memory[0], 255);

        machine.decrement_memory();
        assert_eq!(machine.memory[0], 254);
    }

    #[test]
    fn write_memory_test() {
        let mut machine = set_up();

        machine.write_memory(1);
        assert_eq!(machine.memory[0], 1);

        machine.write_memory(0);
        assert_eq!(machine.memory[0], 0);

        for n in 0..255 {
            machine.write_memory(n);
            assert_eq!(machine.memory[0], n);
        }
    }

    #[test]
    fn read_memory_test() {
        let mut machine = set_up();

        machine.increment_memory();
        assert_eq!(machine.memory[0], machine.read_memory());

        machine.decrement_memory();
        assert_eq!(machine.memory[0], machine.read_memory());
        machine.decrement_memory();
        assert_eq!(machine.memory[0], machine.read_memory());

        machine.advance_pointer();
        assert_eq!(machine.memory[1], machine.read_memory());

        for n in 0..1000 {
            machine.increment_memory();
            assert_eq!(machine.memory[1], machine.read_memory());
        }
    }

    #[test]
    fn combined_memory_operations() {
        let mut machine = set_up();

        machine.increment_memory(); // memory[0] = 1
        machine.advance_pointer();  // memory[1]
        machine.decrement_memory(); // memory[1] = 255
        machine.reverse_pointer();  // memory[0]
        machine.reverse_pointer();  // memory[49_999]
        machine.write_memory(16);   // memory [49_999] = 16

        assert_eq!(machine.memory[0], 1);
        assert_eq!(machine.memory[1], 255);
        assert_eq!(machine.memory[49_999], 16);
    }

    #[test]
    fn advance_pc_test() {
        let mut machine = set_up();

        machine.advance_pc();
        assert_eq!(machine.pc, 1);

        machine.advance_pc();
        assert_eq!(machine.pc, 2);

        machine.advance_pc();
        assert_eq!(machine.pc, 3);
    }

    #[test]
    fn jump_test() {
        let mut machine = set_up();

        machine.jump(2);
        assert_eq!(machine.pc, 2);

        machine.jump(0);
        assert_eq!(machine.pc, 0);

        machine.jump(10);
        assert_eq!(machine.pc, 10);
    }

    #[test]
    fn instruction_test() {
        let mut machine = set_up();

        machine.advance_pc();
        assert_eq!(machine.instruction(), '-');

        machine.jump(0);
        assert_eq!(machine.instruction(), '+');

        machine.advance_pc();
        assert_eq!(machine.instruction(), '-');
    }

    #[test]
    fn end_of_program_test() {
        let mut machine = set_up();

        machine.advance_pc();
        assert!(!machine.end_of_program());

        machine.advance_pc();
        assert!(machine.end_of_program());

        machine.jump(0);
        assert!(!machine.end_of_program());

        machine.jump(10);
        assert!(machine.end_of_program());
    }
}