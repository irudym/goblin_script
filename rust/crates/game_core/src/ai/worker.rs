use crate::bt::result::BTResult;
use crate::character::CharacterId;
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::thread;

use super::super::bt::job::BTJob;
use num_cpus;

use platform::log_debug;
use platform::logger::LogType;
use platform::shared::logger_global::log;

/*
lazy_static! {
    pub static ref JOB_TX: Sender<BTJob> = {
        let (tx, rx) = unbounded();
        std::thread::spawn(move || worker_loop(rx));
        tx
    };
    pub static ref RESULT_MAP: Mutex<HashMap<CharacterId, BTResult>> = Mutex::new(HashMap::new());
}
*/

pub static JOB_TX: OnceLock<Sender<BTJob>> = OnceLock::new();
static RESULT_MAP: OnceLock<Mutex<std::collections::HashMap<CharacterId, BTResult>>> =
    OnceLock::new();

pub fn init_bt_system() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    log_debug!("Thread pool created: {:?}", &pool);

    let (tx, rx) = unbounded::<BTJob>();
    JOB_TX.set(tx).ok();
    RESULT_MAP.set(Mutex::new(HashMap::new())).ok();

    std::panic::set_hook(Box::new(|panic_info| {
        log_debug!("PANIC in worker thread: {:?}", &panic_info);
        eprintln!("PANIC in worker thread: {:?}", panic_info);
    }));

    let handle = thread::spawn(move || worker_loop(rx, pool));

    log_debug!("Separated thread created: {:?}", &handle);
}

pub fn take_result(character_id: CharacterId) -> Option<BTResult> {
    let mut map = RESULT_MAP.get().unwrap().lock().unwrap();
    map.remove(&character_id)
}

fn worker_loop(rx: Receiver<BTJob>, pool: rayon::ThreadPool) {
    loop {
        // batch jobs for the current frame
        let mut jobs = Vec::new();
        while let Ok(job) = rx.try_recv() {
            jobs.push(job);
            if jobs.len() >= 32 {
                break;
            } //limit per batch
        }

        if jobs.is_empty() {
            //avoid spinning hot, sleep a tiny bit
            std::thread::sleep(std::time::Duration::from_micros(100));
            continue;
        }

        // run all jobs in parallel
        pool.scope(|scope| {
            for job in jobs.drain(..) {
                scope.spawn(move |_| {
                    let result = job.bt.tick(&job.snapshot, 0.016);
                    let mut map = RESULT_MAP.get().unwrap().lock().unwrap();

                    map.insert(job.character_id, result);
                });
            }
        })
    }
}
