use std::fs;

enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl From<i32> for OpCode {
    fn from(item: i32) -> OpCode {
        use OpCode::*;

        match item {
            1 => Add,
            2 => Multiply,
            99 => Halt,
            _ => unreachable!()
        }
    }
}

pub fn compute() -> i32 {
    let input = fs::read_to_string("input/input2.txt")
        .expect("Something went wrong reading the input");

    let mut opcodes: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    opcodes[1] = 12;
    opcodes[2] = 2;
    let mut i: usize = 0;
    loop {
        match opcodes[i].into() {
            OpCode::Add => {
                let i1 = opcodes[i + 1] as usize;
                let i2 = opcodes[i + 2] as usize;
                let i3 = opcodes[i + 3] as usize;
                opcodes[i3] = opcodes[i1] + opcodes[i2];
            },
            OpCode::Multiply => {
                let i1 = opcodes[i + 1] as usize;
                let i2 = opcodes[i + 2] as usize;
                let i3 = opcodes[i + 3] as usize;
                opcodes[i3] = opcodes[i1] * opcodes[i2];
            },
            OpCode::Halt => break
        };

        i += 4;
    }

    return opcodes[0];
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compute() {
        assert_eq!(super::compute(), 3850704);
    }
}
