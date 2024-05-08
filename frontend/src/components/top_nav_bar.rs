use leptos::*;

#[component]
pub fn TopNavBar() -> impl IntoView {
    view! {
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
