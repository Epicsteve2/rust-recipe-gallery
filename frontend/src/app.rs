use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::*;
use crate::components::add_post_form::AddRecipeForm;
use crate::components::footer::Footer;
use crate::components::top_nav_bar::TopNavBar;
use crate::models::Recipe;

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
                    <Route path="/recipes/:id/edit" view=  move |cx| view! { cx, <EditRecipe/> }/>
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
        <div class="w-full max-w-xl text-black mx-auto py-8">
            <Suspense
                fallback=move || view! { cx, <h1 class="text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1> }
            >
                <ul class="flex flex-auto flex-col">
                    {move || async_get_recipes.read(cx).map(|inside| {
                        match inside {
                            Err(e) => view! ( cx, <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(cx),
                            Ok(recipe_list) => recipe_list.iter().map(|recipe| {
                                view! ( cx,
                                    <li class="">
                                        // <a class="font-medium text-green-600 hover:underline" href=format!("/recipes/{}", recipe.id.to_string())>
                                        <a class="m-4 p-6 block font-medium border-gray-600 rounded-lg hover:bg-green-200 bg-green-400" href=format!("/recipes/{}", recipe.id.to_string())>
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
pub fn EditRecipe(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    // let id_string = id();

    let async_get_recipe = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        |id| async move { get_recipe_by_id(id).await },
    );

    let (patch_response, set_response) = create_signal(cx, Ok(None::<Recipe>));
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);
    let disabled = Signal::derive(cx, move || wait_for_response.get());
    let patch_recipe_action = create_action(
        cx,
        move |(title, ingredients, steps): &(String, String, String)| {
            let title = title.to_string();
            let ingredients = ingredients.to_string();
            let steps = steps.to_string();
            let id_string = id();

            async move {
                set_wait_for_response.update(|w| *w = true);
                log!("sending post request");
                let response = patch_recipe_by_id(id_string, title, ingredients, steps).await;
                log!("{response:#?}");
                set_response.update(|w| *w = response.map(|inside| Some(inside)));
                log!("finished sending post request");
                set_wait_for_response.update(|w| *w = false);
            }
        },
    );
    view! { cx,
        <Title text="Rust Recipe Gallery - Edit Recipe"/>
        <Suspense fallback=move || view! (cx, <h1 class="mt-5 text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1>)>
            <AddRecipeForm
                action=patch_recipe_action
                response=patch_response
                disabled
                title_fallback=match async_get_recipe.read(cx).map(|inside_some| {
                    log!("{:#?}", inside_some);
                    match inside_some {
                        Err(_) => "".to_string(),
                        Ok(recipe) => recipe.title
                    }
                }) {
                    None => "".to_string(),
                    Some(string_inside) => string_inside
                }
            />
        </Suspense>
    }
}

#[component]
pub fn ShowRecipe(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let generate_title = move || format!("Rust Recipe Gallery - Recipe {}", id());
    let generate_edit_href = move || format!("/recipes/{}/edit", id());

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
            // TODO: wait for response, or not. idc
            let response = delete_recipe_by_id(recipe_id).await;
            let navigate = leptos_router::use_navigate(cx);
            log!("{response:#?}");
            match response {
                Err(_) => {}
                Ok(_) => match navigate("/recipes", Default::default()) {
                    Err(e) => log!("{e:#?}"),
                    Ok(_) => {}
                },
            }
        }
    });

    // let delete_recipe_action_dispatch = move || delete_recipe_action.dispatch(id());

    view! { cx,
        <Title text=generate_title/>
        <div class="max-w-2xl rounded-xl w-full mx-auto py-8">
            <Suspense fallback=move || view! (cx, <h1 class="mt-5 text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1>)>
                {move || async_get_recipe.read(cx).map(|inside_some| {
                    match inside_some {
                        Err(e) => view! ( cx, <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(cx),
                        Ok(recipe) =>
                            view! ( cx,
                                <div><strong>"Title: "</strong>{recipe.title}</div>
                                <div><strong>"Ingredients: "</strong>{recipe.ingredients}</div>
                                <div><strong>"Steps: "</strong>{recipe.body}</div>
                                <button class="mt-6 mr-5 bg-green-300 hover:bg-green-200 p-2 rounded-md" on:click= move |_| delete_recipe_action.dispatch(id())>
                                    "Delete"
                                </button>
                                <a class="bg-green-300 hover:bg-green-200 p-2 rounded-md" href=generate_edit_href>"Edit"</a>
                            ).into_view(cx)
                    }
                })}
            </Suspense>
            <Suspense fallback=move || view! (cx, <h1 class="text-center p-6 bg-green-300 rounded-lg">"Loading Comments..."</h1>)>
                {move || async_get_comments_by_recipe_id.read(cx).map(|inside_some| {
                    match inside_some {
                        Err(e) => view! ( cx, <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(cx),
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
