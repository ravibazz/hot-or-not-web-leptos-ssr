use leptos::*;
use leptos_router::use_navigate;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::{
    auth::logout_identity,
    component::loading::Loading,
    consts::ACCOUNT_CONNECTED_STORE,
    state::auth::auth_state,
    try_or_redirect,
    utils::event_streaming::events::{LogoutClicked, LogoutConfirmation},
};

#[component]
pub fn Logout() -> impl IntoView {
    LogoutClicked.send_event();

    let auth_res = create_local_resource(
        || (),
        move |_| async move {
            let id = try_or_redirect!(logout_identity().await);

            LogoutConfirmation.send_event();

            let auth = auth_state();
            auth.set(id);

            let (_, write_account_connected, _) =
                use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
            write_account_connected(false);

            let navigate = use_navigate();
            navigate("/menu", Default::default());
        },
    );

    view! {
        <Loading text="Logging out...".to_string()>
            <Suspense>{move || auth_res.get().map(|_| ())}</Suspense>
        </Loading>
    }
}
