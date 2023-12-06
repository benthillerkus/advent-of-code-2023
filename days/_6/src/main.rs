use rayon::{join, prelude::*};

struct Run {
    duration: u16,
    record: u16,
}

impl Run {
    fn hold(&self, hold: u16) -> bool {
        let time_left = self.duration - hold;
        let velocity = hold;
        let distance = velocity * time_left;
        distance > self.record
    }
}

const INPUT: &[Run; 4] = &[
    Run {
        duration: 42,
        record: 308,
    },
    Run {
        duration: 89,
        record: 1170,
    },
    Run {
        duration: 91,
        record: 1291,
    },
    Run {
        duration: 89,
        record: 1467,
    },
];

fn a() {
    let product: usize = INPUT
        .par_iter()
        .map(|run| {
            (0..(run.duration))
                .flat_map(|hold| run.hold(hold).then_some(hold))
                .count()
        })
        .product();

    println!("{product}");
}

fn b() {
    let duration: u64 = 42899189;
    let record: u64 = 308117012911467;

    if let (Some(first_win), Some(last_win)) = join(
        || {
            (0..=duration).find(|hold| {
                let time_left = duration - hold;
                let velocity = hold;
                let distance = velocity * time_left;
                distance > record
            })
        },
        || {
            (0..=duration).rev().find(|hold| {
                let time_left = duration - hold;
                let velocity = hold;
                let distance = velocity * time_left;
                distance > record
            })
        },
    ) {
        let wins = last_win - first_win + 1; // off by one

        println!("{wins}");
    }
}

fn main() {
    a();
    b();
}
