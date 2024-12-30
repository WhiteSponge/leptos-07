use leptos::prelude::*;
use leptos_router::components::A;

const BUTTON_STYLE: &str =
    "bg-purple-500 text-white text-2xl rounded w-[24rem] py-4 mt-8 focus:w-[23rem] focus:py-3 transition-all ease-in-out duration-750";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full h-screen bg-stone-900 overlay-x-hide mx-auto items-center justify-center align-center">
            <div class="flex flex-col max-w-[64rem] mx-auto items-center justify-center">
                <p class="text-white text-4xl pt-24 font-bold">"WELCOME TO LEPTOS v0.7!"</p>
                <A href="/info">
                    <button class=BUTTON_STYLE>"Info Page"</button>
                </A>
                <A href="/store">
                    <button class=BUTTON_STYLE>"Store Page"</button>
                </A>
                <A href="/settings">
                    <button class=BUTTON_STYLE>"Protected Settings Page"</button>
                </A>

            </div>
        </div>
    }
}
