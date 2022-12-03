extern crate num_derive;
extern crate core;

pub mod models;
pub mod server;
pub mod packets;
#[macro_use]
pub mod event_system;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::server::Server;

    #[test]
    fn it_works() {
        let server = Server::new().unwrap();

        server.start();
    }
}