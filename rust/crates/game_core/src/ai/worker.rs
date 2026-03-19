use crate::bt::result::BTResult;
use crate::character::CharacterId;
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::thread::{self, JoinHandle};

use super::super::bt::job::BTJob;
use num_cpus;

use platform::log_debug;
use platform::logger::LogType;

pub static JOB_TX: OnceLock<Sender<BTJob>> = OnceLock::new();
static RESULT_MAP: OnceLock<Mutex<HashMap<CharacterId, (u32, BTResult)>>> = OnceLock::new();
static SHUTDOWN_TX: OnceLock<Sender<()>> = OnceLock::new();
static WORKER_HANDLE: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();

pub fn init_bt_system() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    log_debug!("Thread pool created: {:?}", &pool);

    let (tx, rx) = unbounded::<BTJob>();
    let (shutdown_tx, shutdown_rx) = unbounded::<()>();

    JOB_TX.set(tx).expect("init_bt_system called more than once");
    RESULT_MAP
        .set(Mutex::new(HashMap::new()))
        .expect("init_bt_system called more than once");
    SHUTDOWN_TX
        .set(shutdown_tx)
        .expect("init_bt_system called more than once");
    WORKER_HANDLE
        .set(Mutex::new(None))
        .expect("init_bt_system called more than once");

    std::panic::set_hook(Box::new(|panic_info| {
        log_debug!("PANIC in worker thread: {:?}", &panic_info);
        eprintln!("PANIC in worker thread: {:?}", panic_info);
    }));

    let handle = thread::spawn(move || worker_loop(rx, shutdown_rx, pool));
    log_debug!("Separated thread created: {:?}", &handle);

    if let Some(slot) = WORKER_HANDLE.get() {
        *slot.lock().unwrap() = Some(handle);
    }
}

pub fn shutdown_bt_system() {
    // Signal the worker to stop.
    if let Some(tx) = SHUTDOWN_TX.get() {
        let _ = tx.send(());
    }
    // Join the worker thread so it fully stops before the extension is unloaded.
    if let Some(slot) = WORKER_HANDLE.get() {
        if let Some(handle) = slot.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
    log_debug!("BT system shut down");
}

/// Returns the BT result only if it matches `expected_generation`.
/// Always removes the stored entry so stale results don't accumulate.
pub fn take_result(character_id: CharacterId, expected_generation: u32) -> Option<BTResult> {
    let mut map = RESULT_MAP.get().unwrap().lock().unwrap();
    if let Some((gen, result)) = map.remove(&character_id) {
        if gen == expected_generation {
            return Some(result);
        }
    }
    None
}

fn worker_loop(rx: Receiver<BTJob>, shutdown_rx: Receiver<()>, pool: rayon::ThreadPool) {
    loop {
        // Block until the first job arrives or a shutdown signal is received.
        crossbeam::select! {
            recv(shutdown_rx) -> _ => break,
            recv(rx) -> msg => {
                let first = match msg {
                    Ok(job) => job,
                    Err(_) => break, // job channel closed
                };
                let mut jobs = vec![first];

                // Drain any additional queued jobs up to batch limit.
                while jobs.len() < 32 {
                    match rx.try_recv() {
                        Ok(job) => jobs.push(job),
                        Err(_) => break,
                    }
                }

                // Run all jobs in parallel, collect results locally.
                let batch_results: Mutex<Vec<(CharacterId, u32, BTResult)>> =
                    Mutex::new(Vec::new());
                pool.scope(|scope| {
                    for job in jobs.drain(..) {
                        let batch_results = &batch_results;
                        scope.spawn(move |_| {
                            let result = job.bt.tick(&job.snapshot, job.delta);
                            batch_results
                                .lock()
                                .unwrap()
                                .push((job.character_id, job.generation, result));
                        });
                    }
                });

                // Single bulk insert into the global result map.
                let mut map = RESULT_MAP.get().unwrap().lock().unwrap();
                for (id, gen, result) in batch_results.into_inner().unwrap() {
                    map.insert(id, (gen, result));
                }
            }
        }
    }
}
