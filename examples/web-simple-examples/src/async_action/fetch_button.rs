use futures_signals::signal::{Signal, SignalExt};

use async_ui_web::components::DynamicSlot;
use async_ui_web::event_traits::EmitElementEvent;
use async_ui_web::html::Button;
use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;

pub async fn fetch_button<FetchingSignal: Signal<Item=bool>, Fetch: FnOnce() + Clone>(fetching: FetchingSignal, fetch: Fetch) {
    let fetch_button = Button::new();
    let fetch_button_slot = DynamicSlot::new();
    join((
        fetch_button.render(fetch_button_slot.render()),
        fetching.for_each(|fetching| {
            fetch_button_slot.set_future(async move {
                match fetching {
                    false => "Fetch".render(),
                    true => "Fetch Again".render()
                }.await;
            });
            async {}
        }),
        async {
            let fetch = fetch.clone();
            loop {
                let fetch = fetch.clone();
                fetch_button.until_click().await;
                fetch();
            }
        }
    )).await;
}