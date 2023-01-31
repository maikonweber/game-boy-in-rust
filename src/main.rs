use std::fs::File;
use std::io::Read;

pub struct Memory {
    ram : [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory { ram: [0; 65536] };
        mem
    }

    pub fn write_byte(&mut self, address: u16, value: u8 ) {
        self.ram[address as usize] = value
    }


    pub fn read_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn from_rom_file(&mut self, rom_file: &[u8]) {
        let mut i:u16 = 0x0000;

        for &byte in rom_file.iter() {
            self.write_byte(0x0000, byte); 
            i += 1
        }
       
    }
}

pub struct CPU {
    a: u8,
    b: u8,
    d: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16
}

impl  CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            a: 0,
            b: 0,
            d: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0 
        };
        cpu
    }

    pub fn run_instruction(&mut self, mem: &mut Memory) {
        if self.pc < 256 {    

            println!("Byte on {} : {}", self.pc as u16, mem.read_byte(self.pc) as u16);
            self.pc += 1;
        }
    }
}

// Fuctions 


fn main() {
    let mut f:File = File::open("OMS/db-pg-mcd.bin").unwrap();
    let mut rom_file:Vec<u16> = Vec::<u16>::new();

    
    f.read_to_end(&mut rom_file);

    let mut mem = Memory:::new();
    mem.from_rom_file(&rom_file);


    loop {
        cpu.run.instrunction(&mut mem);
    }
}
