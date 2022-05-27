use abci::types::{
    RequestBeginBlock, RequestCommit, RequestDeliverTx, RequestEndBlock, RequestInitChain,
    ResponseBeginBlock, ResponseCommit, ResponseDeliverTx, ResponseEndBlock, ResponseInitChain,
};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Consensus;

impl abci::async_api::Consensus for Consensus {
    fn init_chain<'life0, 'async_trait>(
        &'life0 self,
        init_chain_request: RequestInitChain,
    ) -> Pin<Box<dyn Future<Output = ResponseInitChain> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", init_chain_request);
    }

    fn begin_block<'life0, 'async_trait>(
        &'life0 self,
        begin_block_request: RequestBeginBlock,
    ) -> Pin<Box<dyn Future<Output = ResponseBeginBlock> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", begin_block_request);
    }

    fn deliver_tx<'life0, 'async_trait>(
        &'life0 self,
        deliver_tx_request: RequestDeliverTx,
    ) -> Pin<Box<dyn Future<Output = ResponseDeliverTx> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", deliver_tx_request);
    }

    fn end_block<'life0, 'async_trait>(
        &'life0 self,
        end_block_request: RequestEndBlock,
    ) -> Pin<Box<dyn Future<Output = ResponseEndBlock> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", end_block_request);
    }

    fn commit<'life0, 'async_trait>(
        &'life0 self,
        commit_request: RequestCommit,
    ) -> Pin<Box<dyn Future<Output = ResponseCommit> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", commit_request);
    }
}
