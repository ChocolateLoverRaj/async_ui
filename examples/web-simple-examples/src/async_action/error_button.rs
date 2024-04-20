use async_ui_web::event_traits::EmitElementEvent;
use async_ui_web::html::Button;
use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;

pub async fn error_button<Retry: FnOnce()>(retry: Retry) {
    join((
        "Error".render(),
        async {
            let button = Button::new();
            join((
                button.render("Retry".render()),
                async {
                    button.until_click().await;
                    retry();
                }
            )).await;
        }
    )).await;
}