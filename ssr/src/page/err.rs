use crate::{state::canisters::auth_canisters_store, utils::event_streaming::events::ErrorEvent};
use gloo::history::{BrowserHistory, History};
use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
struct ServerErrParams {
    err: String,
}

impl ServerErrParams {
    fn map_to_err(&self) -> String {
        match self.err.as_str() {
            _ if self.err.contains("IC agent error") || self.err.contains("error running server function") || self.err.contains("Canister error") || self.err.contains("http fetch error") || self.err.contains("ServerError") || self.err.contains("TypeError") || self.err.contains("CanisterError") => "It looks like our system is taking a coffee break. Try again in a bit, and we'll have it back to work!".to_string(),
            _ => self.err.clone(),
        }
    }
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    let params = use_query::<ServerErrParams>();
    let error = Signal::derive(move || {
        params
            .get()
            .map(|p| p.map_to_err())
            .unwrap_or_else(|_| "Server Error".to_string())
    });

    let canister_store = auth_canisters_store();
    ErrorEvent.send_event(error, canister_store);

    view! { <ErrorView error /> }
}

#[component]
pub fn ErrorView(#[prop(into)] error: MaybeSignal<String>) -> impl IntoView {
    let go_back = move || {
        let history = BrowserHistory::new();

        //go back
        history.back();
    };

    view! {
        <div class="flex flex-col p-10 w-dvw h-dvh bg-black justify-center items-center">
            <img
                src="/img/error-logo.svg"
            />
            <h1 class="p-2 text-2xl md:text-3xl font-bold text-white mb-2">"oh no!"</h1>
            <p class="text-center text-xs md:text-sm text-white/60 w-full md:w-2/3 lg:w-1/3 resize-none p-2 mb-4">
                {error.clone()}
            </p>
            // <textarea
            // prop:value=error
            // disabled
            // rows=3
            // class="text-center bg-white/10 text-xs md:text-sm text-white/60 w-full md:w-2/3 lg:w-1/3 resize-none p-2 mb-4"
            // ></textarea>
            <button
                on:click=move |_| go_back()
                class="bg-primary-600 rounded-full py-3 px-12 max-w-full text-white text-lg md:text-xl"
            >
                Go back
            </button>
        </div>
    }
}
