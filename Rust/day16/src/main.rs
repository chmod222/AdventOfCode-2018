use shared::cpu::*;

/*
Lots of part 1 exclusive code here

#[derive(Debug)]
struct TestCase {
    before: RegisterState,
    after: RegisterState,

    raw_instruction: [Word; 4]
}

use regex::Regex;
use lazy_static::*;

// Too boilerplatey
macro_rules! int_match {
    ($group:expr, $matches:expr) => {
        $matches.get($group).unwrap().as_str().parse().unwrap()
    }
}

fn read_testcase(test: &[String]) -> TestCase {
    lazy_static! {
        static ref PAT_BEFORE: Regex = Regex::new(r"^Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
        static ref PAT_AFTER: Regex = Regex::new(r"^After:\s+\[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
        static ref PAT_INSTR: Regex = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    }

    let m_bef = PAT_BEFORE.captures(&test[0]).unwrap();
    let m_aft = PAT_AFTER.captures(&test[2]).unwrap();
    let m_instr = PAT_INSTR.captures(&test[1]).unwrap();

    TestCase {
        before: [int_match!(1, m_bef), int_match!(2, m_bef), int_match!(3, m_bef), int_match!(4, m_bef)],
        after: [int_match!(1, m_aft), int_match!(2, m_aft), int_match!(3, m_aft), int_match!(4, m_aft)],

        raw_instruction: [int_match!(1, m_instr), int_match!(2, m_instr), int_match!(3, m_instr), int_match!(4, m_instr)]
    }
}

use std::collections::{HashSet, HashMap};
*/

fn main() {
    /* Part 1 & 2.1: Decoding and deciphering the instruction set

    let testcases = shared::input::read_stdin_lines().expect("could not lock stdin");
    let testcases = testcases.chunks(4).map(read_testcase).collect::<Vec<_>>();

    let mut alu = Alu::new();

    let mut knowns = HashMap::new();

    loop {
        let mut ops_matched = HashMap::new();

        for testcase in &testcases {
            if knowns.contains_key(&testcase.raw_instruction[0]) {
                continue;
            }

            for op in Opcode::try_all(&testcase.raw_instruction) {
                alu.set_registers(testcase.before);

                match alu.eval(&op) {
                    Ok(()) => if alu.regs == testcase.after {
                        let matches = ops_matched.entry(op.mnemonic).or_insert(HashSet::new());
                        matches.insert(testcase.raw_instruction[0]);
                    }

                    _ => {
                    }
                }
            }
        }

        if ops_matched.len() > 0 {
            for (mnem, matched) in ops_matched {
                if matched.len() == 1 {
                    knowns.insert(*matched.iter().next().unwrap(), mnem);
                }
            }
        } else {
            break;
        }
    }

    println!("Part 2.1: Decoded Instructions: {:?}", knowns);
    */
    let program = shared::input::read_stdin_lines().expect("could not lock stdin");
    let program = program.iter().filter_map(|raw| {
        let mut parts = raw.split(" ");

        let raw = [
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?,
            parts.next()?.parse().ok()?];

        Opcode::decode(&raw)
    }).collect::<Vec<Opcode>>();

    let mut alu = Alu::new();

    for op in program {
        match alu.eval(&op) {
            Ok(_) => {

            },

            Err(e) => {
                println!("ALU error: {:?}", e);
            }
        }
    }

    println!("Part 2.2: {}", alu.regs[0]);
}
