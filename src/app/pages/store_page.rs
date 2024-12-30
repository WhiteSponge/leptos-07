use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::{Field, Store};

use crate::app::models::User;
use crate::app::UserStoreFields;

const BUTTON_STYLE: &str = "bg-purple-500 text-white text-2xl rounded px-24 py-4 mt-8";

#[component]
pub fn StorePage(#[prop(into)] user: Field<User>) -> impl IntoView {
    let on_click = move |_| {
        user.is_authenticated().set(!user.is_authenticated().get());
    };

    view! {
        <div class="w-full h-screen bg-stone-900 overlay-x-hide mx-auto items-center justify-center align-center">
            <div class="flex flex-col max-w-[64rem] mx-auto items-center justify-center">
                <p class="text-white text-4xl pt-24 font-bold">"WELCOME TO STORE PAGE v0.7!"</p>
                <A href="/">
                    <button class=BUTTON_STYLE>"Back to Home"</button>
                </A>
                <hr class="bg-white" />
                <div class="pt-12 border-b-2 border-white pb-4">
                    <p class="text-white text-4xl font-bold">"Name: " {move || user.name().get()}</p>
                    <p class="text-white text-4xl font-bold">"Email: " {move || user.email().get()}</p>
                    <p class="text-white text-4xl font-bold">"Authenticated: " {move || user.is_authenticated().get()}</p>
                </div>
                <div class="pt-4">
                    <button class=BUTTON_STYLE on:click=on_click>
                        <Show when=move || { !user.is_authenticated().get() }>
                            "Login"
                        </Show>
                        <Show when=move || { user.is_authenticated().get() }>
                            "Logout"
                        </Show>

                    </button>
                </div>
            </div>
        </div>
    }
}
