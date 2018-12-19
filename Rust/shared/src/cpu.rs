pub type Register = i8;
pub type Word = i32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mnemonic {
    Addr = 6,
    Addi = 9,
    Mulr = 14,
    Muli = 1,
    Banr = 2,
    Bani = 3,
    Borr = 12,
    Bori = 0,
    Setr = 5,
    Seti = 8,
    Gtir = 4,
    Gtri = 15,
    Gtrr = 13,
    Eqir = 7,
    Eqri = 11,
    Eqrr = 10
}

impl Mnemonic {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "addr" => Some(Mnemonic::Addr),
            "addi" => Some(Mnemonic::Addi),
            "mulr" => Some(Mnemonic::Mulr),
            "muli" => Some(Mnemonic::Muli),
            "banr" => Some(Mnemonic::Banr),
            "bani" => Some(Mnemonic::Bani),
            "borr" => Some(Mnemonic::Borr),
            "bori" => Some(Mnemonic::Bori),
            "setr" => Some(Mnemonic::Setr),
            "seti" => Some(Mnemonic::Seti),
            "gtir" => Some(Mnemonic::Gtir),
            "gtri" => Some(Mnemonic::Gtri),
            "gtrr" => Some(Mnemonic::Gtrr),
            "eqir" => Some(Mnemonic::Eqir),
            "eqri" => Some(Mnemonic::Eqri),
            "eqrr" => Some(Mnemonic::Eqrr),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Slot {
    Reg(Register),
    Immediate(Word)
}

#[derive(Copy, Clone, Debug)]
pub struct Opcode {
    mnemonic: Mnemonic,

    a: Slot,
    b: Slot,
    c: Slot
}

impl std::fmt::Display for Slot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Slot::Reg(r) => write!(fmt, "{}", r),
            Slot::Immediate(i) => write!(fmt, "{}", i)
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mnem = match self.mnemonic {
            Mnemonic::Addr => "addr",
            Mnemonic::Addi => "addi",
            Mnemonic::Mulr => "mulr",
            Mnemonic::Muli => "muli",
            Mnemonic::Banr => "banr",
            Mnemonic::Bani => "bani",
            Mnemonic::Borr => "borr",
            Mnemonic::Bori => "bori",
            Mnemonic::Setr => "setr",
            Mnemonic::Seti => "seti",
            Mnemonic::Gtir => "gtir",
            Mnemonic::Gtri => "gtri",
            Mnemonic::Gtrr => "gtrr",
            Mnemonic::Eqir => "eqir",
            Mnemonic::Eqri => "eqri",
            Mnemonic::Eqrr => "eqrr"
        };

        write!(fmt, "{} {} {} {}", mnem, self.a, self.b, self.c)
    }
}

impl Opcode {
    pub fn build(mnemonic: Mnemonic, a: Word, b: Word, c: Word) -> Option<Self> {
        Opcode::decode(&[mnemonic as Word, a, b, c])
    }

    pub fn decode(raw: &[Word]) -> Option<Self> {
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
                5 => make_rr(Mnemonic::Setr, a as Register, 0 as Register, c as Register),
                6 => make_rr(Mnemonic::Addr, a as Register, b as Register, c as Register),
                7 => make_ir(Mnemonic::Eqir, a as Word, b as Register, c as Register),
                8 => make_ir(Mnemonic::Seti, a as Word, 0 as Register, c as Register),
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
    pub fn try_all<'a>(raw: &'a [Word]) -> OpcodeIterator<'a> {
        OpcodeIterator {
            raw: raw,
            state: 0
        }
    }
}

// Part 1 only
pub struct OpcodeIterator<'a> {
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

#[derive(Debug)]
pub enum AluError {
    InvalidRegister,
    CannotStoreToImmediate
}

// Helper macros to simply loading and storing registers

macro_rules! load {
    ($slot:expr, $regs:expr) => {
        match $slot {
            Slot::Immediate(i) => Ok(i),
            Slot::Reg(r) => if r as usize > $regs.len() {
                println!("AAAA, {} sux", r);
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

pub struct Alu {
    pub regs: RegisterState
}

pub type AluFunc<'a> = &'a dyn Fn(Word, Word) -> Word;

pub const REGISTER_COUNT: usize = 6;
pub type RegisterState = [Word; REGISTER_COUNT];

impl Alu {
    pub fn new() -> Self {
        Alu {
            regs: [Default::default(); REGISTER_COUNT]
        }
    }

    pub fn eval(&mut self, opcode: &Opcode) -> Result<(), AluError> {
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
    pub fn set_registers(&mut self, new: RegisterState) {
        self.regs = new;
    }
}