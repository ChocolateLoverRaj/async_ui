use std::future::Future;

use futures::{FutureExt, StreamExt};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::convert::IntoWasmAbi;

use async_ui_web::{join, race};
use async_ui_web::event_traits::{EmitElementEvent, EmitHtmlElementEvent};
use async_ui_web::html::{Br, Input};
use async_ui_web::shortcut_traits::{ShortcutRenderStr, UiFutureExt};

use crate::set_async_ui::set_async::SetAsync;
use crate::set_async_ui::stream_render_ext::StreamRenderExt;

mod set_async;
mod stream_render_ext;

pub async fn set_async_ui() {
    let saved_value: Mutable<String> = Default::default();
    let set_async = SetAsync::<String, ()>::new(Box::new({
        let saved_value = saved_value.clone();

        move |value| {
            let saved_value = saved_value.clone();
            Box::new(Box::pin(async move {
                TimeoutFuture::new(2000).await;
                saved_value.set(value);
            }))
        }
    }));

    join((
        set_async.run(),
        async {
            let input = Input::new();
            input.render().meanwhile(async {
                loop {
                    input.until_input().await;
                    set_async.set(input.value()).await;
                }
            }).await;
        },
        set_async.get_future_signal().to_stream().map(|future| async {
            match future {
                Some(future) => {
                    race((
                        "Saving".render(),
                        future.map(|_| ())
                    )).await;
                    "Saved".render().await;
                }
                None => {
                    "".render().await;
                }
            }
        }).render(),
        set_async.get_queued_signal().to_stream().map(|queued| async {
            match queued {
                Some(queued) => {
                    format!(" (Queued: {:#?})", queued).render().await;
                },
                None => {
                    "".render().await;
                }
            }
        }).render(),
        Br::new().render(),
        saved_value.signal_cloned().to_stream().map(|value| value.render()).render()
    )).await;
}
