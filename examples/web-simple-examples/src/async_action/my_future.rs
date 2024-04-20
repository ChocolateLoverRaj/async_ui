use std::future::Future;
use crate::async_action::my_result::MyResult;

pub type MyFuture = Box<dyn Future<Output=MyResult> + Unpin>;
