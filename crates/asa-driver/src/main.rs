// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Guna: {} <fail.asa>", args[0]);
        process::exit(1);
    }
    let source = fs::read_to_string(&args[1]).expect("Gagal membaca fail sumber");
    let mut lexer = asa_lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Ralat lekser: {}", e);
            process::exit(1);
        }
    };
    let mut parser = asa_parser::Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Ralat sintaks: {}", e);
            process::exit(1);
        }
    };
    if let Err(e) = asa_codegen::generate(&ast, &mut std::io::stdout()) {
        eprintln!("Ralat penjanaan kod: {}", e);
        process::exit(1);
    }
}
