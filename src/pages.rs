use std::fmt::Display;

use crate::content::{
    AchievementListProperties, ArticleListProperties, CreationListProperties, SkillListProperties,
    ToHtml,
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
    #[at("/achievements")]
    Achievements,
    #[at("/achievements/:achievement")]
    AchievementSpecific { achievement: String },
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
            Self::Achievements => write!(f, "/achievements"),
            Self::AchievementSpecific { achievement } => write!(f, "/achievements/{}", achievement),
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

            <p>{"Hi, I'm Kira Hudson, and I somehow exist, unfortunately for you."}</p>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills(SkillListProperties { skills }: &SkillListProperties) -> Html {
    html! {
        <div>
            <h1>{"Skills"}</h1>

            <p>{"These are (some of) my skills."}</p>

            <hr />

            <div>{skills.to_html()}</div>
        </div>
    }
}

#[function_component(Achievements)]
pub fn achievements(
    AchievementListProperties { achievements }: &AchievementListProperties,
) -> Html {
    html! {
        <div>
            <h1>{"Achievements"}</h1>

            <p>{"These are (some of) my achievements."}</p>

            <hr />

            <div>{achievements.to_html()}</div>
        </div>
    }
}

#[function_component(Creations)]
pub fn creations(CreationListProperties { creations }: &CreationListProperties) -> Html {
    html! {
        <div>
            <h1>{"Creations"}</h1>

            <p>{"These are (some of) my creations."}</p>

            <hr />

            <div>{creations.to_html()}</div>
        </div>
    }
}

#[function_component(Articles)]
pub fn articles(ArticleListProperties { articles }: &ArticleListProperties) -> Html {
    html! {
        <div>
            <h1>{"Articles"}</h1>

            <p>{"I am working on implementing parsing markdown files for articles."}</p>

            <hr />

            <div>{articles.to_html()}</div>
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
