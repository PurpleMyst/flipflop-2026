use std::{collections::HashMap, fmt::Display};

enum Instruction {
    Load {
        val: u16,
        dest_reg: usize,
    },
    Copy {
        src_reg: usize,
        dest_reg: usize,
    },
    Add {
        src_reg1: usize,
        src_reg2: usize,
        dest_reg: usize,
    },
    Sub {
        src_reg1: usize,
        src_reg2: usize,
        dest_reg: usize,
    },
    Mul {
        src_reg1: usize,
        src_reg2: usize,
        dest_reg: usize,
    },
    Mod {
        src_reg1: usize,
        src_reg2: usize,
        dest_reg: usize,
    },
    Inc {
        reg: usize,
    },
    Dec {
        reg: usize,
    },
    Jmp {
        label: usize,
    },
    Jz {
        reg: usize,
        label: usize,
    },
    Jnz {
        reg: usize,
        label: usize,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Load { val, dest_reg } => write!(f, "mov {val} -> r{dest_reg}"),
            Instruction::Copy { src_reg, dest_reg } => write!(f, "mov r{src_reg} -> r{dest_reg}"),
            Instruction::Add {
                src_reg1,
                src_reg2,
                dest_reg,
            } => write!(f, "add r{src_reg1} + r{src_reg2} -> r{dest_reg}"),
            Instruction::Sub {
                src_reg1,
                src_reg2,
                dest_reg,
            } => write!(f, "sub r{src_reg1} - r{src_reg2} -> r{dest_reg}"),
            Instruction::Mul {
                src_reg1,
                src_reg2,
                dest_reg,
            } => write!(f, "mul r{src_reg1} * r{src_reg2} -> r{dest_reg}"),
            Instruction::Mod {
                src_reg1,
                src_reg2,
                dest_reg,
            } => write!(f, "mod r{src_reg1} % r{src_reg2} -> r{dest_reg}"),
            Instruction::Inc { reg } => write!(f, "inc r{reg}"),
            Instruction::Dec { reg } => write!(f, "dec r{reg}"),
            Instruction::Jmp { label } => write!(f, "jmp {label}"),
            Instruction::Jz { reg, label } => write!(f, "jz  r{reg} {label}"),
            Instruction::Jnz { reg, label } => write!(f, "jnz r{reg} {label}"),
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (program, labels) = parse_program();

    exec_program(&program, &labels, 0, 0).unwrap()
}

fn parse_program() -> (Vec<Instruction>, HashMap<usize, usize>) {
    let mut program = Vec::new();
    let mut labels = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let mut chunks = line.as_bytes().chunks_exact(2);
        if chunks.next().unwrap() != b"ba" {
            let label = chunks.count();
            labels.insert(label, program.len());
            continue;
        }
        let mut nas = || chunks.by_ref().take_while(|c| c == b"na").count();
        let instr = nas();
        program.push(match instr {
            0 => Instruction::Load {
                val: nas().try_into().unwrap(),
                dest_reg: nas(),
            },
            1 => Instruction::Copy {
                src_reg: nas(),
                dest_reg: nas(),
            },
            2 => Instruction::Add {
                src_reg1: nas(),
                src_reg2: nas(),
                dest_reg: nas(),
            },
            3 => Instruction::Sub {
                src_reg1: nas(),
                src_reg2: nas(),
                dest_reg: nas(),
            },
            4 => Instruction::Mul {
                src_reg1: nas(),
                src_reg2: nas(),
                dest_reg: nas(),
            },
            5 => Instruction::Mod {
                src_reg1: nas(),
                src_reg2: nas(),
                dest_reg: nas(),
            },
            6 => Instruction::Inc { reg: nas() },
            7 => Instruction::Dec { reg: nas() },
            8 => Instruction::Jmp { label: nas() },
            9 => Instruction::Jz {
                reg: nas(),
                label: nas(),
            },
            10 => Instruction::Jnz {
                reg: nas(),
                label: nas(),
            },

            _ => panic!("unimplemented instr {instr}"),
        });
    }

    (program, labels)
}

fn exec_program(program: &[Instruction], labels: &HashMap<usize, usize>, init_r0: u16, init_r1: u16) -> Option<u16> {
    let mut pc = 0;
    let mut regs = [0u16; 16];
    regs[0] = init_r0;
    regs[1] = init_r1;
    let mut instrs = 0;
    while pc < program.len() {
        instrs += 1;
        if instrs > 5_000_000 {
            return None;
        }

        match program[pc] {
            Instruction::Load { val, dest_reg } => regs[dest_reg] = val,
            Instruction::Copy { src_reg, dest_reg } => regs[dest_reg] = regs[src_reg],
            Instruction::Add {
                src_reg1,
                src_reg2,
                dest_reg,
            } => regs[dest_reg] = regs[src_reg1].wrapping_add(regs[src_reg2]),
            Instruction::Sub {
                src_reg1,
                src_reg2,
                dest_reg,
            } => regs[dest_reg] = regs[src_reg1].wrapping_sub(regs[src_reg2]),
            Instruction::Mul {
                src_reg1,
                src_reg2,
                dest_reg,
            } => regs[dest_reg] = regs[src_reg1].wrapping_mul(regs[src_reg2]),
            Instruction::Mod {
                src_reg1,
                src_reg2,
                dest_reg,
            } => {
                regs[dest_reg] = if regs[src_reg2] != 0 {
                    regs[src_reg1].wrapping_rem_euclid(regs[src_reg2])
                } else {
                    0
                }
            }
            Instruction::Inc { reg } => regs[reg] = regs[reg].wrapping_add(1),
            Instruction::Dec { reg } => regs[reg] = regs[reg].wrapping_sub(1),
            Instruction::Jmp { label } => {
                pc = labels[&label];
                continue;
            }
            Instruction::Jz { reg, label } => {
                if regs[reg] == 0 {
                    pc = labels[&label];
                    continue;
                }
            }
            Instruction::Jnz { reg, label } => {
                if regs[reg] != 0 {
                    pc = labels[&label];
                    continue;
                }
            }
        }
        pc += 1;
    }

    Some(regs[0])
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (program, labels) = parse_program();

    (0..16)
        .filter(|&r0| exec_program(&program, &labels, r0, 0).is_none())
        .count()
        * (100 / 16)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (program, labels) = parse_program();

    (0..16)
        .map(|r1| {
            (0..16)
                .filter(|&r0| exec_program(&program, &labels, r0, r1).is_none())
                .count()
                * (65536 / 16)
        })
        .sum::<usize>()
}
