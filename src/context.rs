use std::cell::RefCell;

use lighthouse_client::{Lighthouse, TokioWebSocket};
use rustyline::{completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper};
use tokio::runtime::Handle;

use crate::path::VirtualPathBuf;

pub struct Context {
    // TODO: If/when complete is &mut self, we can remove interior mutability again.
    pub lh: RefCell<Lighthouse<TokioWebSocket>>,
    pub cwd: VirtualPathBuf,
    pub handle: Handle,
}

impl Completer for Context {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let child = self.cwd.join(VirtualPathBuf::from(line));
        let candidates = self.handle.block_on(async {
            self.lh.borrow_mut().list(&child.as_relative().as_str_vec()).await
        }).map(|response| {
            response.payload.entries.into_keys().collect()
        }).unwrap_or_default();
        Ok((0, candidates))
    }
}

impl Hinter for Context {
    type Hint = String;
}

impl Helper for Context {}

impl Highlighter for Context {}

impl Validator for Context {}
