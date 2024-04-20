use futures::future::Shared;
use futures::FutureExt;
use futures_signals::signal::{Mutable, SignalExt};
use gloo_timers::future::TimeoutFuture;
use rand::random;

use async_ui_web::html::{Br, Input, Label};
use async_ui_web::join;
use async_ui_web::prelude_traits::*;

use crate::async_action::fetch_button::fetch_button;
use crate::async_action::future_option_ui::future_option_ui;
use crate::async_action::my_future::MyFuture;
use crate::async_action::reset_button::reset_button;

mod fetch_button;
mod my_result;
mod my_future;
mod reset_button;
mod error_button;
mod result_ui;
mod future_ui;
mod future_option_ui;

pub async fn async_action() {
    let promise = Mutable::new(None::<Shared<MyFuture>>);

    let input = Input::new_checkbox();
    let fetch = {
        let input = input.clone();
        || {
            let fut: MyFuture = Box::new(Box::pin(async move {
                TimeoutFuture::new(1000).await;
                match input.checked() {
                    true => Ok(random()),
                    false => Err(())
                }
            }));
            promise.set(Some(fut.shared()));
        }
    };
    join((
        Label::new().render(join(("Succeed".render(), input.render()))),
        Br::new().render(),
        fetch_button(promise.signal_cloned().map(|option| option.is_some()), fetch.clone()),
        reset_button(|| promise.set(None)),
        future_option_ui(promise.signal_cloned(), fetch)
    )).await;
}
