use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::animations::Animations;
use crate::components::navbar::NavBar;
use crate::components::page_not_found::PageNotFound;
use crate::components::page_transition::PageTransition;
use crate::old_showcase::showcase_component::ShowcaseGallery;
use crate::pages::blog::index::Blog;
use crate::pages::docs::index::Docs;
use crate::pages::docs::index::DocsLanding;
use crate::pages::home::index::Home;

// Turn off rustfmt since we're doing layouts and routes in the same enum
#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(NavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        #[transition(Fade)]
        Home {},

        // Wrap the next routes in a layout and a nest
        #[nest("/docs")]
        #[layout(Docs)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            #[transition(SlideLeft)]
            DocsLanding {},

            #[route("/transitions")]
            #[transition(SlideUp)]
            PageTransition {},

            // At "/blog/:name", we want to show a specific blog post, using the name slug
            #[route("/animations")]
            #[transition(SlideDown)]
            Animations {},

        // We need to end the blog layout and nest
        // Note we don't need either - we could've just done `/blog/` and `/blog/:name` without nesting,
        // but it's a bit cleaner this way
        #[end_layout]
        #[end_nest]

        #[route("/blog")]
        #[transition(SlideDown)]
        Blog {},

        #[route("/old_showcase")]
        #[transition(Fade)]
        ShowcaseGallery {},


    // And the regular page layout
    #[end_layout]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
