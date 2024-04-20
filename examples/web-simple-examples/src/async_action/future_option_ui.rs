use futures::future::Shared;
use futures_signals::signal::{Signal, SignalExt};
use async_ui_web::components::DynamicSlot;
use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;
use crate::async_action::future_ui::future_ui;
use crate::async_action::my_future::MyFuture;

pub async fn future_option_ui<S: Signal<Item = Option<Shared<MyFuture>>>, Retry: FnOnce() + Clone>(signal: S, retry: Retry) {
    let slot = DynamicSlot::new();
    join((
        slot.render(),
        signal.for_each(|future| {
            let retry = retry.clone();
            slot.set_future(async move {
                match future {
                    None => "".render().await,
                    Some(future) => future_ui(future, retry).await
                }
            });
            async {}
        })
    )).await;
}