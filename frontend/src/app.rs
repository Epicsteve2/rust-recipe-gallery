use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use crate::components::add_post_form::AddRecipeForm;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rust-recipe-gallery-frontend.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="flex flex-col min-h-screen bg-green-50">
        <TopNavBar/>
        // idk if i want this
        <main class="flex flex-auto">
        // <main class="">
            <Router>
                <Routes>
                    <Route path="/" view=  move |cx| view! { cx, <Home/> }/>
                    <Route path="/recipes/add" view=  move |cx| view! { cx, <AddRecipe/> }/>
                </Routes>
            </Router>
        </main>
        <Footer/>
        </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Rust Recipe Gallery"/>
        <div class="bg-gradient-to-tl from-lime-300 to-lime-100 text-black font-mono flex flex-auto items-center justify-center">
            <h1 class="m-auto text-center">"Cook!"</h1>
        </div>
    }
}

#[component]
pub fn TopNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="bg-green-600 text-white">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-6">
                <div><a class="font-medium" href="/">"Rust Recipe Gallery"</a></div>
                <div class="w-auto">
                    <ul class="flex">
                        <li><a class="pr-4 hover:text-green-200" href="/recipes">"Gallery"</a></li>
                        <li><a class="pr-4 hover:text-green-200" href="/recipes/add">"Add Recipe"</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[derive(Debug, Serialize, Validate)]
pub struct PostRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    pub title: String,
    #[validate(length(min = 2, message = "must have at least 1 ingredient"))]
    pub ingredients: String,
    #[validate(length(min = 2, message = "must have a body"))]
    pub body: String,
}

#[derive(Debug, serde::Deserialize)] // clone for signals
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub ingredients: String,
    pub body: String,
}

#[derive(Error, Debug)] // clone for signals
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    GlooError(#[from] gloo_net::Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

async fn post_recipe(
    title: String,
    ingredients: String,
    steps: String,
) -> Result<Recipe, AppError> {
    let recipe = PostRecipe {
        title,
        ingredients,
        body: steps,
    };
    // dbg!(&recipe);
    recipe.validate()?;
    let json_response = Request::post("http://0.0.0.0:7979/api/recipe/new")
        .json(&recipe)?
        .send()
        .await?
        // .map_err(|e| log!("Error: {e:#?}"))
        .json::<Recipe>()
        .await?;

    // .json()
    // .await;
    // dbg!(json_response);
    // log!("{:?}", json_response);
    // todo!()
    Ok(json_response)
}

#[component]
pub fn AddRecipe(cx: Scope) -> impl IntoView {
    let (post_response, set_response) = create_signal(cx, Ok(None::<Recipe>));

    // let (post_error, set_post_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let post_recipe_action = create_action(
        cx,
        move |(title, ingredients, steps): &(String, String, String)| {
            let title = title.to_string();
            let ingredients = ingredients.to_string();
            let steps = steps.to_string();
            log!("{title}, {ingredients}, {steps}");
            async move {
                set_wait_for_response.update(|w| *w = true);
                log!("sending post request");
                let response = post_recipe(title.clone(), ingredients.clone(), steps.clone()).await;
                match &response {
                    Ok(recipe) => {
                        log!("{recipe:#?}")
                    }
                    Err(e) => {
                        log!("{e:#?}")
                    }
                }
                set_response.update(|w| *w = response.map(|inside| Some(inside)));
                log!("finished sending post request");
                set_wait_for_response.update(|w| *w = false);
            }
        },
    );
    let disabled = Signal::derive(cx, move || wait_for_response.get());

    view! { cx,
        <Title text="Rust Recipe Gallery - Add Recipe"/>
        <AddRecipeForm
            action=post_recipe_action
            // error=post_error.into()
            response=post_response
            disabled
        />
    }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="bg-emerald-600 mt-auto">
            <div class="w-full max-w-screen-xl mx-auto p-4 md:py-8 text-center">
                <ul class="flex flex-wrap items-center mt-3 sm:mt-0 text-sm font-medium text-white justify-center">
                    <li>
                        <a href="/" class="mr-4 hover:underline md:mr-6 ">"Home"</a>
                    </li>
                    <li>
                        <a href="https://github.com/Epicsteve2/rust-recipe-gallery" class="hover:underline">"GitHub"</a>
                    </li>
                </ul>
            </div>
        </footer>
    }
}
