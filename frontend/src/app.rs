use gloo_net::http::Request;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use validator::Validate;

use crate::components::add_post_form::AddRecipeForm;
use crate::components::footer::Footer;
use crate::components::top_nav_bar::TopNavBar;
use crate::models::{AppError, PostRecipe, Recipe, RecipeComment, RecipeCommentsJson};

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
                    <Route path="/recipes" view=  move |cx| view! { cx, <AllRecipes/> }/>
                    <Route path="/recipes/add" view=  move |cx| view! { cx, <AddRecipe/> }/>
                    <Route path="/recipes/:id" view=  move |cx| view! { cx, <ShowRecipe/> }/>
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

async fn get_all_recipes() -> Result<Vec<Recipe>, String> {
    let json_response = Request::get("http://0.0.0.0:7979/api/recipe")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Recipe>>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json_response)
}

async fn get_recipe_by_id(id: String) -> Result<Recipe, String> {
    let json_response = Request::get(format!("http://0.0.0.0:7979/api/recipe/{id}").as_str())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Recipe>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json_response)
}

async fn get_comments_by_recipe_id(id: String) -> Result<Vec<RecipeComment>, String> {
    let json_response =
        Request::get(format!("http://0.0.0.0:7979/api/recipe/{id}/comments").as_str())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<RecipeCommentsJson>()
            .await
            .map_err(|e| e.to_string())?;
    Ok(json_response.results)
}

async fn delete_comment_by_id(
    recipe_id: String,
    comment_id: String,
) -> Result<RecipeComment, String> {
    let json_response = Request::get(
        format!("http://0.0.0.0:7979/api/recipe/{recipe_id}/comments/{comment_id}").as_str(),
    )
    .send()
    .await
    .map_err(|e| e.to_string())?
    .json::<RecipeComment>()
    .await
    .map_err(|e| e.to_string())?;
    Ok(json_response)
}

async fn delete_recipe_by_id(recipe_id: String) -> Result<Recipe, String> {
    let json_response =
        Request::delete(format!("http://0.0.0.0:7979/api/recipe/{recipe_id}").as_str())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<Recipe>()
            .await
            .map_err(|e| e.to_string())?;
    Ok(json_response)
}

#[component]
pub fn AllRecipes(cx: Scope) -> impl IntoView {
    let async_get_recipes = create_resource(
        cx,
        || (),
        |_| async move {
            log!("Getting recipes...");
            get_all_recipes().await
        },
    );

    view! { cx,
        <Title text="Rust Recipe Gallery - Recipe Gallery"/>
        <div>
            <Suspense
                fallback=move || view! { cx, <p>"Loading..."</p> }
            >
                <ul>
                    {move || async_get_recipes.read(cx).map(|inside| {
                        match inside {
                            Err(e) => view! ( cx, <p>"Error: " {e.to_string()}</p>).into_view(cx),
                            Ok(recipe_list) => recipe_list.iter().map(|recipe| {
                                view! ( cx,
                                    <li>
                                        <a href=format!("/recipes/{}", recipe.id.to_string())>
                                            {recipe.title.clone()}
                                        </a>
                                    </li>
                                )
                            }).collect_view(cx)
                        }
                    })}
                </ul>
            </Suspense>
        </div>
    }
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

#[component]
pub fn ShowRecipe(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let get_id = move || id();

    let async_get_recipe = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        |id| async move { get_recipe_by_id(id).await },
    );

    let async_get_comments_by_recipe_id = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        |id| async move { get_comments_by_recipe_id(id).await },
    );

    // let delete_comment_action =
    //     create_action(cx, move |(recipe_id, comment_id): &(String, String)| {
    //         let recipe_id = recipe_id.to_string();
    //         let comment_id = comment_id.to_string();
    //         async move {
    //             // TODO: wait for response
    //             let response = delete_comment_by_id(recipe_id, comment_id).await;
    //             let navigate = leptos_router::use_navigate(cx);
    //             log!("{response:#?}");
    //             navigate("/", Default::default());
    //         }
    //     });

    let delete_recipe_action = create_action(cx, move |recipe_id: &String| {
        let recipe_id = recipe_id.to_string();
        async move {
            // TODO: wait for response
            let response = delete_recipe_by_id(recipe_id).await;
            let navigate = leptos_router::use_navigate(cx);
            log!("{response:#?}");
            match navigate("/recipes", Default::default()) {
                Err(e) => log!("{e:#?}"),
                Ok(_) => {}
            }
        }
    });

    // let delete_recipe_action_dispatch = move || delete_recipe_action.dispatch(id());

    view! { cx,
        <Title text=format!("Rust Recipe Gallery - Recipe {}", get_id())/>
        <div>
            <Suspense fallback=move || view! (cx, <p>"Loading..."</p>)>
                {move || async_get_recipe.read(cx).map(|inside_some| {
                    match inside_some {
                        Err(e) => view! ( cx, <p>"Error: " {e.to_string()}</p>).into_view(cx),
                        Ok(recipe) =>
                            view! ( cx,
                                <li>
                                    <div>{recipe.title}</div>
                                    <div>{recipe.ingredients}</div>
                                    <div>{recipe.body}</div>
                                    <button on:click= move |_| delete_recipe_action.dispatch(id())>
                                        "Delete?"
                                    </button>
                                    <button>"Edit?"</button>
                                </li>
                            ).into_view(cx)
                    }
                })}
            </Suspense>
            <Suspense fallback=move || view! (cx, <p>"Loading comments..."</p>)>
                {move || async_get_comments_by_recipe_id.read(cx).map(|inside_some| {
                    match inside_some {
                        Err(e) => view! ( cx, <p>"Error: " {e.to_string()}</p>).into_view(cx),
                        Ok(comments) =>
                            comments.iter().map(|comment| {
                                view! ( cx,
                                    <li>
                                        <div>{comment.comment.clone()}</div> // idk why i have to clone
                                        <button>
                                            "Delete?"
                                        </button>
                                        <button>"Edit?"</button>
                                    </li>
                                ).into_view(cx)
                            }).collect_view(cx)
                    }
                })}
            </Suspense>
        </div>
    }
}
