use std::{
    env, path::PathBuf
};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
    select,
    sync::mpsc::Receiver,
};

mod defmt_parser;
use defmt_parser::{handle_stream, Source};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    list_ports()?;

    // We create the source outside of the run command since recreating the stdin looses us some frames
    let mut source = Source::serial(PathBuf::from("COM4"), 115200)?;
    let mut manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let _ = manifest_dir.pop();
    let elf = manifest_dir
        .join("target")
        .join("thumbv7em-none-eabihf")
        .join("debug")
        .join("nucleo-f413zh-logger");

    log::debug!("absolute path of elf file with defmt messages: {:?}", elf);
    run_and_watch(elf, &mut source).await
}

fn list_ports() -> anyhow::Result<()> {
    let ports = tokio_serial::available_ports()?;
    if ports.is_empty() {
        println!("No COM ports found.");
    } else {
        println!("Available COM Ports:");
        for port in ports {
            println!(" - {}", port.port_name);
        }
    }
    Ok(())
}

async fn run_and_watch(elf: PathBuf, source: &mut Source) -> anyhow::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let path = elf.clone().canonicalize().unwrap();

    // We want the elf directory instead of the elf, since some editors remove
    // and recreate the file on save which will remove the notifier
    let directory_path = path.parent().unwrap();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.blocking_send(res);
        },
        Config::default(),
    )?;
    watcher.watch(directory_path.as_ref(), RecursiveMode::NonRecursive)?;

    loop {
        select! {
            r = handle_stream(elf.clone(), source) => r?,
            _ = has_file_changed(&mut rx, &path) => ()
        }
    }
}

async fn has_file_changed(rx: &mut Receiver<Result<Event, notify::Error>>, path: &PathBuf) -> bool {
    loop {
        if let Some(Ok(event)) = rx.recv().await {
            if event.paths.contains(path) {
                if let notify::EventKind::Create(_) | notify::EventKind::Modify(_) = event.kind {
                    break;
                }
            }
        }
    }
    true
}
