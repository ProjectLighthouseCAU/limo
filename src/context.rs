use lighthouse_client::{Lighthouse, TokioWebSocket};
use rustyline::{completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper};

use crate::path::VirtualPathBuf;

pub struct Context {
    pub lh: Lighthouse<TokioWebSocket>,
    pub cwd: VirtualPathBuf,
}

impl Completer for Context {
    type Candidate = String;
}

impl Hinter for Context {
    type Hint = String;
}

impl Helper for Context {}

impl Highlighter for Context {}

impl Validator for Context {}
