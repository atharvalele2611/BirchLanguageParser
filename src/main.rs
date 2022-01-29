use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    #[clap(value_name = "FILE.bir")]
    file: String,
    /// Report allocation and time statistics
    #[clap(short, long)]
    stats: bool,
    /// Trace execution
    #[clap(short, long)]
    trace: bool,
}

use std::fs;
use std::process;
use std::time::Instant;

fn main() {
    let args = Args::parse();
    let file = &args.file;
    let src: String = match fs::read_to_string(file) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Failed to read {} [{}]", file, err);
            process::exit(1);
        }
    };
    let prog = match src.parse::<birch::Prog>() {
        Ok(prog) => prog,
        Err(prog_parse_error) => {
            eprintln!("{:?}", prog_parse_error);
            process::exit(1);
        }
    };
    println!("{:?}", prog);
    let exec_start = Instant::now();
    match prog.exec(args.trace) {
        Ok(n) => println!("{}", n),
        Err(prog_exec_error) => {
            eprintln!("{:?}", prog_exec_error);
            process::exit(1);
        }
    }
    let exec_time = {
        let exec_time = exec_start.elapsed();
        exec_time.as_secs_f64() + exec_time.subsec_nanos() as f64 / 1.0e9
    };
    let stats = GLOBAL.stats();
    if args.stats {
        println!();
        println!("allocated   : {:>13} bytes", stats.bytes_allocated);
        println!("reallocated : {:>13} bytes", stats.bytes_reallocated);
        println!("exec time   : {:>13.10} seconds", exec_time);
    }
}
