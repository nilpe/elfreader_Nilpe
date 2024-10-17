use std::env;
use std::fs::File;
use std::io::Read;

struct ELFheader {
    e_ident: [u8; 16], // ELF magic number
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}
fn parse64(buf: [u8; 64]) -> ELFheader {
    let mut header = ELFheader {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    header.e_ident.copy_from_slice(&buf[0..16]);
    header.e_type = u16::from_le_bytes([buf[16], buf[17]]);
    header.e_machine = u16::from_le_bytes([buf[18], buf[19]]);
    header.e_version = u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]);
    header.e_entry = u64::from_le_bytes([
        buf[24], buf[25], buf[26], buf[27], buf[28], buf[29], buf[30], buf[31],
    ]);
    header.e_phoff = u64::from_le_bytes([
        buf[32], buf[33], buf[34], buf[35], buf[36], buf[37], buf[38], buf[39],
    ]);
    header.e_shoff = u64::from_le_bytes([
        buf[40], buf[41], buf[42], buf[43], buf[44], buf[45], buf[46], buf[47],
    ]);
    header.e_flags = u32::from_le_bytes([buf[48], buf[49], buf[50], buf[51]]);
    header.e_ehsize = u16::from_le_bytes([buf[52], buf[53]]);
    header.e_phentsize = u16::from_le_bytes([buf[54], buf[55]]);
    header.e_phnum = u16::from_le_bytes([buf[56], buf[57]]);
    header.e_shentsize = u16::from_le_bytes([buf[58], buf[59]]);
    header.e_shnum = u16::from_le_bytes([buf[60], buf[61]]);
    header.e_shstrndx = u16::from_le_bytes([buf[62], buf[63]]);
    return header;
}
fn parse32(buf: [u8; 64]) -> ELFheader {
    let mut header = ELFheader {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    header.e_ident.copy_from_slice(&buf[0..16]);
    header.e_type = u16::from_le_bytes([buf[16], buf[17]]);
    header.e_machine = u16::from_le_bytes([buf[18], buf[19]]);
    header.e_version = u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]);
    header.e_entry = u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]).into();
    header.e_phoff = u32::from_le_bytes([buf[28], buf[29], buf[30], buf[31]]).into();
    header.e_shoff = u32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]).into();
    header.e_flags = u32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]);
    header.e_ehsize = u16::from_le_bytes([buf[40], buf[41]]);
    header.e_phentsize = u16::from_le_bytes([buf[42], buf[43]]);
    header.e_phnum = u16::from_le_bytes([buf[44], buf[45]]);
    header.e_shentsize = u16::from_le_bytes([buf[46], buf[47]]);
    header.e_shnum = u16::from_le_bytes([buf[48], buf[49]]);
    header.e_shstrndx = u16::from_le_bytes([buf[50], buf[51]]);
    return header;
}
fn parse_elfheader(file: &mut File) -> ELFheader {
    let buffer = &mut [0u8; 64];
    let e = file.read_exact(buffer);
    if e.is_err() {
        panic!("ファイル読み込みしくじったよ");
    }
    if &buffer[0..4] != b"\x7fELF" {
        panic!("ELFファイルじゃないよ");
    }
    let header = if buffer[4] == 1 {
        parse32(*buffer)
    } else if buffer[4] == 2 {
        parse64(*buffer)
    } else {
        panic!("ELFファイルじゃないよ");
    };
    return header;
}
fn print_elfheader(header: &ELFheader) {
    println!("e_ident(マジックナンバー): {:?}", header.e_ident);
    println!("e_type(ファイルの種類): {}", header.e_type);
    println!("e_machine(命令セット): 0x{:x}", header.e_machine);
    println!("e_version(ELFのバージョン): {}", header.e_version);
    println!("e_entry(エントリーポイント): 0x{:x}", header.e_entry);
    println!("e_phoff(プログラムヘッダの始点): {}(bytes)", header.e_phoff);
    println!("e_shoff(セクションヘッダの始点): {}(bytes)", header.e_shoff);
    println!("e_flags(フラグ): {}", header.e_flags);
    println!("e_ehsize(ELFヘッダサイズ): {}(bytes)", header.e_ehsize);
    println!(
        "e_phentsize(プログラムヘッダサイズ): {}(bytes)",
        header.e_phentsize
    );
    println!("e_phnum(プログラムヘッダの数): {}", header.e_phnum);
    println!(
        "e_shentsize(セクションヘッダのサイズ): {}(bytes)",
        header.e_shentsize
    );
    println!("e_shnum(セクションヘッダの数): {}", header.e_shnum);
    println!(
        "e_shstrndx(文字列テーブルセクションのインデックス): {}",
        header.e_shstrndx
    );
}
fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ELF file>", args[0]);
        return;
    }
    let mut file = File::open(&args[1]).unwrap();
    let header = parse_elfheader(&mut file);
    print_elfheader(&header);
    return;
}
