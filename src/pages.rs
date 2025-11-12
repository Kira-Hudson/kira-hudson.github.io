use std::fmt::Display;

use crate::content::{
    ArticleListProperties, CreationListProperties, SkillListProperties,
    ToListItem,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/skills")]
    Skills,
    #[at("/skills/:skill")]
    SkillSpecific { skill: String },
    #[at("/creations")]
    Creations,
    #[at("/creations/:creation")]
    CreationSpecific { creation: String },
    #[at("/articles")]
    Articles,
    #[at("/articles/:article")]
    ArticleSpecific { article: String },
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Home => write!(f, "/"),
            Self::Skills => write!(f, "/skills"),
            Self::SkillSpecific { skill } => write!(f, "/skills/{}", skill),
            Self::Creations => write!(f, "/creations"),
            Self::CreationSpecific { creation } => write!(f, "/creations/{}", creation),
            Self::Articles => write!(f, "/articles"),
            Self::ArticleSpecific { article } => write!(f, "/articles/{}", article),
            Self::Contact => write!(f, "/contact"),
            Self::NotFound => write!(f, "/404"),
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>

            <p>{"Hi, I'm Kira Hudson, and I somehow exist, unfortunately for you. I am a trans woman, currently studying Computer Games Design at the University of South Wales."}</p>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills(SkillListProperties { skills }: &SkillListProperties) -> Html {
    html! {
        <div>
            <h1>{"Skills"}</h1>

            <p>{"These are some of the skills I have accrued over my few decades of life."}</p>

            <hr />

            <div>{skills.to_list_item()}</div>
        </div>
    }
}

#[function_component(Creations)]
pub fn creations(CreationListProperties { creations }: &CreationListProperties) -> Html {
    html! {
        <div>
            <h1>{"Creations"}</h1>

            <p>{"These are some of my creations that I'd like to share with the world."}</p>

            <hr />

            <div>{creations.to_list_item()}</div>
        </div>
    }
}

#[function_component(Articles)]
pub fn articles(ArticleListProperties { articles }: &ArticleListProperties) -> Html {
    html! {
        <div>
            <h1>{"Articles"}</h1>

            <p>{"I am working on implementing parsing markdown files for articles. Oh, and also what to write. More is yet to come..."}</p>

            <hr />

            <div>{articles.to_list_item()}</div>
        </div>
    }
}

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <h1>{"Contact"}</h1>

            <div class={"contact-details-container"}>
                <a
                    class={"contact-icon"}
                    href={"mailto:kira.hudson.v0@gmail.com"}
                    title={"kira.hudson.v0@gmail.com"}
                    id={"Email"}
                    target={"_blank"}
                >
                    <i class={"fa-solid fa-square-envelope"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://github.com/Kira-Hudson/"}
                    id={"GitHub"}
                    title={"@Kira-Hudson"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-github"}></i>
                </a>
            </div>
        </div>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{"Error 404"}</h1>

            <p>{"Error 404 : Page Not Found"}</p>

            <p>{"Please navigate your way back to the main site"}</p>

            <p>
                <b>{"EMERGENCY EXIT:"}</b>{" "}<a href={"/"}>{"Home"}</a>
            </p>
        </div>
    }
}
