impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<_> = s.split_whitespace().collect();
        match line.as_slice() {
            ["addx", param] => Ok(Instruction::Addx(
                param
                    .parse()
                    .map_err(|_| format!("invalid parameter in \"{}\"", s))?,
            )),
            ["noop"] => Ok(Instruction::Noop),
            _ => Err(format!("unknown instruction \"{}\"", s)),
        }
    }
}
