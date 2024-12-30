use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="w-full h-screen bg-stone-900 overlay-x-hide mx-auto items-center justify-center align-center">
            <div class="flex flex-col max-w-[64rem] mx-auto items-center justify-center">
                <p class="text-white text-4xl pt-24 font-bold">"WELCOME TO SETTINGS (PROTECTED) PAGE v0.7!"</p>
                <A href="/">
                    <button class="bg-purple-500 text-white text-2xl rounded px-24 py-4 mt-8">"Back to Home"</button>
                </A>

            </div>
        </div>

    }
}
