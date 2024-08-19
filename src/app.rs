use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::{
    home::Home,
    uuid::UUIDPage,
    ulid::ULIDPage,
    qrcode::QRCodePage,
    haversine::HaversinePage,
    back_of_the_envelope::SystemDesignPage,
    not_found::NotFound,
};
use crate::components::navbar::Navbar;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Navbar />
        // Note: In order to navigate, All route should be under this router
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
                <Route path="uuid" view=move || view! { <UUIDPage/> }/>
                <Route path="ulid" view=move || view! { <ULIDPage/> }/>
                <Route path="qrcode" view=move || view! { <QRCodePage/> }/>
                <Route path="haversine" view=move || view! { <HaversinePage/> }/>
                <Route path="back_of_the_envelope" view=move || view! { <SystemDesignPage/> }/>
                <Route path="/*any" view=|| view! { <NotFound /> }/>
            </Routes>
        </Router>
    }
}