use ail_project::cdcl;
use ail_project::cdcl::decision::DecideFirstVariable;
use ail_project::cdcl::first_uip::FirstUIP;
use clap::Parser;
use clio::*;
use std::io::{BufReader, Write};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// Output file '-' for stdout
    #[clap(long, short, value_parser, default_value = "-")]
    output: Output,

    /// Directory to store log files in
    #[clap(long, short, value_parser = clap::value_parser!(ClioPath).exists().is_dir(), default_value = ".")]
    log_dir: ClioPath,
}

fn main() {
    let mut opt = Opt::parse();

    let (n, formula) = cdcl::read_dimacs(&mut BufReader::new(opt.input));

    let ans = cdcl::cdcl_solve::<DecideFirstVariable, FirstUIP>(n, &mut formula.clone());

    match ans {
        None => {
            writeln!(opt.output, "UNSAT").unwrap();
        }
        Some(assignment) => {
            assert!(cdcl::is_satisfying(&formula, &assignment));

            writeln!(opt.output, "SAT").unwrap();

            let assignment: Vec<_> = assignment
                .iter()
                .copied()
                .map(|v| match v {
                    false => "0",
                    true => "1",
                })
                .collect();

            writeln!(opt.output, "{}", assignment.join(" ")).unwrap();
        }
    }
}
