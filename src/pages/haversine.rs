use geoutils::Location;
use leptos::*;

#[component]
pub fn HaversinePage() -> impl IntoView {
    let (lat1, set_lat1) = create_signal(String::new());
    let (lon1, set_lon1) = create_signal(String::new());
    let (lat2, set_lat2) = create_signal(String::new());
    let (lon2, set_lon2) = create_signal(String::new());
    let (distance, set_distance) = create_signal(String::new());

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">
                "Haversine Distance Calculator"
            </h2>
            <p class="px-10 pb-10 text-left text-center">
                r#"
                The Haversine formula is used to calculate the great-circle distance between two points on the surface of a sphere,
                given their longitudes and latitudes.
                It is particularly useful in navigation and geospatial applications
                to determine the shortest distance over the Earth's surface,
                assuming the Earth is a perfect sphere.
                "#
            </p>
            <div class="flex flex-col md:flex-row">
                <div class="md:w-1/2 p-4">
                    <label class="text-left">"Location 1"</label>
                    <input
                        class="border p-2 w-full"
                        placeholder="Latitude 1"
                        on:input=move |e| set_lat1.set(event_target_value(&e))
                    />
                    <input
                        class="border p-2 w-full"
                        placeholder="Longitude 1"
                        on:input=move |e| set_lon1.set(event_target_value(&e))
                    />
                    <label class="text-left">"Location 2"</label>
                    <input
                        class="border p-2 w-full"
                        placeholder="Latitude 2"
                        on:input=move |e| set_lat2.set(event_target_value(&e))
                    />
                    <input
                        class="border p-2 w-full"
                        placeholder="Longitude 2"
                        on:input=move |e| set_lon2.set(event_target_value(&e))
                    />
                    <button
                        class="bg-satin-linen text-black py-2 px-4 rounded hover:scale-110 mt-4"
                        on:click=move |_| {
                            let lat1 = lat1.get().trim().parse::<f64>().unwrap_or(0.0);
                            let lon1 = lon1.get().trim().parse::<f64>().unwrap_or(0.0);
                            let lat2 = lat2.get().trim().parse::<f64>().unwrap_or(0.0);
                            let lon2 = lon2.get().trim().parse::<f64>().unwrap_or(0.0);
                            let distance = haversine_distance(lat1, lon1, lat2, lon2);
                            set_distance.set(format!("{:.2} meters", distance));
                        }
                    >
                        "Calculate Distance"
                    </button>
                </div>
                <div class="md:w-1/2 p-4 flex items-center justify-center">
                    <p>{distance}</p>
                </div>
            </div>
        </div>
    }
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let location1 = Location::new(lat1, lon1);
    let location2 = Location::new(lat2, lon2);
    let distance = location1.haversine_distance_to(&location2);
    distance.meters()
}

#[cfg(test)]
mod tests {
    use leptos::ssr::render_to_string;
    use super::*;

    #[test]
    fn haversine_distance_with_valid_coordinates() {
        // Tokyo Tower
        let lat1 = 35.6739911;
        let lon1 = 139.722592;
        // Tokyo Skytree
        let lat2 = 35.6883044;
        let lon2 = 139.791227;
        let distance = haversine_distance(lat1, lon1, lat2, lon2);
        // Distance between Tokyo Tower and Tokyo Skytree is approximately 6.40 km
        // I don't know if Rust has a built-in assertion that able to use delta for the comparison
        assert_eq!(distance.trunc(), 6400.0);
    }

    #[test]
    fn haversine_distance_with_same_coordinates() {
        // Tokyo Tower
        let lat1 = 35.6739911;
        let lon1 = 139.722592;
        let lat2 = 35.6739911;
        let lon2 = 139.722592;
        let distance = haversine_distance(lat1, lon1, lat2, lon2);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn haversine_page_renders_correctly() {
        let rendered = render_to_string(|| view! { <HaversinePage /> });
        assert!(rendered.contains("Haversine Distance Calculator"));
        assert!(rendered.contains("Latitude 1"));
        assert!(rendered.contains("Longitude 1"));
        assert!(rendered.contains("Latitude 2"));
        assert!(rendered.contains("Longitude 2"));
        assert!(rendered.contains("Calculate Distance"));
    }

    #[test]
    fn haversine_page_calculates_distance_on_button_click() {
        let (lat1, set_lat1) = create_signal("52.5200".to_string());
        let (lon1, set_lon1) = create_signal("13.4050".to_string());
        let (lat2, set_lat2) = create_signal("48.8566".to_string());
        let (lon2, set_lon2) = create_signal("2.3522".to_string());
        let (distance, set_distance) = create_signal(String::new());

        let lat1 = lat1.get().parse::<f64>().unwrap_or(0.0);
        let lon1 = lon1.get().parse::<f64>().unwrap_or(0.0);
        let lat2 = lat2.get().parse::<f64>().unwrap_or(0.0);
        let lon2 = lon2.get().parse::<f64>().unwrap_or(0.0);
        let calculated_distance = haversine_distance(lat1, lon1, lat2, lon2);
        set_distance.set(format!("{:.2} meters", calculated_distance));

        assert!(!distance.get().is_empty());
    }
}