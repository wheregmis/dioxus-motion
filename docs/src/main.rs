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
        nav { id: "navbar take it",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::DocsLanding {}, "Docs" }
            Link { to: Route::Blog {}, "Blog" }
        }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

#[component]
fn Docs() -> Element {
    rsx! {
        h1 { "Blog" }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn DocsLanding() -> Element {
    rsx! {
        h1 { "Docs Landing Page" }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn PageTransition() -> Element {
    rsx! {
        h1 { "Blog" }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn Animations() -> Element {
    rsx! {
        h1 { "Blog" }
        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
