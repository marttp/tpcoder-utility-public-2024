use leptos::*;

#[component]
pub fn Home() -> impl IntoView {

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to the TP Coder - Utilities Tools Site"</h2>
            <p class="px-10 pb-10 text-left">
                r#"
                This site provides a collection of utility tools that can be used for various purposes.
                These tools are designed to help developers and users perform common tasks more efficiently.
                Honestly speaking, I have searched for these tools many times and I thought it would be a good idea to have them all in one place.
                If I struggle to find these, I am sure others will too. 😆
                "#
            </p>
            <h3 class="text-2xl">"Available Tools"</h3>
            <ul class="list-disc list-inside text-left px-10">
                <li><a href="/uuid" class="text-blue-500 hover:underline">"UUID Generator"</a></li>
                <li><a href="/ulid" class="text-blue-500 hover:underline">"ULID Generator"</a></li>
                <li><a href="/qrcode" class="text-blue-500 hover:underline">"QR Code Generator"</a></li>
                <li><a href="/haversine" class="text-blue-500 hover:underline">"Haversine Distance Calculator"</a></li>
                <li><a href="/back_of_the_envelope" class="text-blue-500 hover:underline">"System Design Estimator"</a></li>
            </ul>
        </div>
    }
}