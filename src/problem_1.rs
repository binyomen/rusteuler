use std;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;

pub fn problem_1() -> Result<u64, String> {
    const UPPER_BOUND: u64 = 1000;
    const MAX_THREADS: u64 = 5;
    let (tx, rx) = mpsc::channel();

    for i in 0..MAX_THREADS {
        let tx = tx.clone();
        thread::spawn(move || {
            let start = (i * UPPER_BOUND) / MAX_THREADS;
            let end = start + (UPPER_BOUND / MAX_THREADS);
            for n in start..end {
                if (n % 3 == 0) || (n % 5 == 0) {
                    if let Err(err) = tx.send(n) {
                        let mut stderr = std::io::stderr();
                        writeln!(&mut stderr, "sending to channel failed: {}", err)
                            .expect("could not write to stderr");
                    }
                }
            }
        });
    }
    drop(tx);

    let mut sum = 0;
    for val in rx {
        sum += val;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let solution = problem_1();
        let solution = solution.unwrap_or_else(|err| panic!("{}", err));
        assert_eq!(233168, solution);
    }
}
