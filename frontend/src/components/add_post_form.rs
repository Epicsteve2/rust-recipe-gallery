use leptos::*;

use crate::models::{AppError, Recipe};

#[component]
pub fn AddRecipeForm(
    cx: Scope,
    action: Action<(String, String, String), ()>,
    response: ReadSignal<Result<Option<Recipe>, AppError>>,
    disabled: Signal<bool>,
    #[prop(optional)] title_fallback: String,
    #[prop(optional)] ingredients_fallback: String,
    #[prop(optional)] steps_fallback: String,
) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());
    let (ingredients, set_ingredients) = create_signal(cx, String::new());
    let (body, set_body) = create_signal(cx, String::new());

    let dispatch_action = move || action.dispatch((title.get(), ingredients.get(), body.get()));

    let button_is_disabled = Signal::derive(cx, move || {
        disabled.get()
            || title.get().is_empty()
            || ingredients.get().is_empty()
            || body.get().is_empty()
    });

    view! { cx,
        <div class="w-full max-w-lg text-black mx-auto py-8">
            <form class="bg-white shadow-md rounded px-8 pt-6 pb-5 mb-2" on:submit=|ev| ev.prevent_default()>
                <div class="w-full text-black text-2xl pb-4 text-center">
                    <h1>"Create new recipe"</h1>
                </div>
                <div class="mb-5">
                    <label for="title" class="block text-gray-700 text-lg font-bold mb-1">"Title"</label>
                    <input type="text" id="title" placeholder="Title"
                        value=title_fallback
                        required
                        class="shadow
                            rounded-lg
                            w-full
                            py-2
                            px-3
                            bg-gray-50
                            text-gray-700
                            border
                            leading-tight
                            border-gray-300
                            focus:ring-green-500
                            focus:border-green-500
                            disabled:bg-slate-300
                            "
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_title.update(|v| *v = val);
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_title.update(|v| *v = val);
                            }
                    />
                </div>
                <div class="mb-5">
                    <label for="ingredients" class="block text-gray-700 text-lg font-bold mb-1">Ingredients</label>
                    <textarea id="ingredients" rows="4" cols="50"
                        required
                        value=ingredients_fallback
                        class="block
                            p-2.5
                            w-full
                            bg-gray-50
                            text-gray-700
                            rounded-lg
                            leading-tight
                            border
                            border-gray-300
                            focus:ring-green-500
                            focus:border-green-500
                            disabled:bg-slate-300"
                        placeholder="Write your ingredients here..."
                        prop:disabled=move || disabled.get()
                        on:keyup=move |ev: ev::KeyboardEvent| {
                            let val = event_target_value(&ev);
                            set_ingredients.update(|v| *v = val);
                        }
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_ingredients.update(|v| *v = val);
                        }
                    />
                </div>
                <div class="mb-5">
                    <label for="steps" class="block text-gray-700 text-lg font-bold mb-1">Steps</label>
                    <textarea
                        id="steps"
                        rows="4"
                        required
                        value=steps_fallback
                        cols="50"
                        class="block
                            p-2.5
                            w-full
                            bg-gray-50
                            text-gray-700
                            rounded-lg
                            leading-tight
                            border
                            border-gray-300
                            focus:ring-green-500
                            focus:border-green-500
                            disabled:bg-slate-300"
                        placeholder="Write your steps here..."
                        prop:disabled=move || disabled.get()
                        on:keyup=move |ev: ev::KeyboardEvent| {
                            let val = event_target_value(&ev);
                            set_body.update(|v| *v = val);
                        }
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_body.update(|v| *v = val);
                        }
                    />
                </div>
                <div class="text-right">
                    <button class="bg-green-500
                        hover:bg-green-700
                        text-white
                        border-gray-300
                        disabled:bg-slate-500
                        font-bold
                        py-2
                        px-4
                        rounded-lg
                        focus:outline-none
                        focus:shadow-outline"
                        prop:disabled=move || button_is_disabled.get()
                        on:click=move |_| {dispatch_action();}
                    >
                        "Create Recipe"
                    </button>
                </div>
                <div class="pt-2">
                    // I am doing this very incorrectly
                    {move || if response.with(|n| n.as_ref().is_ok()) {
                        if response.with(|n| n.as_ref().unwrap().is_some()) {
                            view! { cx,
                                <p class="text-green-500">
                                    <strong>"Success! Recipe ID: "</strong>
                                    {response.with(|n| n.as_ref().unwrap().as_ref().unwrap().id.to_string())}
                                </p>
                            }.into_view(cx)
                        } else {
                            view! { cx,
                                <></>
                            }.into_view(cx)
                        }
                    } else {
                        view! { cx,
                            <p class="text-red-500">
                                    <strong>"Error: "</strong>
                                    {response.with(|n| n.as_ref().unwrap_err().to_string())}
                            </p>
                        }.into_view(cx)
                    }}
                </div>
            </form>
        </div>
    }
}
