extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate daemonize_me;
use daemonize_me::{Daemon, Group, User};
use std::convert::TryFrom;
use std::fs::File;

#[cfg(debug_assertions)]
fn init_daemon() -> Result<(), daemonize_me::DaemonError>
{
    let stdout = File::create("debug.stdout").unwrap();
    let stderr = File::create("debug.stderr").unwrap();
    let pid_file = "img_sync.pid";

    let daemon = Daemon::new()
                    .pid_file(pid_file, Some(false))
                    .user(User::try_from("daemon").unwrap())
                    .group(Group::try_from("daemon").unwrap())
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
                            .user(User::try_from("daemon").unwrap())
                            .group(Group::try_from("daemon").unwrap())
                            .umask(0o022)
                            .work_dir("/tmp")
                            .start();

    daemon

}

fn main()
{
    // This program run as daemon process
    let daemon = init_daemon();

    if let Err(e) = daemon {
        eprintln!("Error happned, {}", e);
        panic!();
    }

    if let Err(e) = watch() {
        println!("error: {:?}", e);
        panic!()
    }

}

fn watch() -> notify::Result<()>
{

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch("/home/isato/Sources/ImgSync/test", RecursiveMode::NonRecursive)?;

    loop
    {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(error)  => println!("{:?}", error)
        }
    }

}
