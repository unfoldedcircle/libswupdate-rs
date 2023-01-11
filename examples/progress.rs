use libswupdate::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut swupdate = SWUpdateProgress::new();

    println!("Connecting to: {}", swupdate.get_prog_socket());

    swupdate.connect_progress()?;

    loop {
        println!("{:?}", swupdate.receive_progress()?);
    }
}
