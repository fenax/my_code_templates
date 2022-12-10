#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(Int),
}

struct InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    instr: &'a mut It,
    wait: Int,
    x: Int,
    next_x: Int,
}

impl<'a, It> Iterator for InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        if self.wait <= 0 {
            self.x = self.next_x;
            match self.instr.next() {
                Some(Instruction::Noop) => Some(self.x),
                Some(Instruction::Addx(val)) => {
                    self.next_x += val;
                    self.wait = 1;
                    Some(self.x)
                }
                None => None,
            }
        } else {
            self.wait -= 1;
            Some(self.x)
        }
    }
}

fn iter_cycles<'a, It>(iter: &'a mut It) -> InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    InstructionIter {
        instr: iter,
        wait: 0,
        x: 1,
        next_x: 1,
    }
}
