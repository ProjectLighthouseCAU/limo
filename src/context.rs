use lighthouse_client::{Lighthouse, TokioWebSocket};

use crate::path::VirtualPathBuf;

pub struct Context {
    pub lh: Lighthouse<TokioWebSocket>,
    pub cwd: VirtualPathBuf,
    pub username: String,
    pub host: String,
}
