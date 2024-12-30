use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::server_functions::get_all_users;

#[component]
pub fn InfoPage() -> impl IntoView {
    let (paragraph, set_paragraph) = signal(String::from(""));

    let get_users_info = Resource::new(|| (), |_| async move { get_all_users().await });

    view! {

        <div class="w-full h-screen bg-stone-900 overlay-x-hide mx-auto items-center justify-center align-center">
            <div class="flex flex-col max-w-[64rem] mx-auto items-center justify-center">
                <div class="text-white text-4xl pt-24 font-bold">"WELCOME TO INFO PAGE v0.7!"</div>
                <A href="/">
                    <button class="bg-purple-500 text-white text-2xl rounded px-24 py-4 mt-8">"Back to Home"</button>
                </A>

                <div class="mt-20 border-white border-t-2 max-w-[24rem] w-full mx-auto items-center align-center justify-center flex flex-col">
                    <div class="text-white mt-8">"Input Change"</div>
                    <input type="text"
                        bind:value=(paragraph,set_paragraph)
                        class="rounded px-4 outline-none py-2 mt-8 text-black" />
                    <div class="text-white mt-4">{paragraph}</div>
                </div>

                <div class="mt-16 border-white border-t-2 max-w-[24rem] w-full mx-auto items-center align-center justify-center flex flex-col text-white">
                    <div class="text-white mt-4 mb-4">"Users retrieved from server function:"</div>
                    <Suspense fallback=move || {
                        view! { <p>"Loading users..."</p> }
                    }>
                    {
                        move || {
                            get_users_info.get().map(|data| {
                                match data {
                                    Ok(users_data) => {
                                        Either::Left(users_data.into_iter().map(|each_user| view! {
                                            <div>{each_user.clone().name}</div>
                                        }).collect_view())
                                    },
                                    Err(_) => {
                                        Either::Right(
                                            view! { <div>"No users yet..."</div> }
                                        )
                                    }
                                }
                            })
                        }
                    }
                    </Suspense>
                </div>
            </div>
        </div>

    }
}
