#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub mod RISCV {
    use super::*;

    pub fn hex_iterator(buf: BufReader<File>) -> Box<dyn Iterator<Item = u32>> {
        Box::new(
            buf.lines().map(|l| l.unwrap().trim().to_string()).
            map(|l| hex::decode(if l.contains("0x") {&l[2..]} else {l.as_str()}).unwrap())
            .map(|v| (v[0] as u32) << 24 | (v[1] as u32) << 16 | (v[2] as u32) << 8 | (v[3] as u32))
        )
    }

    pub fn ins_collect(hexes: Box<dyn Iterator<Item = u32>>) -> Vec<(u8, String)> {
        hexes.map(|x| ((x & 0x7f) as u8, parse::opcodes().get(&(x & 0x7f)).unwrap_or_else(|| &(parse::bad_opcode as fn(u32) -> String))(x as u32))).collect()
    }

    pub fn op_trim<'a>(nobr: &'a Vec<(u8, String)>) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(nobr.iter().map(|u| &u.1))
    }

    pub fn add_labels(nobr: Vec<(u8, String)>) -> Vec<String> {
        let branchable = vec![99, 111];
        // contains the line numbers of the source and destination of the branches
        let branches: HashSet<(usize, i32)> = HashSet::from_iter(nobr.iter().enumerate().filter(|t| branchable.contains(&t.1.0))
            .map(|t| (t.0, t.0 as i32 + t.1.1[t.1.1.rfind(" ").unwrap() + 1..t.1.1.len()].parse::<i32>().expect(".") / 4)));
        // dbg!(&branches);
        let source: HashMap<usize, i32> = HashMap::from_iter(branches.iter().map(|t| (t.0, t.1)));
        let dest: HashMap<usize, i32> = HashMap::from_iter(branches.iter().map(|t| (t.1 as usize, t.1)));

        let wbr: Vec<String> = nobr.iter().enumerate().map(|ins| 
            if let Some(count) = dest.get(&ins.0) {
                format!("L{}: {}", count, ins.1.1)
            }
            else if let Some(count) = source.get(&ins.0) {
                format!("{} L{}", &ins.1.1[0..=ins.1.1.rfind(",").unwrap()], count)
            }
            else {
                ins.1.1.clone()
            }).collect();

        wbr
    }


    pub fn riscv() {
        let fr = File::open("src/io/riscv_input.txt").unwrap();
        let hexes = RISCV::hex_iterator(BufReader::new(fr));
        let nobr: Vec<(u8, String)> = RISCV::ins_collect(hexes);
        let wbr = RISCV::add_labels(nobr);
        
        dbg!(&wbr);
    }


    pub mod parse {
        use super::*;    
        pub fn opcodes() -> HashMap<u32, fn(u32) -> String>  {
            HashMap::from([
                (51, R as fn (u32) -> String),
                (19, I as fn (u32) -> String),
                (3, L as fn (u32) -> String),
                (103, jalr as fn (u32) -> String),
                (35, S as fn (u32) -> String),
                (99, B as fn (u32) -> String),
                (111, J as fn (u32) -> String),
                (55, lui as fn (u32) -> String),
                (23, auipc as fn (u32) -> String),
                (47, A as fn(u32) -> String)
                // (59, M as fn(u32) -> String)
            ])
        }

        pub fn bad_opcode(h: u32) -> String {
            format!("\n***{h} is an invalid opcode.***\n")
        }

        // fn M(h: u32) -> String {
        //     let rd = (h & 0xf80) >> 7;
        //     let f3 = (h & 0x7000) >> 12;
        //     let r1 = (h & 0xf8000) >> 15;
        //     let r2 = (h & 0x1f00000) >> 20;
        // }
        fn A(h: u32) -> String {
            let rd = (h & 0xf80) >> 7;
            let r1 = (h & 0xf8000) >> 15;
            let r2 = (h & 0x1f00000) >> 20;
            let f7 = (h & 0xfe000000) >> 25;

            let hm = &HashMap::from([
                (0, "amoadd.w"), (4, "amoswap.w"), (8, "lr.w"), (12, "sc.w"), (16, "amoxor.w"), (32, "amoor.w"), (48, "amoand.w"), 
                (64, "amomin.w"), (80, "amomax.w"), (96, "amominu.w")
            ]);

            format!("{} x{}, x{}, x{}", hm.get(&f7).copied().unwrap_or(format!["Invalid f7:{f7}"].as_str()), rd, r1, r2)
        }   

        fn R(h: u32) -> String {
            // let r = HashMap::from(_)
            // "..".to_string()
            let rd = (h & 0xf80) >> 7;
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let r2 = (h & 0x1f00000) >> 20;
            let f7 = (h & 0xfe000000) >> 25;

            let nhm1 = &HashMap::from([
                (0, "add"), (1, "sll"), (2, "slt"), (3, "sltu"), (4, "xor"), (5, "srl"), (6, "or"), (7, "and")
            ]);
            let nhm2 = &HashMap::from([
                (0, "sub"), (5, "sra")
            ]);
            let nhm3 = &HashMap::from([
                (0, "mul"), (1, "mulh"), (2, "mulhsu"), (3, "mulhu"), (4, "div"), (5, "divu"), (6, "rem"), (7, "remu")
            ]);

            let r_f = HashMap::from([
                (0, nhm1),
                (32, nhm2),
                (1, nhm3)
            ]);

            format!("{} x{}, x{}, x{}", r_f.get(&f7).copied().unwrap_or(&HashMap::from([(f3.clone(), format!["Invalid f7: {f7}"].as_str())])).get(&f3).copied().unwrap_or(format!["Invalid f3: {f3}"].as_str()), rd, r1, r2)
        }

        fn I(h: u32) -> String{
            let rd = (h & 0xf80) >> 7;
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let f6 = (h & 0xfc000000) >> 25;
            // 
            let imm1: i32 = (h & 0xfff00000) as i32 >> 20;

            //{0:"addi", 1:"slli", 4:"xori", 6:"ori", 7:"andi"}

            let i_f1 = HashMap::from([
                (0, "addi"), (1, "slli"), (4, "xori"), (6, "ori"), (7, "andi")
            ]);
            let nhm1 = &HashMap::from([
                (2, "slti"), (3, "sltiu"), (5, "srli")
            ]);
            let nhm2 = &HashMap::from([(5, "srai")]);
            let i_f2 = HashMap::from([
                (0, nhm1),
                (32, nhm2)
            ]);
            let imm2 : i32;

            let q = format!["Invalid f6: {f6}"];
            let inv_f3 = format!["Invalid f3: {f3}"];

            format!("{} x{}, x{}, {}", match i_f1.get(&f3).copied() {
                Some(s) => {imm2 = imm1; s},
                None => {
                    imm2 = imm1 % 1 << 6;
                    i_f2.get(&f6).copied().unwrap_or(&HashMap::from([(f3.clone(), q.as_str())])).get(&f3).copied().unwrap_or(inv_f3.as_str())
                }
            }, rd, r1, match imm2.cmp(&(1 << 11)) {
                Ordering::Less => imm2 as i32,
                _other => 1 << 11 - imm2 as i32
            })
        }

        fn L(h: u32) -> String {
            let rd = (h & 0xf80) >> 7;
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let imm: i32 = match ((h & 0xfff00000) >> 20).cmp(&(1 << 11)) {
                Ordering::Less => ((h & 0xfff00000) >> 20) as i32, 
                _other => 1 << 11 - ((h & 0xfff00000) >> 20)
            };
            let v = ["lb", "lh", "lw", "ld", "lbu", "lhu", "lwu", "ldu"];
            let inv_f3 = format!("Invalid f3: {f3}");

            format!("{} x{}, {}(x{})", if f3 < 8 {v[f3 as usize]} else {inv_f3.as_str()}, rd, imm, r1)
        }

        fn jalr(h: u32) -> String{
            let rd = (h & 0xf80) >> 7;
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let imm: i32 = match ((h & 0xfff00000) >> 20).cmp(&(1 << 11)) {
                Ordering::Less => ((h & 0xfffff000) >> 20) as i32, 
                _other => 1 << 11 - ((h & 0xfff00000) >> 20)
            };
            format!("jalr x{}, x{}, {}", rd, r1, imm)
        }

        fn S(h: u32) -> String {
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let r2 = (h & 0x1f00000) >> 20;
            let t = (h & 0xfe000000) >> 20 + (h & 0xf80) >> 7;

            let imm: i32 = match t.cmp(&(1 << 11)) {
                Ordering::Less => t as i32,
                _other => 1 << 11 - t
            };

            let v = ["sb", "sh", "sw", "sd"];
            let inv_f3 = format!("Invalid f3: {f3}");
            format!("{} x{}, {}(x{})", if f3 < 4 {v[f3 as usize]} else {inv_f3.as_str()}, r2, imm, r1)
        }

        fn B(h: u32) -> String {
            let f3 = (h & 0x7000) >> 12;
            let r1 = (h & 0xf8000) >> 15;
            let r2 = (h & 0x1f00000) >> 20;

            let t = ((h & 0xf00) >> 7) + ((h & 0x7e000000) >> 20) + ((h & 0x80) << 4) + ((h & 0x80000000) >> 19);
            let imm: i32 = match t.cmp(&(1 << 12)) {
                Ordering::Less => t as i32,
                _other => t as i32 - (1 << 13)
            };

            let b_f = HashMap::from([
                (0, "beq"), (1, "bne"), (4, "blt"), (5, "bge"), (6, "bltu"), (7, "bgeu")
            ]);

            format!("{} x{}, x{}, {}", b_f.get(&(f3)).copied().unwrap_or(format!("Invalid f3: {f3}").as_str()), r1, r2, imm)
        }

        fn J(h: u32) -> String {
            let rd = (h & 0xf80) >> 7;
            let t = (h & 0x7fe00000) >> 20 + (h & 0x100000) >> 9 + (h & 0xff000) + (h & 0x80000000) >> 11;

            let imm: i32 = match t.cmp(&(1 << 20)) {
                Ordering::Less => t as i32,
                _other => 1 << 20 - t
            };

            format!("jal x{}, {}", rd, imm)
        }

        fn lui(h: u32) -> String {
            let rd = (h & 0xf80) >> 7;
            let imm = (h & 0xfffff000) >> 12;

            format!("lui x{rd}, {imm}")
        }

        fn auipc(h: u32) -> String {
            let rd = (h & 0xf80) >> 7;
            let imm = (h & 0xfffff000) >> 12;

            format!("auipc x{rd}, {imm}")
        }
    }

}

fn main(){
    RISCV::riscv()
}

