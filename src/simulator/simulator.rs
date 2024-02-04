use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use terminable_threads::TerminableThreads;

use super::object::SimObject;

pub const DEFAULT_TPS: AtomicU64 = AtomicU64::new(10);

struct Simulator {
    objects: Vec<Arc<Mutex<SimObject>>>,
    max_tps: u64,
    time: Arc<AtomicU64>,
    time_thread: TerminableThreads<TimeSummary, 1>,
    terminate_flag: Arc<AtomicBool>,
}

pub struct TimeSummary {
    started: u64,
    finished: u64,
}

impl Simulator {
    fn new(max_tps: u64) -> Self {
        let (tt_builder, arc_flag) = TerminableThreads::build();
        let time = Arc::new(AtomicU64::new(0));

        let termimate_flag = Arc::clone(&arc_flag);
        let time_c = Arc::clone(&time);

        let thread = move || {
            thread::spawn(move || {
                let millis = 1000000u64 / {
                    if max_tps == 0 {
                        u64::MAX
                    } else {
                        max_tps
                    }
                };

                while !termimate_flag.as_ref().load(Ordering::SeqCst) {
                    time_c.fetch_add(1, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(millis))
                }

                TimeSummary {
                    started: 0,
                    finished: time_c.load(Ordering::SeqCst),
                }
            })
        };

        Self {
            objects: vec![],
            max_tps: match max_tps {
                0 => DEFAULT_TPS.load(Ordering::SeqCst),
                m => m,
            },
            time: Arc::clone(&time),
            terminate_flag: arc_flag,
            time_thread: tt_builder.build_with_threads([thread()]),
        }
    }
}
