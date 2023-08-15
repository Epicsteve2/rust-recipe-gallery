use leptos::*;

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
