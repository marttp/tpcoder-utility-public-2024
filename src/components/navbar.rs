use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-siam-sage p-4 flex justify-center">
            <ul class="flex flex-col md:flex-row space-y-2 md:space-y-0 md:space-x-4">
                <li class="text-black hover:scale-110"><a href="/">"Home"</a></li>
                <li class="text-black hover:scale-110"><a href="/uuid">"UUID"</a></li>
                <li class="text-black hover:scale-110"><a href="/ulid">"ULID"</a></li>
                <li class="text-black hover:scale-110"><a href="/qrcode">"QR Code"</a></li>
                <li class="text-black hover:scale-110"><a href="/haversine">"Haversine"</a></li>
                <li class="text-black hover:scale-110"><a href="/back_of_the_envelope">"System Design Estimator"</a></li>
            </ul>
        </nav>
    }
}