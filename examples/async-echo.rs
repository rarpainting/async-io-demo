#![feature(async_await)]
#![feature(await_macro)]

#[macro_use]
extern crate log;

use asyncio::executor::{block_on, spawn, TcpListener};
use failure::Error;

// pub enum Poll<T> {
//     Ready(T),
//     Pending,
// }

fn main() -> Result<(), Error> {
    env_logger::init();
    block_on(
        async {
            let mut listener = TcpListener::bind(&"127.0.0.1:7878".parse()?)?;
            info!("Listening on 127.0.0.1:7878");
            while let Ok((mut stream, addr)) = await!(listener.accept()) {
                info!("connection from {}", addr);
                spawn(
                    async move {
                        let client_hello = await!(stream.read())?;
                        let read_length = client_hello.len();
                        let write_length =
                            await!(stream.write(client_hello))?;
                        assert_eq!(read_length, write_length);
                        stream.close();
                        Ok(())
                    },
                )?;
            };
            Ok(())
        },
    )?
}
