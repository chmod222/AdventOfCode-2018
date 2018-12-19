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

    let mut alu = cpu::Alu::new();

    alu.regs[0] = 0;
    let mut i = 0;

    while alu.regs[ireg as usize] < prog.len() as cpu::Word {
        let ip = alu.regs[ireg as usize] as usize;
        let op = prog[ip];

        let pre = alu.regs;
        
        match alu.eval(&op) {
            Ok(_) => {
                println!("ip={} {:?} {} {:?}", ip, pre, op, alu.regs);
            },

            Err(e) => {
                println!("ALU error: {:?}: {}", e, op);
            }
        }

        alu.regs[ireg as usize] += 1;
        i += 1;
    }

    // Transpiled to C and back to Rust:
    assert_eq!(transpiled(false), 3224);
    assert_eq!(transpiled(true), 32188416); // Actually too slow even transpiled
}

fn transpiled(p2: bool) -> usize {
    // Fairly direct translation from elf code to C  to Rust, heavy mutation
    // and not very sexy.

    let (mut r0, mut r1, mut r2, mut r3, mut r4, mut r5) = (p2 as usize, 0, 0, 0, 0, 0);

    r3 += 2;
	r3 *= r3;
	r3 *= 19;
	r3 *= 11;
	r5 = r5 + 7;
	r5 *= 22;
	r5 += 18;
	r3 += r5;

	if r0 != 0 {
		// part 2
		r5 = 27;
		r5 *= 28;
		r5 += 29;
		r5 *= 30;
		r5 *= 14;
		r5 *= 32;
		r3 += r5;
		r0 = 0;
	}

    r4 = 1;
    while r4 <= r3 {
        r2 = 1;

		while r2 <= r3 {
			if (r4 * r2) == r3 {
				r0 += r4;
			}

            r2 += 1
		}

        r4 += 1;
    }

    r0
}