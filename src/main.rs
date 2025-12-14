mod components;
mod utils;

use crate::utils::get_love_sentence;
use components::card::*;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CUTE_IMAGE: Asset = asset!("/assets/us.jpg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta {
            name: "description",
            content: "All my love for you..."
        }
        document::Title {
            "Love Sentences"
        }
        AppContent {}

    }
}

struct LoveSentence {
    sentence: String,
    loading: bool,
}

impl LoveSentence {
    fn new_sentence(&mut self) {
        self.loading = true;
        self.sentence = get_love_sentence();
        self.loading = false;
    }

    fn new() -> Self {
        Self {
            sentence: "Ich liebe dich bis zum Mond und zurück.".to_string(),
            loading: false,
        }
    }
}

#[component]
pub fn PageHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center space-y-8",
            h1 {
                class: "text-xl font-bold text-pink-700 text-center mb-4 italic",
                "Some love sentences for you..."
            },
            img {
                class: "justify-center w-[50%] h-[50%]",
                src: CUTE_IMAGE,
                alt: "Two cats (one black and one white) hugging each other (a representaton of our love)",
            }
            p {
                class: "text-lg text-gray-500 text-center italic mb-8",
                "A picture of us if we were cats!🐱❤️🐱"
            }
        }
    }
}

#[component]
pub fn AppContent() -> Element {
    let mut state = use_signal(LoveSentence::new);

    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen p-4 bg-gradient-to-br from-white via-pink-100 to-pink-300",
            PageHeader {},
            Card {
                class: "w-full max-w-xl shadow-lg",
                CardHeader {
                    CardTitle {
                        class: "text-center text-lg",
                        "Do you believe in love?💕"
                    }
                    CardDescription {
                        class: "text-center",
                        "Well, your girlfriend (Clelia) does - and because of you!🥺👉👈"
                    }
                },
                CardContent {
                    p {
                        class: "text-center",
                        "So here is a sentence to show you all her love:"
                    },
                    br {},
                    p {
                        class: "font-medium text-pink-600 italic text-center",
                        {state.read().sentence.clone()}
                    }
                    br {},
                    p {
                        class: "text-center",
                        "Want to see another love sentence? Click below!👇"
                    },
                },
                CardFooter {
                    button {
                        onclick: move |_| state.write().new_sentence(),
                        class: "w-full bg-gradient-to-br from-pink-300 via-red-400 to-red-600 mb-4 mt-2",
                        {
                            if state.read().loading {
                                "Loading..."
                            } else {
                                "🤍🤍🤍"
                            }
                        },
                    }
                }
            }
        }
    }
}
