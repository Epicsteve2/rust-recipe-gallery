use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::*;
use crate::components::add_post_form::AddRecipeForm;
use crate::components::footer::Footer;
use crate::components::top_nav_bar::TopNavBar;
use crate::models::Recipe;

// example
// #[component]
// pub fn App() -> impl IntoView {
//     provide_meta_context(cx);

//     view! {
//         cx,
//         <Router>
//             <Routes>
//                 <Route path="/:id" view=  move |cx| view! { cx, <Component/> }/>
//             </Routes>
//         </Router>
//     }
// }
// async fn fetch_data(value: String) -> String {
//     // TimeoutFuture::new(1_000).await;
//     value + "!"
// }
// #[component]
// pub fn Component() -> impl IntoView {
//     let params = use_params_map(cx);
//     let async_get_data = create_resource(
//         cx,
//         move || params().get("id").cloned().unwrap_or_default(),
//         move |id| async move {
//             let fetched_data = fetch_data(id).await;
//             fetched_data
//         },
//     );

//     view! {
//         cx,
//         <Suspense
//             fallback=move || view! (cx, <p>"Loading..."</p>)
//         >
//             <InnerComponent
//                 data=async_get_data
//             />
//         </Suspense>
//     }
// }
// #[component]
// pub fn InnerComponent(, data: Resource<String, String>) -> impl IntoView {
//     let (counter, set_counter) = create_signal(cx, 0);
//     view! {
//         cx,
//             <textarea name="input" id="input" cols="30" rows="10">{move || data.read(cx)}</textarea>
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust-recipe-gallery-frontend.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="flex flex-col min-h-screen bg-green-50">
        <TopNavBar/>
        <main class="flex flex-auto">
            <Router>
                <Routes>
                    <Route path="/" view=  move || view! {  <Home/> }/>
                    // can use nesting, but nah too lazy
                    <Route path="/recipes" view=  move || view! {  <AllRecipes/> }/>
                    <Route path="/recipes/add" view=  move || view! {  <AddRecipe/> }/>
                    <Route path="/recipes/:id" view=  move || view! {  <ShowRecipe/> }/>
                    <Route path="/recipes/:id/edit" view=  move || view! {  <EditRecipe/> }/>
                </Routes>
            </Router>
        </main>
        <Footer/>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Rust Recipe Gallery"/>
        <div class="bg-gradient-to-tl from-lime-300 to-lime-100 text-black font-mono flex flex-auto items-center justify-center">
            <h1 class="m-auto text-center">"Cook!"</h1>
        </div>
    }
}

#[component]
pub fn AllRecipes() -> impl IntoView {
    let async_get_recipes = create_resource(
        || (),
        |_| async move {
            logging::log!("Getting recipes...");
            get_all_recipes().await
        },
    );

    view! {
        <Title text="Rust Recipe Gallery - Recipe Gallery"/>
        <div class="w-full max-w-xl text-black mx-auto py-8">
            <Suspense
                fallback=move || view! {  <h1 class="text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1> }
            >
                <ul class="flex flex-auto flex-col">
                    {move || async_get_recipes.get().map(|inside| {
                        match inside {
                            Err(e) => view! (  <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(),
                            Ok(recipe_list) => recipe_list.iter().map(|recipe| {
                                view! (
                                    <li class="">
                                        // <a class="font-medium text-green-600 hover:underline" href=format!("/recipes/{}", recipe.id.to_string())>
                                        <a class="m-4 p-6 block font-medium border-gray-600 rounded-lg hover:bg-green-200 bg-green-400" href=format!("/recipes/{}", recipe.id.to_string())>
                                            {recipe.title.clone()}
                                        </a>
                                    </li>
                                )
                            }).collect_view()
                        }
                    })}
                </ul>
            </Suspense>
        </div>
    }
}

#[component]
pub fn AddRecipe() -> impl IntoView {
    let (post_response, set_response) = create_signal(Ok(None::<Recipe>));
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let post_recipe_action = create_action(
        move |(title, ingredients, steps): &(String, String, String)| {
            let title = title.to_string();
            let ingredients = ingredients.to_string();
            let steps = steps.to_string();
            logging::log!("Title: {title}, Ingredients: {ingredients}, Stepes: {steps}");
            async move {
                set_wait_for_response.update(|w| *w = true);
                logging::log!("sending post request");
                let response = post_recipe(title.clone(), ingredients.clone(), steps.clone()).await;
                logging::log!("{response:#?}");
                set_response.update(|w| *w = response.map(|inside| Some(inside)));
                logging::log!("finished sending post request");
                set_wait_for_response.update(|w| *w = false);
            }
        },
    );
    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <Title text="Rust Recipe Gallery - Add Recipe"/>
        <AddRecipeForm
            action=post_recipe_action
            response=post_response
            disabled
        />
    }
}

#[derive(Params, PartialEq)]
struct EditRecipeParams {
    id: Option<String>, // normally can just do String in nightly. but nightly doesn't work cuz 1.78 has breaking changes
}

#[component]
pub fn EditRecipe() -> impl IntoView {
    let params = use_params::<EditRecipeParams>();
    let id = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params_reference| params_reference.id.clone())
                .unwrap_or_default()
        })
    };

    let async_get_recipe = create_resource(
        move || {
            use_params::<EditRecipeParams>().with(|params| {
                params
                    .as_ref()
                    .map(|params_reference| params_reference.id.clone())
                    .unwrap_or_default()
            })
        },
        move |id| async move {
            let result = get_recipe_by_id(id.unwrap_or_default()).await;
            logging::log!("{result:#?}");
            result
        },
    );

    let (patch_response, set_response) = create_signal(Ok(None::<Recipe>));
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let disabled = Signal::derive(move || wait_for_response.get());
    let patch_recipe_action = create_action(
        move |(title, ingredients, steps): &(String, String, String)| {
            let title = title.to_string();
            let ingredients = ingredients.to_string();
            let steps = steps.to_string();
            let id_string = id().unwrap_or_default();

            async move {
                set_wait_for_response.update(|w| *w = true);
                logging::log!("sending post request");
                let response = patch_recipe_by_id(id_string, title, ingredients, steps).await;
                logging::log!("{response:#?}");
                set_response.update(|w| *w = response.map(|inside| Some(inside)));
                logging::log!("finished sending post request");
                set_wait_for_response.update(|w| *w = false);
            }
        },
    );

    let get_title = Signal::derive(move || {
        match async_get_recipe.get() {
            None => String::new(),
            Some(inside_some) => match inside_some {
                Err(_) => String::new(),
                Ok(recipe) => recipe.title.clone(),
            },
        }
        // .map(|inside_option| match inside_option {
        //     Err(_) => String::new(),
        //     Ok(recipe) => recipe.title.clone(),
        // })
    });
    let get_ingredients = Signal::derive(move || match async_get_recipe.get() {
        None => String::new(),
        Some(inside_some) => match inside_some {
            Err(_) => String::new(),
            Ok(recipe) => recipe.ingredients.clone(),
        },
    });
    let get_steps = Signal::derive(move || match async_get_recipe.get() {
        None => String::new(),
        Some(inside_some) => match inside_some {
            Err(_) => String::new(),
            Ok(recipe) => recipe.body.clone(),
        },
    });

    view! {
        <Title text="Rust Recipe Gallery - Edit Recipe"/>
        <Suspense fallback=move || view! ( <h1 class="mt-5 text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1>)>
            <AddRecipeForm
                action=patch_recipe_action
                response=patch_response
                disabled
                title_fallback=get_title
                ingredients_fallback=get_ingredients
                steps_fallback=get_steps
                action_name="Edit"
            />
        </Suspense>
    }
}

