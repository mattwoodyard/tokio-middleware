//! Implements a service that resolves hostnames and ports into SocketAddr
//! Heavily influenced by a similar bit of code in hyper
//! Spawn a thread via thread pool
//! The service runs  the dns lookup in the thread
use futures_cpupool::{CpuPool, CpuFuture};
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio_service::Service;


#[derive(Clone)]
pub struct Resolver {
    pool: CpuPool
}

// TODO find a real dns lookup that doesn't need threads
// TODO maybe cache some results around here somewhere
impl Resolver {
    pub fn new() -> Resolver {
        // TODO - configure with builder
        Resolver {
            pool: CpuPool::new(1)
        }

    }
}

#[derive(Clone, Debug)]
pub struct SocketAddrs {
    pub addrs: Vec<SocketAddr>
}

impl Service for Resolver {
    type Request = (String, u16);
    type Response = SocketAddrs;
    type Future = CpuFuture<Self::Response, Self::Error>;
    type Error = io::Error;


    fn call(&self,  req: Self::Request) -> Self::Future {
        let (hostname, port) = req;
        self.pool.spawn_fn(move || { query(hostname, port) })
    }
}

fn query(hostname: String, port: u16) -> io::Result<SocketAddrs> {
    (hostname.as_str(), port).to_socket_addrs().map(|r| SocketAddrs { addrs: r.collect() })
}
