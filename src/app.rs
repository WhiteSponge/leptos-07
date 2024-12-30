use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, ProtectedRoute, Route, Router},
    path, WildcardSegment,
};
use models::user::{User, UserStore, UserStoreFields, UserStoreStoreFields};
use pages::{HomePage, InfoPage, SettingsPage, StorePage};
use reactive_stores::Store;

mod models;
mod pages;
mod server_functions;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let store = Store::new(UserStore {
        user: User::new("David", "david@mymail.com"),
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-07.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css" />

        // sets the document title
        <Title text="Welcome to Leptos"/>
        // content for this welcome page
        <Router>
            <main>
                <FlatRoutes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/info") view=InfoPage/>
                    <Route path=path!("/store") view=move || {
                        view! {
                            <StorePage user=store.user()/>
                        }
                    } />
                    <ProtectedRoute
                        path=path!("/settings")
                        condition=move || Some(store.user().is_authenticated().get())
                        redirect_path=|| "/"
                        view=SettingsPage />
                    <Route path=WildcardSegment("any") view=NotFound/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
