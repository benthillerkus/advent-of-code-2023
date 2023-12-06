use rayon::prelude::*;
use std::time::Duration;

struct Run {
    duration: Duration,
    record: u16,
}

impl Run {
    fn hold(&self, hold: Duration) -> bool {
        let time_left = self.duration - hold;
        if time_left.is_zero() {
            return false;
        }
        let velocity = hold.as_millis();
        let distance = velocity * time_left.as_millis();
        distance as u16 > self.record
    }
}

const INPUT: &[Run] = &[
    Run {
        duration: Duration::from_millis(42),
        record: 308,
    },
    Run {
        duration: Duration::from_millis(89),
        record: 1170,
    },
    Run {
        duration: Duration::from_millis(91),
        record: 1291,
    },
    Run {
        duration: Duration::from_millis(89),
        record: 1467,
    },
];

fn a() {
    let product: usize = INPUT
        .par_iter()
        .map(|run| {
            (0..(run.duration.as_millis()))
                .flat_map(|hold| run.hold(Duration::from_millis(hold as u64)).then_some(hold))
                .count()
        })
        .product();

    println!("{product}");
}

fn main() {
    a();
}
