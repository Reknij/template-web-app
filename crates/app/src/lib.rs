mod args;
mod banner;

use db::storage::{FullStorage, sqlite_impl::StorageSqliteImpl};
use service::CommonService;
use std::process;
use std::sync::Arc;
use sys_core::config::Config;
use tokio::runtime::Builder;
use tracing::{error, info, warn};

pub fn run() {
    banner::banner();
    tracing_subscriber::fmt::init();
    info!("Welcome to using template system..");

    let args = args::parse();

    let config: Config = if args.config.is_file() {
        info!("Loading configuration from file: {:?}", args.config);
        if let Ok(config) = Config::load_from_path(&args.config) {
            config
        } else {
            warn!("Configuration file found but cannot be loaded or parsed. Using default config.");
            Config::default()
        }
    } else {
        warn!("Configuration file not found or path is not a file: {:?}. Using default config.", args.config);
        Config::default()
    };

    info!("Configuration loaded successfully!");

    let rt = Builder::new_multi_thread().enable_all().build().expect("Failed to build tokio runtime");

    rt.block_on(async {
        let storage: Arc<dyn FullStorage> = {
            let db_url = &config.db.url;

            let scheme = db_url.split(':').next().unwrap_or("unknown");

            match scheme {
                "sqlite" => {
                    info!("Initializing SQLite storage from URL: {}", db_url);

                    match StorageSqliteImpl::new(db_url.clone()).await {
                        Ok(sqlite_storage) => Arc::new(sqlite_storage),
                        Err(e) => {
                            error!("FATAL: Failed to initialize SQLite storage: {}", e);
                            process::exit(1);
                        }
                    }
                }
                "postgres" | "mysql" | "mssql" => {
                    error!(
                        "FATAL: Database type '{}' is currently not supported. I plan to support it in a future release! Please use 'sqlite:' for now.",
                        scheme
                    );
                    process::exit(1);
                }
                _ => {
                    error!("FATAL: Unknown database scheme specified in config: {}", scheme);
                    process::exit(1);
                }
            }
        };

        let mut worker_factory = worker::WorkerFactory::new();
        // worker_factory.push(SomeWorker::new(storage.clone()));
        tokio::spawn(async move {
            worker_factory.run_all().await.expect("Workers run error!");
        });

        let service = CommonService::new(storage, config.clone());

        info!("Starting web server...");
        web::serve(config, service).await;
    });
}