#[component]
pub fn ShowRecipe() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let generate_title = move || format!("Rust Recipe Gallery - Recipe {}", id());
    let generate_edit_href = move || format!("/recipes/{}/edit", id());

    let async_get_recipe = create_resource(
        move || {
            use_params::<EditRecipeParams>().with(|params| {
                params
                    .as_ref()
                    .map(|params_reference| params_reference.id.clone())
                    .unwrap_or_default()
            })
        },
        |id| async move { get_recipe_by_id(id.unwrap_or_default()).await },
    );

    let async_get_comments_by_recipe_id = create_resource(
        move || {
            use_params::<EditRecipeParams>().with(|params| {
                params
                    .as_ref()
                    .map(|params_reference| params_reference.id.clone())
                    .unwrap_or_default()
            })
        },
        |id| async move { get_comments_by_recipe_id(id.unwrap_or_default()).await },
    );

    // let delete_comment_action =
    //     create_action( move |(recipe_id, comment_id): &(String, String)| {
    //         let recipe_id = recipe_id.to_string();
    //         let comment_id = comment_id.to_string();
    //         async move {
    //             // TODO: wait for response
    //             let response = delete_comment_by_id(recipe_id, comment_id).await;
    //             let navigate = leptos_router::use_navigate();
    //             logging::log!("{response:#?}");
    //             navigate("/", Default::default());
    //         }
    //     });

    let delete_recipe_action = create_action(move |recipe_id: &String| {
        let recipe_id = recipe_id.to_string();
        async move {
            // TODO: wait for response, or not. idc
            let response = delete_recipe_by_id(recipe_id).await;
            let navigate = leptos_router::use_navigate();
            logging::log!("{response:#?}");
            match response {
                Err(_) => {}
                Ok(_) => navigate("/recipes", Default::default()),
            }
        }
    });

    // let delete_recipe_action_dispatch = move || delete_recipe_action.dispatch(id());

    view! {
        <Title text=generate_title/>
        <div class="max-w-2xl rounded-xl w-full mx-auto py-8">
            <Suspense fallback=move || view! ( <h1 class="mt-5 text-center p-6 bg-green-400 rounded-lg">"Loading..."</h1>)>
                {move || async_get_recipe.get().map(|inside_some| {
                    match inside_some {
                        Err(e) => view! (  <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(),
                        Ok(recipe) =>
                            view! (
                                <div><strong>"Title: "</strong>{recipe.title}</div>
                                <div><strong>"Ingredients: "</strong>{recipe.ingredients}</div>
                                <div><strong>"Steps: "</strong>{recipe.body}</div>
                                <button class="mt-6 mr-5 bg-green-300 hover:bg-green-200 p-2 rounded-md" on:click= move |_| delete_recipe_action.dispatch(id())>
                                    "Delete"
                                </button>
                                <A class="bg-green-300 hover:bg-green-200 p-2 rounded-md" href=generate_edit_href>"Edit"</A>
                            ).into_view()
                    }
                })}
            </Suspense>
            <Suspense fallback=move || view! ( <h1 class="text-center p-6 bg-green-300 rounded-lg">"Loading Comments..."</h1>)>
                {move || async_get_comments_by_recipe_id.get().map(|inside_some| {
                    match inside_some {
                        Err(e) => view! (  <h1 class="text-center bg-red-200 p-6 rounded-lg">"Error: " {e.to_string()}</h1>).into_view(),
                        Ok(comments) =>
                            comments.iter().map(|comment| {
                                view! (
                                    <li>
                                        <div>{comment.comment.clone()}</div> // idk why i have to clone
                                        <button>
                                            "Delete?"
                                        </button>
                                        <button>"Edit?"</button>
                                    </li>
                                ).into_view()
                            }).collect_view()
                    }
                })}
            </Suspense>
        </div>
    }
}
