use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct JokeResponse {
    id: String,
    joke: String,
}

const STYLE: &str = r#"
    body {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        background-color: #1a1a2e;
        color: #e0e0e0;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        margin: 0;
    }
    .container {
        text-align: center;
        padding: 2rem;
        max-width: 700px;
    }
    h1 {
        color: #e94560;
        margin-bottom: 1.5rem;
    }
    .joke-image {
        max-width: 100%;
        border-radius: 12px;
        box-shadow: 0 4px 20px rgba(233, 69, 96, 0.3);
        margin: 1.5rem 0;
    }
    .btn {
        background-color: #e94560;
        color: white;
        border: none;
        padding: 0.8rem 2rem;
        font-size: 1.1rem;
        border-radius: 8px;
        cursor: pointer;
        transition: background-color 0.2s;
    }
    .btn:hover {
        background-color: #c73652;
    }
    .loading {
        font-size: 1.2rem;
        color: #aaa;
        padding: 2rem;
    }
    .error {
        color: #ff6b6b;
        padding: 1rem;
    }
"#;

async fn fetch_joke() -> Result<JokeResponse, reqwest::Error> {
    reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .header("User-Agent", "DioxusDadJokeApp (https://github.com)")
        .send()
        .await?
        .json::<JokeResponse>()
        .await
}

#[component]
fn App() -> Element {
    let mut joke = use_signal::<Option<Result<JokeResponse, String>>>(|| None);
    let mut loading = use_signal(|| false);

    let fetch = move |_| {
        spawn(async move {
            loading.set(true);
            let result = fetch_joke().await.map_err(|e| e.to_string());
            joke.set(Some(result));
            loading.set(false);
        });
    };

    // Fetch on first render
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            let result = fetch_joke().await.map_err(|e| e.to_string());
            joke.set(Some(result));
            loading.set(false);
        });
    });

    rsx! {
        style { {STYLE} }
        div { class: "container",
            h1 { "🤣 Dad Joke of the Moment" }

            if *loading.read() {
                p { class: "loading", "Fetching a groan-worthy joke..." }
            } else {
                match joke.read().as_ref() {
                    Some(Ok(j)) => rsx! {
                        img {
                            class: "joke-image",
                            src: "https://icanhazdadjoke.com/j/{j.id}.png",
                            alt: "{j.joke}",
                        }
                        p { style: "color: #888; font-style: italic;", "{j.joke}" }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "error", "Failed to fetch joke: {e}" }
                    },
                    None => rsx! {
                        p { class: "loading", "Click the button to get a joke!" }
                    },
                }
            }

            button {
                class: "btn",
                onclick: fetch,
                disabled: *loading.read(),
                if *loading.read() { "Loading..." } else { "Give Me Another! 🎲" }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
