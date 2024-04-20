use futures::future::Shared;
use async_ui_web::components::DynamicSlot;
use async_ui_web::html::Br;
use async_ui_web::join;
use async_ui_web::shortcut_traits::ShortcutRenderStr;
use crate::async_action::my_future::MyFuture;
use crate::async_action::my_result::MyResult;
use crate::async_action::result_ui::result_ui;

pub async fn future_ui<Retry: FnOnce() + Clone>(future: Shared<MyFuture>, retry: Retry) {
    join((
        Br::new().render(),
        async {
            let retry = retry.clone();
            let slot = DynamicSlot::new();
            join((
                slot.render(),
                async {
                    let my_render = |result: Option<MyResult>| async move {
                        match result {
                            Some(result) => result_ui(result, retry).await,
                            None => {
                                "Fetching".render().await;
                            }
                        }
                    };
                    match future.peek() {
                        Some(result) => {
                            slot.set_future(my_render(Some(result.to_owned())));
                        }
                        None => {
                            slot.set_future(my_render.clone()(None));
                            let result = future.await;
                            slot.set_future(my_render(Some(result)));
                        }
                    }
                }
            )).await;
        }
    )).await;
}