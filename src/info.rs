use abci::types::{RequestInfo, ResponseInfo};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Info;

impl abci::async_api::Info for Info {
    fn info<'life0, 'async_trait>(
        &'life0 self,
        info_request: RequestInfo,
    ) -> Pin<Box<dyn Future<Output = ResponseInfo> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!("{:#?}", info_request);
    }
}
