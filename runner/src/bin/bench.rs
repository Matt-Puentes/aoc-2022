use gag::Gag;
use took::{Timer, Took};

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);
    let print_gag = Gag::stdout().unwrap();

    let times: Vec<_> = runner::jobs()
        .into_iter()
        .map(|(j, n, i)| {
            (
                n,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j(i);
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();
    drop(print_gag);

    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}
