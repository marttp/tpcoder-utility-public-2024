use leptos::*;
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex-col my-0 mx-auto max-w-3xl">
            <h2 class="p-6 text-4xl text-center">
                "404 Not Found"
            </h2>
            <p class="px-10 pb-10 text-left text-center">
                "The page you are looking for does not exist."
            </p>
        </div>
    }
}