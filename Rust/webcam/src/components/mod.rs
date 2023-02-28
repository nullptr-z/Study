use sycamore::prelude::*;

mod controls;
mod video;

use crate::AppState;
pub use controls::*;
use tracing::info;
pub use video::Video;

#[component]
pub async fn App<G: Html>(ctx: BoundedScope<'_, '_>) -> View<G> {
    let state = AppState::new().await;
    provide_context(ctx, state);

    view! {ctx,
            div(class="container p-2"){
                Video()
        }
    }
}
