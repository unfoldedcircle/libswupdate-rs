use libswupdate::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut progress = SWUpdateProgress::new();

    println!("Connecting to: {}", progress.get_socket_path());

    progress.connect()?;

    loop {
        println!("{:?}", progress.receive()?);
    }
}
