use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            Router::<Route> {}
        }
    });
}

// Turn off rustfmt since we're doing layouts and routes in the same enum
#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(NavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        #[transition(SlideDown)]
        Home {},

        // Wrap the next routes in a layout and a nest
        #[nest("/docs")]
        #[layout(Docs)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            #[transition(SlideUp)]
            DocsLanding {},

            #[route("/transitions")]
            #[transition(SlideUp)]
            PageTransition {},

            // At "/blog/:name", we want to show a specific blog post, using the name slug
            #[route("/animations")]
            #[transition(SlideRight)]
            Animations { },

        // We need to end the blog layout and nest
        // Note we don't need either - we could've just done `/blog/` and `/blog/:name` without nesting,
        // but it's a bit cleaner this way
        #[end_layout]
        #[end_nest]

        #[route("/blog")]
        #[transition(SlideDown)]
        Blog {},

    // And the regular page layout
    #[end_layout]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-sm py-4 px-6",
            div { class: "max-w-4xl mx-auto flex gap-6",
                Link {
                    class: "text-gray-600 hover:text-gray-900 font-medium",
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: "text-gray-600 hover:text-gray-900 font-medium",
                    to: Route::DocsLanding {},
                    "Docs"
                }
                Link {
                    class: "text-gray-600 hover:text-gray-900 font-medium",
                    to: Route::Blog {},
                    "Blog"
                }
            }
        }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Welcome to the Dioxus Blog!" }
        }
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Welcome to the Dioxus Blog!" }
        }
    }
}

#[component]
fn Docs() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Documentation" }
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
fn DocsLanding() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Docs Landing Page" }
            p { class: "text-gray-600 mb-4", "This is the landing page for the documentation." }

            Link {
                class: "text-blue-600 hover:underline",
                to: Route::PageTransition {},
                "Page Transitions"
            }
            Link {
                class: "text-blue-600 hover:underline",
                to: Route::Animations {},
                "Animations"
            }
        }
    }
}

#[component]
fn PageTransition() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Page Transitions" }
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
fn Animations() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Animations" }
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Page not found" }
            p { class: "text-gray-600 mb-4",
                "We are terribly sorry, but the page you requested doesn't exist."
            }
            pre { class: "bg-red-50 text-red-600 p-4 rounded-md font-mono text-sm",
                "log:\nattemped to navigate to: {route:?}"
            }
        }
    }
}
