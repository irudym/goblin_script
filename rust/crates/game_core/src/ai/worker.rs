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
    JOB_TX
        .set(tx)
        .expect("init_bt_system called more than once");
    RESULT_MAP
        .set(Mutex::new(HashMap::new()))
        .expect("init_bt_system called more than once");

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
        // block until the first job arrives (no CPU spin)
        let first = match rx.recv() {
            Ok(job) => job,
            Err(_) => break, // channel closed, exit worker loop
        };
        let mut jobs = vec![first];

        // drain any additional queued jobs up to batch limit
        while jobs.len() < 32 {
            match rx.try_recv() {
                Ok(job) => jobs.push(job),
                Err(_) => break,
            }
        }

        // run all jobs in parallel, collect results locally
        let batch_results: Mutex<Vec<(CharacterId, BTResult)>> = Mutex::new(Vec::new());
        pool.scope(|scope| {
            for job in jobs.drain(..) {
                let batch_results = &batch_results;
                scope.spawn(move |_| {
                    let result = job.bt.tick(&job.snapshot, job.delta);
                    batch_results
                        .lock()
                        .unwrap()
                        .push((job.character_id, result));
                });
            }
        });

        // single bulk insert into the global result map
        let mut map = RESULT_MAP.get().unwrap().lock().unwrap();
        for (id, result) in batch_results.into_inner().unwrap() {
            map.insert(id, result);
        }
    }
}
