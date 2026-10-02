use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::navbar::NavBar;
use crate::components::page_not_found::PageNotFound;
use crate::components::page_transition::PageTransition;
use crate::pages::basic_guide::BasicAnimationGuide;
use crate::pages::blog::index::Blog;
use crate::pages::complex_guide::ComplexAnimationGuide;
use crate::pages::docs::index::Docs;
use crate::pages::docs::index::DocsLanding;
use crate::pages::home::index::Home;
use crate::pages::intermediate_guide::IntermediateAnimationGuide;
use crate::pages::motion_style_guide::MotionStyleGuide;
use crate::pages::presence_guide::PresenceGuide;
use crate::showcase::showcase_component::ShowcaseGallery;

#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[transition(Fade)]
        Home {},

        #[nest("/docs")]
        #[layout(Docs)]
            #[route("/")]
            #[transition(SlideLeft)]
            DocsLanding {},

            #[route("/transitions")]
            #[transition(SlideLeft)]
            PageTransition {},

            #[route("/basic_guide")]
            #[transition(SlideLeft)]
            BasicAnimationGuide {},

            #[route("/intermediate_guide")]
            #[transition(SlideLeft)]
            IntermediateAnimationGuide {},

            #[route("/motion_style")]
            #[transition(SlideLeft)]
            MotionStyleGuide {},

            #[route("/complex_guide")]
            #[transition(SlideLeft)]
            ComplexAnimationGuide {},

            #[route("/presence")]
            #[transition(SlideLeft)]
            PresenceGuide {},



        #[end_layout]
        #[end_nest]

        #[route("/blog")]
        #[transition(SlideDown)]
        Blog {},

        #[redirect("/old_showcase", || Route::ShowcaseGallery { demo: None })]
        #[route("/examples?:demo")]
        #[transition(Fade)]
        ShowcaseGallery {
            demo: Option<String>,
        },

    #[end_layout]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
