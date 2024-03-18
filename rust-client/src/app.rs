
use leptos::*;
use leptos::{error::Result, html::Input};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub label: String,
}


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-client.css"/>

        // sets the document title
        <Title text="Leptos Perf Testing page"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    let (data, set_data) = create_signal(Vec::<Entry>::new());
    // let (count, set_count) = create_signal(0);

    let retrieve_bibupkas = create_action(|count: &u32| {
        let input = count.to_owned();

        async move { get_data(input).await }
    });

    let entries = retrieve_bibupkas.value();

    create_effect(move |_| {
        if let Some(Ok(data)) = entries() {
            set_data(data);
        }
    });

    let input_ref = create_node_ref::<Input>();

    let list_view = move || {
        data()
            .iter()
            .map(|item| view! { <li>{ item.id.clone() }  { item.label.clone() }</li> })
            .collect_view()
    };

    view! {
        <div>
            <label for="entry-count">Data Entry Count:</label>
            <input type="number" id="entry-count" name="entry-count" node_ref=input_ref />
            <button id="generate-data" on:click=move |ev| {
                ev.prevent_default();
                let input: u32 = input_ref.get().expect("input to exist").value().parse().expect("input to contain a valid number");

                retrieve_bibupkas.dispatch(input);
            }>"Get Data"</button>
            <button id="clear" on:click=move |ev| {
                ev.prevent_default();
                set_data.set(Vec::new());
            }>"Clear"</button>
            Data:
            <ul>
                { list_view }
            </ul>
        </div>
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

async fn get_data(count: u32) -> Result<Vec<Entry>> {
    let res = reqwasm::http::Request::get(&format!("http://localhost:8080/api/data/{}", count))
        .send()
        .await?
        .json::<Vec<Entry>>()
        .await?;

    Ok(res)
}
