use std::fs::canonicalize;
use std::path::PathBuf;

use clap;
use clap::{App, Arg};

use parser::Parser;
use scanner::Scanner;
use tab::Tab;
use trace::Trace;

mod trace;
mod scanner;
mod parser;
mod tab;
mod dfa;

fn main() {
    let matches = App::new("CocoRust")
        .version("0.1")
        .author("samw3 <sam@p1xl.com>")
        .arg(Arg::with_name("module")
            .long("module")
            .value_name("moduleName")
            .help("a module name for parser")
            .takes_value(true))
        .arg(Arg::with_name("frames")
            .long("frames")
            .value_name("frameFilesDirectory")
            .help("directory location of .frame files")
            .takes_value(true))
        .arg(Arg::with_name("trace")
            .long("trace")
            .value_name("traceString")
            .help("Valid characters in the trace string:\n  \
                              A  trace automaton\n  \
                              F  list first/follow sets\n  \
                              G  print syntax graph\n  \
                              I  trace computation of first sets\n  \
                              J  list ANY and SYNC sets\n  \
                              P  print statistics\n  \
                              S  list symbol table\n  \
                              X  list cross reference table")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("outputDirectory")
            .help("directory where generated files will be written")
            .takes_value(true))
        .arg(Arg::with_name("grammarFile")
            .help(".ATG Grammar file path")
            .required(true)
            .index(1))
        .after_help("Scanner.frame and Parser.frame files needed in ATG directory or in a directory specified in the --frames option.")
        .get_matches();

    let module_name = matches.value_of("module");

    let mut frame_path = None;
    if let Some(frame_dir) = matches.value_of("frames") {
        frame_path = canonicalize(PathBuf::from(frame_dir)).ok();
    }

    let ddt_string = matches.value_of("trace");

    let mut out_path = None;
    if let Some(out_dir) = matches.value_of("output") {
        out_path = canonicalize(PathBuf::from(out_dir)).ok();
    }

    let src_name = some_or_die(matches.value_of("grammarFile"), "Missing grammar file");
    let src_path = ok_or_die(canonicalize(PathBuf::from(src_name)), "Cannot find grammar file.");
    let mut src_dir = PathBuf::from(&src_path);
    src_dir.pop();
    let src_dir = ok_or_die(canonicalize(PathBuf::from(&src_path)), "Cannot find source directory.");

    let scanner = Scanner::new(PathBuf::from(&src_path));
    let parser = Parser::new(
        scanner,
        Trace::new(PathBuf::from(&src_dir)),
        Tab::new(module_name, frame_path, ddt_string, out_path, &src_path, src_dir),
    );

    parser.parse();

    println!("{} error(s) detected", parser.errors.count)
}

fn ok_or_die<T, U>(result: Result<T, U>, message: &str) -> T {
    if let Ok(val) = result {
        return val;
    }
    eprintln!("{}", message);
    std::process::exit(1);
}

fn some_or_die<T>(option: Option<T>, message: &str) -> T {
    if let Some(val) = option {
        return val;
    }
    eprintln!("{}", message);
    std::process::exit(1);
}
