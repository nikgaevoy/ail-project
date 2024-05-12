use self::Solver::*;
use ail_project::cdcl;
use ail_project::cdcl::decision::DecideFirstVariable;
use ail_project::cdcl::first_uip::FirstUIP;
use ail_project::cdcl::mincut::*;
use ail_project::cdcl::propagation::ConflictAnalysis;
use ail_project::cdcl::Formula;
use clap::Parser;
use clio::*;
use std::io::{BufReader, Write};
use std::time::SystemTime;

#[derive(clap::ValueEnum, Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Solver {
    #[default]
    FirstUIPBasic,
    FirstUIP,
    SecondUIP,
    ThirdUIP,
    AllUIP,
    SaturatingAllUIP,
    RelSat,
    MinCut,
}

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// Output file '-' for stdout
    #[clap(long, short, value_parser, default_value = "-")]
    output: Output,

    #[clap(long, short, default_value_t, value_enum)]
    solver: Solver,
}

fn get_solver<C: ConflictAnalysis + 'static>(
) -> Box<dyn FnOnce(usize, &mut Formula) -> Option<Vec<bool>>> {
    #[cfg(debug_assertions)]
    eprintln!("Running: {}", std::any::type_name::<C>());

    Box::new(cdcl::cdcl_solve::<DecideFirstVariable, C>)
}

fn main() {
    let mut opt = Opt::parse();

    let (n, formula) = cdcl::read_dimacs(&mut BufReader::new(opt.input));

    let solver = match opt.solver {
        FirstUIPBasic => get_solver::<FirstUIP>(),
        FirstUIP => get_solver::<CutFirstUIP>(),
        SecondUIP => get_solver::<CutSecondUIP>(),
        ThirdUIP => get_solver::<CutThirdUIP>(),
        AllUIP => get_solver::<CutAllUIP>(),
        SaturatingAllUIP => get_solver::<CutSatAllUIP>(),
        RelSat => get_solver::<CutRelSat>(),
        MinCut => get_solver::<CutMinimal>(),
    };

    let start = SystemTime::now();

    let ans = solver(n, &mut formula.clone());

    writeln!(
        opt.output,
        "Time used: {}s",
        start.elapsed().unwrap().as_secs_f64()
    )
    .unwrap();

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
