use error::SqlittleError;

use async_trait::async_trait;
use std::sync::Arc;

pub trait DatabaseStorage: Send + Sync {
    async fn read_header(&self) -> Result<Arc<Buffer>>;
    async fn read_page(&self, page_idx: usize, io_ctx: &IOContext) -> Result<Arc<Buffer>>;
    async fn write_page(&self, page_idx: usize, buffer: Arc<Buffer>, io_ctx: &IOContext) -> Result<()>;
    async fn write_pages(&self, first_page_idx: usize, page_size: usize, buffers: Vec<Arc<Buffer>>, io_ctx: &IOContext) -> Result<()>;
    async fn sync(&self) -> Result<()>;
    async fn size(&self) -> Result<u64>;
    async fn truncate(&self, len: usize) -> Result<()>;
}
