use std::rc::Rc;
//// use tracing_subscriber::{
////     fmt::{
////         format::{FmtSpan, Pretty},
////         time::UtcTime,
////     },
////     prelude::*,
//// };
use yew::prelude::*;
use yew_router::prelude::*;

mod content;
mod pages;
mod typing;

#[allow(clippy::wildcard_imports)]
use content::*;
#[allow(clippy::wildcard_imports)]
use pages::*;
#[allow(clippy::wildcard_imports)]
use typing::*;

#[function_component(App)]
fn app() -> Html {
    //// let fmt_layer = tracing_subscriber::fmt::layer()
    //// .with_ansi(false)
    //// .with_timer(UtcTime::rfc_3339())
    //// .with_writer(tracing_web::MakeConsoleWriter)
    //// .with_span_events(FmtSpan::ACTIVE);
    //// let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());
    ////
    //// tracing_subscriber::registry()
    //// .with(fmt_layer)
    //// .with(perf_layer)
    //// .init();

    // ****************************** //

    let areas: AreaList = AreaList::from(vec![
        Area::from("development"),
        Area::from("development/backend"),
        Area::from("development/database"),
        Area::from("development/frontend"),
        Area::from("development/game"),
        Area::from("development/systems"),
        Area::from("development/web"),
        Area::from("graphics/digital"),
        Area::from("language"),
        Area::from("scripting"),
    ]);
    let tools: ToolList = ToolList::from(vec![
        Tool::from("Blender"),
        Tool::from("Illustrator"),
        Tool::from("Photoshop"),
        Tool::from("Unity"),
        Tool::from("Visual Studio"),
        Tool::from("Visual Studio Code"),
    ]);

    let skills: SkillList = SkillList::new(vec![
        Skill {
            id: 0,
            name: "Rust".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/systems")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Intermediate,
            description: "Well, this website is made in Rust using the Yew framework and the Trunk application bundler for WebAssembly.".to_string(),
        },
        Skill {
            id: 2,
            name: "Python".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Intermediate,
            description: "I have used Python for a couple small scripts, but not for any full-on programs.".to_string(),
        },
        Skill {
            id: 3,
            name: "C++".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/database")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/systems")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("graphics/digital")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Novice,
            description: "I have mostly only done experimenting in C++ and have not created any scripts or programs with it.".to_string(),
        },
        Skill {
            id: 1,
            name: "Web Development".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/web")),
            ],
            competency: Competency::Advanced,
            description: "While I'm not sure I can call myself an expert with just three and a bit years of it under my belt, I am definitely trying more advanced methods of web development.".to_string(),
        },
    ]);

    let creations: CreationList = CreationList::new(vec![Creation {
        id: 0,
        name: "Web Profile".to_string(),
        completed: KiraDate::new(2023, 9, 16),
        areas: vec![Rc::new(
            areas.get_unwrap_or("development/web", Area::not_found()),
        )],
        tools: vec![Rc::new(
            tools.get_unwrap_or("Visual Studio Code", Tool::not_found()),
        )],
        skills: vec![Rc::new(
            skills.get_unwrap_or("Web Development", Skill::not_found()),
        )],
        description: "A personal profile website I made for my level 3 BTEC course at Coleg y Cymoedd.".to_string(),
    }]);

    let articles: ArticleList = ArticleList::new(vec![Article {
        id: 0,
        title: "<NAME>".to_string(),
        published: KiraDate::new(2023, 8, 17),
        topics: vec![],
        summary: "<SUMMARY>".to_string(),
        content: html! {
            <div>
                <h1>{"<NAME>"}</h1>
            </div>
        },
    }]);

    // ****************************** //

    html! {
        <div id={"root"}>
            <div  id={"header"}>
                <h1>{ "Hello, world!" }</h1>
                <img src={"/icon.png"} alt={"Website Icon"} />
            </div>
            <div id="page-wrapper">
                <div id={"nav"}>
                    <BrowserRouter>
                        <ul>
                            <li>
                                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Skills}>{ "Skills" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Creations}>{ "Creations" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
                            </li>
                        </ul>
                    </BrowserRouter>
                </div>

                <div id={"main"}>
                    <BrowserRouter>
                        <Switch<Route>
                            render={move |route: Route| {
                                tracing::debug!("{}", route.clone());
                                match route {
                                    Route::Home => html! { <Home /> },
                                    Route::Skills => html! { <Skills skills={skills.clone()} /> },
                                    Route::SkillSpecific { skill } => html! {
                                        <ContentComponent
                                            content={skills.get(&skill).map_or_else(|| Content::NotFound, Content::Skill)} />
                                    },
                                    Route::Creations => html! { <Creations creations={creations.clone()} /> },
                                    Route::CreationSpecific { creation } => html! {
                                        <ContentComponent
                                            content={creations.get(&creation).map_or_else(|| Content::NotFound, Content::Creation)} />
                                    },
                                    Route::Articles => html! { <Articles articles={articles.clone()} /> },
                                    Route::ArticleSpecific { article } => html! {
                                        <ContentComponent
                                            content={articles.get(&article).map_or_else(|| Content::NotFound, Content::Article)} />
                                    },
                                    Route::Contact => html! { <Contact /> },
                                    Route::NotFound => html! { <NotFound /> },
                                }
                            }}
                        />
                    </BrowserRouter>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
