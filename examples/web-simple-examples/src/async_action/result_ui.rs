use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;
use crate::async_action::error_button::error_button;
use crate::async_action::my_result::MyResult;

pub async fn result_ui<Retry: FnOnce()>(result: MyResult, retry: Retry) {
    match result {
        Ok(result) => {
            join(("Result: ".render(), result.to_string().render())).await;
        }
        Err(()) => error_button(retry).await
    }
}