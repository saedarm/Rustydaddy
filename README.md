# Dad Joke Image App

A simple Dioxus web app that fetches dad jokes from [icanhazdadjoke](https://icanhazdadjoke.com/) and displays them as images.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started)

```bash
cargo install dioxus-cli
```

## Run

```bash
cd dadjoke-app
dx serve
```

Then open [http://localhost:8080](http://localhost:8080) in your browser.

## How It Works

1. On load (and on button click), the app calls the icanhazdadjoke API with `Accept: application/json` to get a joke ID and text.
2. It then renders the joke as an image using `https://icanhazdadjoke.com/j/{id}.png`.
3. The joke text is also shown below the image as alt-text/caption.
