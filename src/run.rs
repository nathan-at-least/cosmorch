use crate::{Consensus, Info, Mempool, Snapshot};
use abci::async_api::Server;

/// run the ABCI application server process
pub async fn run() -> anyhow::Result<()> {
    use std::str::FromStr;

    let s = Server::new(Consensus, Mempool, Info, Snapshot);
    let addr = std::net::SocketAddr::from_str("127.0.0.1:9287").unwrap();
    s.run(addr).await?;
    Ok(())
}
