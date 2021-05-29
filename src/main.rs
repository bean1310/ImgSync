extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate daemonize_me;
use daemonize_me::{Daemon, Group, User};
use std::convert::TryFrom;
use std::fs::File;

mod storage;
use storage::slack::Slack;

#[macro_use]
extern crate ini;

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
    // let config = ini!("/etc/img_sync");
    let config      = ini!("./etc/img_sync");
    
    let observe_dir = config["basic"]["dir"].clone().unwrap();
    let token       = config["slack"]["token"].clone().unwrap();
    // This program run as daemon process
    let daemon = init_daemon();

    if let Err(e) = daemon {
        eprintln!("Error happned, {}", e);
        panic!();
    }

    if let Err(e) = watch(&observe_dir) {
        println!("error: {:?}", e);
        panic!()
    }

}

fn watch(observe_dir: &str) -> notify::Result<()>
{

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(observe_dir, RecursiveMode::NonRecursive)?;

    loop
    {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(error)  => println!("{:?}", error)
        }
    }

}
