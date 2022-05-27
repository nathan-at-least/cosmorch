#[derive(Debug)]
pub struct Snapshot;

impl abci::async_api::Snapshot for Snapshot {}
