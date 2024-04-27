use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_test::console_log;

use async_ui_web::event_traits::EmitElementEvent;
use async_ui_web::html::Button;
use async_ui_web::{join,race};
use async_ui_web::shortcut_traits::{ShortcutRenderStr, UiFutureExt};

pub async fn effects() {
    let button = Button::new();
    join((
        button.render("Toggle visible".render()),
        async {
            loop {
                button.until_click().meanwhile(sub_component()).await;
                button.until_click().await;
            }
        }
    )).await;
}

async fn sub_component() {
    struct CleanUpEr;

    impl Drop for CleanUpEr {
        fn drop(&mut self) {
            console_log!("I have been dropped. This is a place where you can put cleanup code");
        }
    }

    let clean_up_er = CleanUpEr {};

    "Sub Component".render().meanwhile(async {
        let mut t = 0;
        loop {
            race((
                format!(" - I have been mounted for ~{:#?} seconds", t).render(),
                TimeoutFuture::new(1000),
            )).await;
            t += 1;
        }
    }).await;
}