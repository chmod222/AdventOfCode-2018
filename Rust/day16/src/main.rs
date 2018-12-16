type Register = i8;
type Word = i32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Mnemonic {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr
}

#[derive(Copy, Clone, Debug)]
enum Slot {
    Reg(Register),
    Immediate(Word)
}

#[derive(Copy, Clone, Debug)]
struct Opcode {
    mnemonic: Mnemonic,

    a: Slot,
    b: Slot,
    c: Slot
}

impl Opcode {
    fn decode(raw: &[Word]) -> Option<Self> {
        if raw.len() != 4 {
            None
        } else {
            let make = |m, a, b, c| Some(Opcode { mnemonic: m, a, b, c });
            let make_rr = |m, a, b, c| make(m, Slot::Reg(a), Slot::Reg(b), Slot::Reg(c));
            let make_ri = |m, a, b, c| make(m, Slot::Reg(a), Slot::Immediate(b), Slot::Reg(c));
            let make_ir = |m, a, b, c| make(m, Slot::Immediate(a), Slot::Reg(b), Slot::Reg(c));

            let (a, b, c) = (raw[1], raw[2], raw[3]);

            match raw[0] {
                0 => make_ri(Mnemonic::Bori, a as Register, b as Word, c as Register),
                1 => make_ri(Mnemonic::Muli, a as Register, b as Word, c as Register),
                2 => make_rr(Mnemonic::Banr, a as Register, b as Register, c as Register),
                3 => make_ri(Mnemonic::Bani, a as Register, b as Word, c as Register),
                4 => make_ir(Mnemonic::Gtir, a as Word, b as Register, c as Register),
                5 => make_rr(Mnemonic::Setr, a as Register, b as Register, c as Register),
                6 => make_rr(Mnemonic::Addr, a as Register, b as Register, c as Register),
                7 => make_ir(Mnemonic::Eqir, a as Word, b as Register, c as Register),
                8 => make_ir(Mnemonic::Seti, a as Word, b as Register, c as Register),
                9 => make_ri(Mnemonic::Addi, a as Register, b as Word, c as Register),
                10 => make_rr(Mnemonic::Eqrr, a as Register, b as Register, c as Register),
                11 => make_ri(Mnemonic::Eqri, a as Register, b as Word, c as Register),
                12 => make_rr(Mnemonic::Borr, a as Register, b as Register, c as Register),
                13 => make_rr(Mnemonic::Gtrr, a as Register, b as Register, c as Register),
                14 => make_rr(Mnemonic::Mulr, a as Register, b as Register, c as Register),
                15 => make_ri(Mnemonic::Gtri, a as Register, b as Word, c as Register),

                _ => None
            }
        }
    }

    // Iterating through all opcodes, used for deciphering the instruction set in part 1
    fn try_all<'a>(raw: &'a [Word]) -> OpcodeIterator<'a> {
        OpcodeIterator {
            raw: raw,
            state: 0
        }
    }
}

// Part 1 only
struct OpcodeIterator<'a> {
    raw: &'a [Word],
    state: usize
}

// Part 1 only
impl<'a> Iterator for OpcodeIterator<'a> {
    type Item = Opcode;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b, c) = (self.raw[1], self.raw[2], self.raw[3]);

        let next = Opcode::decode(&[self.state as Word, a, b, c]);

        if next.is_some() {
            self.state += 1;
        }

        next
    }
}

type RegisterState = [Word; 4];


#[derive(Debug)]
enum AluError {
    InvalidRegister,
    CannotStoreToImmediate
}

// Helper macros to simply loading and storing registers

macro_rules! load {
    ($slot:expr, $regs:expr) => {
        match $slot {
            Slot::Immediate(i) => Ok(i),
            Slot::Reg(r) => if r as usize > $regs.len() {
                Err(AluError::InvalidRegister)
            } else {
                Ok($regs[r as usize])
            }

        }
    }
}

macro_rules! store {
    ($slot:expr, $val:expr, $regs:expr) => {
        match $slot {
            Slot::Immediate(_i) => Err(AluError::CannotStoreToImmediate),
            Slot::Reg(r) => if r as usize > $regs.len() {
                Err(AluError::InvalidRegister)
            } else {
                $regs[r as usize] = $val;

                Ok(())
            }

        }
    }
}

struct Alu {
    regs: RegisterState
}

type AluFunc<'a> = &'a dyn Fn(Word, Word) -> Word;

impl Alu {
    fn new() -> Self {
        Alu {
            regs: [Default::default(); 4]
        }
    }

    fn eval(&mut self, opcode: &Opcode) -> Result<(), AluError> {
        let r = &mut self.regs;
        let f = match opcode.mnemonic {
            Mnemonic::Addi | Mnemonic::Addr => &(|a, b| a + b) as AluFunc,
            Mnemonic::Muli | Mnemonic::Mulr => &(|a, b| a * b) as AluFunc,
            Mnemonic::Bani | Mnemonic::Banr => &(|a, b| a & b) as AluFunc,
            Mnemonic::Bori | Mnemonic::Borr => &(|a, b| a | b) as AluFunc,
            Mnemonic::Seti | Mnemonic::Setr => &(|a, _| a) as AluFunc,
            Mnemonic::Gtrr | Mnemonic::Gtri | Mnemonic::Gtir => &(|a, b| (a > b) as Word) as AluFunc,
            Mnemonic::Eqrr | Mnemonic::Eqri | Mnemonic::Eqir => &(|a, b| (a == b) as Word) as AluFunc,
        };

        Alu::exec(r, opcode.a, opcode.b, opcode.c, f)
    }

    fn exec(regs: &mut RegisterState, a: Slot, b: Slot, c: Slot, op: AluFunc) -> Result<(), AluError> {
        store!(c, op(load!(a, regs)?, load!(b, regs)?), regs)
    }

    // Part 1 only, force register override
    fn set_registers(&mut self, new: RegisterState) {
        self.regs = new;
    }
}

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
