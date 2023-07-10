use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        // <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
        <div class="bg-gradient-to-tl from-lime-300 to-lime-100 text-black font-mono flex flex-auto items-center justify-center">
            <h1 class="m-auto text-center">"Cook!"</h1>
        </div>
    }
}

#[component]
pub fn TopNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        // <nav class="bg-green-600 flex flex-col h-6 p-6 py-8 text-center text-xl text-white font-medium">
        <nav class="bg-green-600 text-white">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-6">
                <div><a class="font-medium" href="/">"Rust Recipe Gallery"</a></div>
                <div class="w-auto">
                    <ul class="flex">
                        <li><a class="pr-4 hover:text-green-200" href="/recipes">"Gallery"</a></li>
                        <li><a class="pr-4 hover:text-green-200" href="/recipes/add">"Add Recipe"</a></li>
                        // <li><a class="pr-4 hover:text-blue-700" href="">"Login?????"</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn AddRecipe(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let title = move || query().get("title").cloned().unwrap_or_default();
    let ingredients = move || query().get("ingredients").cloned().unwrap_or_default();
    let steps = move || query().get("steps").cloned().unwrap_or_default();

    view! { cx,
        <Title text="Rust Recipe Gallery - Add Recipe"/>

        // <nav class="bg-green-600 flex flex-col h-6 p-6 py-8 text-center text-xl text-white font-medium">
        <div class="w-full max-w-lg text-black mx-auto py-8">
            <Form class="bg-white shadow-md rounded px-8 pt-6 pb-5 mb-2" method="GET" action="">
                <div class="w-full text-black text-2xl pb-4 text-center">
                    <h1>Create new recipe</h1>
                </div>
                <div class="mb-5">
                    <label for="title" class="block text-gray-700 text-lg font-bold mb-1" value=title>Title</label>
                    <input type="text" id="title" placeholder="Title"
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
                            focus:border-green-500"
                    />
                </div>
                <div class="mb-5">
                    <label for="ingredients" class="block text-gray-700 text-lg font-bold mb-1">Ingredients</label>
                    // <input type="text" id="ingredients"/>
                    <textarea id="ingredients" rows="4" cols="50"
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
                            focus:border-green-500"
                        placeholder="Write your ingredients here..."
                        value=ingredients
                    />
                </div>
                <div class="mb-5">
                    <label for="steps" class="block text-gray-700 text-lg font-bold mb-1">Steps</label>
                    // <input type="text"/>
                    <textarea
                        id="steps"
                        rows="4"
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
                            focus:border-green-500"
                        placeholder="Write your steps here..."
                        value=steps
                    />
                </div>
                <div class="text-right">
                    <input class="bg-green-500
                        hover:bg-green-700
                        text-white
                        border-gray-300
                        font-bold
                        py-2
                        px-4
                        rounded-lg
                        focus:outline-none
                        focus:shadow-outline"
                        type="submit"
                    >
                        "Create Recipe"
                    </input>
                    //  <input type="submit"/>
                </div>
            </Form>
            <h1>{title}</h1>
        </div>
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
