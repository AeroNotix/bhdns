use packed_struct::prelude::*;

mod dns;

use dns::server;

use std::fs::read;

fn main() -> Result<(), PackingError> {
    println!(
        "{:?}",
        read("/home/xeno/dev/bhdns/fixtures/google-query.bin").unwrap()
    );

    server::serve().unwrap();

    Ok(())
}
