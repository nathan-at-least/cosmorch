use abci::types::{RequestCheckTx, ResponseCheckTx};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Mempool;

impl abci::async_api::Mempool for Mempool {
    fn check_tx<'life0, 'async_trait>(
        &'life0 self,
        check_tx_request: RequestCheckTx,
    ) -> Pin<Box<dyn Future<Output = ResponseCheckTx> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", check_tx_request);
    }
}
