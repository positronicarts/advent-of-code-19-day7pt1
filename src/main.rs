#[derive(Debug)]
enum OpCode {
    ADD,
    MULTIPLY,
    INPUT,
    OUTPUT,
    JUMPIFNZ,
    JUMPIFZ,
    JUMPLT,
    JUMPEQ,
    EXIT
}

impl OpCode {
    fn from(chars: &mut Vec<char>) -> Self {

        let opcode = chars.pop().unwrap().to_digit(10).unwrap() + (chars.pop().unwrap_or('0').to_digit(10).unwrap()) * 10;

        match opcode {
            1 => OpCode::ADD,
            2 => OpCode::MULTIPLY,
            3 => OpCode::INPUT,
            4 => OpCode::OUTPUT,
            5 => OpCode::JUMPIFNZ,
            6 => OpCode::JUMPIFZ,
            7 => OpCode::JUMPLT,
            8 => OpCode::JUMPEQ,
            99 => OpCode::EXIT,
            x => panic!("Unrecognized opcode {}", x)
        }
    }
}

#[derive(Debug)]
enum ReferenceType {
    DIRECT,
    INDIRECT,
}

impl ReferenceType {
    fn from(c: char) -> Self {

        match c {
            '1' => ReferenceType::DIRECT,
            '0' => ReferenceType::INDIRECT,
            x => panic!("Unrecognized reference type {}", x)
        }
    }
}

#[derive(Default, Clone)]
struct Computer {
    memory: Vec<i64>,
    index: usize,
    instruction_chars: Vec<char>,
}

impl Computer {

    fn new_from_file(filename: &str) -> Self {
        Computer {
            memory: std::fs::read_to_string(filename).unwrap().split(',').map(|input| input.parse::<i64>().unwrap()).collect(),
            ..Default::default()
        }
    }

    fn get_next_value(&mut self) -> i64 {
        let val = match ReferenceType::from(self.instruction_chars.pop().unwrap_or('0')) {
            ReferenceType::INDIRECT => self.memory[self.memory[self.index] as usize],
            ReferenceType::DIRECT => self.memory[self.index],
        };
        self.index += 1;
        val
    }

    fn write(&mut self, val: i64) {
        let dest = self.memory[self.index] as usize;
        self.memory[dest] = val;
        self.index += 1;
    }

    fn get_instruction(&mut self) -> Vec<char> {
        let instruction = self.memory[self.index].to_string().chars().collect();
        self.index += 1;
        instruction
    }

    fn run(mut self, mut inputs: Vec<i64>) -> i64 {
        loop {
            self.instruction_chars = self.get_instruction();            
            let opcode = OpCode::from(&mut self.instruction_chars);
            println!("{:?}", opcode);

            match opcode {
                OpCode::ADD => {
                    let val = self.get_next_value() + self.get_next_value();
                    self.write(val);
                },
                OpCode::MULTIPLY => {
                    let val = self.get_next_value() * self.get_next_value();
                    self.write(val);
                },
                OpCode::INPUT => {
                    self.write(inputs.remove(0));
                },
                OpCode::OUTPUT => {
                    let v1 = self.get_next_value();
                    if v1 != 0 {
                        return v1;
                    }
                },
                OpCode::JUMPIFZ => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    if v1 == 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                },
                OpCode::JUMPIFNZ => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());                    
                    if v1 != 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                },
                OpCode::JUMPLT => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 < v2 {                    
                        1
                    } else {
                        0
                    });
                },
                OpCode::JUMPEQ => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 == v2 {
                        1
                    } else {
                        0
                    });
                },                                    
                OpCode::EXIT => {
                    break;
                },
            };
        }

        self.memory[0]
    }
}

fn main() {
    
    let computer = Computer::new_from_file("inputs.txt");
    let mut orderings = Vec::<Vec<i64>>::new();

    for ii in 0..5 {
        for jj in 0..5 {
            for kk in 0..5 {
                for ll in 0..5 {
                    for mm in 0..5 {
                        let phase_settings : Vec<i64> = vec![ii, jj, kk, ll, mm];
                        let set : std::collections::HashSet<_> = phase_settings.iter().collect();
                        if set.len() != 5 {
                            continue;
                        }

                        orderings.push(phase_settings);
                    }
                }
            }
        }
    }

    let max = orderings.into_iter().map(|mut ordering| {
        print!("Ordering {:?}", ordering);   
        let mut next_input = 0;     
        while !ordering.is_empty() {
            next_input = computer.clone().run(vec![ordering.remove(0).clone(), next_input]);
        }
        println!("-> {}", next_input);
        next_input
    }).max();

    println!("Max output {}", max.unwrap());
}
