pub mod error;
pub mod worker;

use futures::future::join_all;
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::{select, signal, sync::Notify, task::JoinHandle, time};
use tracing::{error, info, warn};

use crate::worker::Worker;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type WorkerFuture = Pin<Box<dyn Future<Output = Result<Duration>> + Send>>;

pub struct WorkerFactory {
    workers: Vec<Arc<dyn Worker>>,
    stop_notify: Arc<Notify>,
}

impl WorkerFactory {
    pub fn new() -> Self {
        Self {
            workers: vec![],
            stop_notify: Arc::new(Notify::const_new()),
        }
    }

    pub fn push(&mut self, worker: impl Worker + 'static) -> &mut Self {
        self.workers.push(Arc::new(worker));
        self
    }

    pub async fn run_all(self) -> Result<()> {
        let stop_notify_clone = self.stop_notify.clone();

        tokio::spawn(async move {
            if let Err(e) = signal::ctrl_c().await {
                error!("Failed to listen for Ctrl+C: {e}");
            }
            stop_notify_clone.notify_waiters();
            info!("Shutdown signal received. Notifying workers to stop...");
        });

        let supervisor_handle = self.start_loop();

        self.stop_notify.notified().await;

        self.stop(supervisor_handle).await
    }

    async fn stop(self, supervisor_handle: JoinHandle<()>) -> Result<()> {
        match supervisor_handle.await {
            Ok(_) => info!("All workers have gracefully stopped."),
            Err(e) => {
                if e.is_cancelled() {
                    warn!("Supervisor task was cancelled.");
                } else if e.is_panic() {
                    error!("Supervisor task panicked: {:?}", e);
                }
            }
        }
        Ok(())
    }

    fn start_loop(&self) -> JoinHandle<()> {
        let workers = self.workers.clone();
        let stop_notify = self.stop_notify.clone();

        tokio::spawn(async move {
            let mut worker_tasks: Vec<JoinHandle<()>> = Vec::new();
            for worker in workers {
                let notify_clone = stop_notify.clone();
                worker_tasks.push(tokio::spawn(async move {
                    let name = worker.name();
                    loop {
                        match worker.loop_process().await {
                            Ok(duration) => {
                                select! {
                                    _ = time::sleep(duration) => {}
                                    _ = notify_clone.notified() => {
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Cannot process worker `{name}` loop due {err}!");
                                break;
                            }
                        }
                    }
                }));
            }

            let task_results = join_all(worker_tasks).await;
            for result in task_results {
                if let Err(e) = result {
                    if e.is_panic() {
                        error!("A worker task panicked!");
                    } else {
                        error!("A worker task failed to join: {:?}", e);
                    }
                }
            }
        })
    }
}
