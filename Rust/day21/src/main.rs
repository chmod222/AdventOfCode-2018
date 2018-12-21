use shared::cpu;
use shared::input;

fn parse_instr(s: &str) -> Option<cpu::Opcode> {
    let mut p = s.split(" ");

    let mnem = p.next()?;
    let a = p.next()?.parse().ok()?;
    let b = p.next()?.parse().ok()?;
    let c = p.next()?.parse().ok()?;

    Some(cpu::Opcode::build(cpu::Mnemonic::from_str(mnem)?, a, b, c)?)
}

fn main() {
    let prog = input::read_stdin_lines().expect("could not lock stdin");
    
    let ireg: cpu::Register = prog[0].split(" ").nth(1).unwrap().parse().ok().unwrap();
    let prog = &prog[1..].iter().filter_map(|l| parse_instr(&l)).collect::<Vec<_>>();

    run_with(ireg, 0, prog);
}

use std::collections::HashSet;

fn run_with(ireg: cpu::Register, r0: cpu::Word, prog: &[cpu::Opcode]) {
    let mut alu = cpu::Alu::new();
    
    alu.regs[0] = r0;

    let mut reqs = HashSet::new();    
    let mut last = 0;

    while alu.regs[ireg as usize] < prog.len() as cpu::Word {
        let ip = alu.regs[ireg as usize] as usize;
        let op = prog[ip];
       
        match alu.eval(&op) {
            Ok(_) => {
                if ip == 28 {
                    let target = alu.regs[1];

                    if !reqs.contains(&target) {
                        if reqs.len() == 0 {
                            println!("Part 1: {}", target);
                        }

                        reqs.insert(target);

                        last = target;
                    } else {
                        println!("Part 2: {}", last);
                        break;
                    }
                }
            },

            Err(e) => {
                println!("ALU error: {:?}: {}", e, op);
            }
        }

        alu.regs[ireg as usize] += 1;
    }
}