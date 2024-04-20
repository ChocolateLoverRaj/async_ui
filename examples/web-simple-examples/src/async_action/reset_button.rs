use async_ui_web::event_traits::EmitElementEvent;
use async_ui_web::html::Button;
use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;

pub async fn reset_button<Reset: Fn()>(reset: Reset) {
    let reset_button = Button::new();
    join((
        reset_button.render("Reset".render()),
        async {
            loop {
                reset_button.until_click().await;
                reset();
            }
        })
    ).await;
}