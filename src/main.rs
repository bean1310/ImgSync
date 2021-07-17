extern crate notify;

use std::path::PathBuf;
use notify::DebouncedEvent::Create;
use std::env;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate daemonize_me;
use daemonize_me::{Daemon};

use std::fs::File;

mod storage;
use storage::Storage;
use storage::StorageError;
use storage::slack::Slack;

#[macro_use]
extern crate ini;

struct Config
{
    storage: Box<dyn Storage>,
    observe_dir: PathBuf
}

impl Config
{
    fn new() -> Self
    {
        let _config = ini!("/etc/img_sync");
        let observe_dir = _config["basic"]["dir"].clone().unwrap();

        let token = _config["slack"]["token"].clone().unwrap();
        let channel_id = _config["slack"]["channel_id"].clone().unwrap();
        
        Self
        {
            observe_dir: PathBuf::from(&observe_dir),
            storage: Box::new(Slack::new(&token, &channel_id)),
        }
    }
}

#[cfg(debug_assertions)]
fn init_daemon() -> Result<(), daemonize_me::DaemonError>
{
    let stdout = File::create("debug.stdout").unwrap();
    let stderr = File::create("debug.stderr").unwrap();
    let pid_file = "img_sync.pid";

    let daemon = Daemon::new()
                    .pid_file(pid_file, Some(false))
                    .umask(0o022)
                    .work_dir(".")
                    .stdout(stdout)
                    .stderr(stderr)
                    .start();
    println!("Created daemon: {:?}", daemon);

    daemon
}

#[cfg(not(debug_assertions))]
fn init_daemon() -> Result<(), daemonize_me::DaemonError>
{
    let pidFile = "/var/run/img_sync.pid";

    let daemon = Daemon::new()
                    .pid_file(pidFile, Some(false))
                    .umask(0o022)
                    .work_dir("/tmp")
                    .start();

    daemon

}



fn main()
{
    
    let config = Config::new();
    // Option handling
    let _arg_len = env::args().len();
    if _arg_len > 0 {
        let _args: Vec<String> = env::args().collect();

        let _help_flag = _args.iter().any(|options| options == "--help");
        let _daemon_flag = _args.iter().any(|options| options == "--daemon");

        if _help_flag {
            println!("Image Syncer written in Rust\n");
            println!("USAGE:\n\timg_sync [OPTIONS]\n");
            println!("OPTIONS:\n\t--help\t\tPrint help information\n\t--daemon\tRun as daemon process");
            std::process::exit(0);
        }

        if _daemon_flag {
            // If given "--daemon" flag, this program run as daemon process
            let daemon = init_daemon();

            if let Err(e) = daemon {
                eprintln!("Failed to daemonize.");
                eprintln!("Error happned, {}", e);
                panic!();
            }
        }
    }

    // Start main process
    if let Err(e) = watch(config.observe_dir, config.storage) {
        eprintln!("Failed to observe directory");
        eprintln!("error: {:?}", e);
        panic!()
    }

}

fn watch(observe_dir: PathBuf, storage: Box<dyn Storage>) -> Result<(), Box<dyn std::error::Error>>
{

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(observe_dir, RecursiveMode::NonRecursive)?;

    loop
    {
        match rx.recv() {
            Ok(event) =>
            {
                if let Create(path) = event
                {
                    println!("[log] detect file create event");
                    let upload_result = storage.upload(&path);
                    match upload_result
                    {
                        Ok(()) => println!("[log] Successed to upload file: {}", path.display()),
                        Err(error)  =>
                            {
                                match error.downcast_ref::<StorageError>() {
                                    Some(e) => {
                                        match e {
                                            StorageError::ApiError(msg)  =>  {
                                                eprintln!("[Warning] Failed to upload file: {}", path.display());
                                                eprintln!("\t Storage API Error messsage is : {}", msg);
                                            },
                                            StorageError::HttpError(http_status_code)  =>  {
                                                eprintln!("[Warning] Failed to upload file: {}", path.display());
                                                eprintln!("\t Http status code is : {}", http_status_code);
                                            }
                                        }
                                    },
                                    _   =>  ()
                                }
                            }
                    }
                }
            },
            Err(error)  => eprintln!("[Error] Error happend: {:?}", error)
        }
    }

}
