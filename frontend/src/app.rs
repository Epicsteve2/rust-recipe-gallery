use gloo_net::http::Request;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use validator::Validate;

use crate::components::add_post_form::AddRecipeForm;
use crate::components::footer::Footer;
use crate::components::top_nav_bar::TopNavBar;
use crate::models::{AppError, PostRecipe, Recipe};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rust-recipe-gallery-frontend.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="flex flex-col min-h-screen bg-green-50">
        <TopNavBar/>
        <main class="flex flex-auto">
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
    recipe.validate()?;
    let json_response = Request::post("http://0.0.0.0:7979/api/recipe/new")
        .json(&recipe)?
        .send()
        .await?
        .json::<Recipe>()
        .await?;
    Ok(json_response)
}

#[component]
pub fn AddRecipe(cx: Scope) -> impl IntoView {
    let (post_response, set_response) = create_signal(cx, Ok(None::<Recipe>));
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let post_recipe_action = create_action(
        cx,
        move |(title, ingredients, steps): &(String, String, String)| {
            let title = title.to_string();
            let ingredients = ingredients.to_string();
            let steps = steps.to_string();
            log!("Title: {title}, Ingredients: {ingredients}, Stepes: {steps}");
            async move {
                set_wait_for_response.update(|w| *w = true);
                log!("sending post request");
                let response = post_recipe(title.clone(), ingredients.clone(), steps.clone()).await;
                log!("{response:#?}");
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
            response=post_response
            disabled
        />
    }
}
