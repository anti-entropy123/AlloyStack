use alloc::string::String;
// use alloc::

use crate::{HostCallID, IsolationContext};

pub type IsolationID = u64;

pub type FindHostCallFunc = unsafe extern "C" fn(IsolationID, HostCallID) -> usize;
pub type SetHandlerFunc = unsafe extern "C" fn(IsolationContext) -> HostCallResult;
pub type GetHandlerFunc = unsafe extern "C" fn() -> usize;
pub type RustMainFunc = unsafe fn() -> ();
pub type HostWriteFunc = fn(i32, &str) -> isize;
pub type HostStdioFunc = fn(&str) -> isize;

pub type ServiceName = String;
pub type SymbolName = String;

pub type HostCallResult = Result<(), HostCallError>;
#[derive(Debug)]
pub enum HostCallError {
    HasBeenSet,
}
