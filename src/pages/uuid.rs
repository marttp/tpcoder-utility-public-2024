use leptos::*;

#[component]
pub fn UUIDPage() -> impl IntoView {
    let (uuidv4, set_uuidv4) = create_signal(Vec::<String>::new());

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">
                "UUID Generator"
            </h2>
            <div class="flex-col justify-center p-5">
                <button
                    class="bg-satin-linen text-black py-2 px-4 rounded hover:scale-110"
                    on:click=move |_| set_uuidv4.update(|vec| {
                        vec.clear();
                        vec.extend(generate_uuid(4));
                    })>
                    "Generate UUID v4 - 10 ids"
                </button>
                <ul class="py-2">
                    { move || uuidv4.get().into_iter()
                            .map(|id| view! { <li class="p-2">{id}</li> })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        </div>
    }
}

fn generate_uuid(version: u8) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for _ in 0..10 {
        let id = if version == 4 {
            uuid::Uuid::new_v4().to_string()
        } else {
            panic!("Invalid available UUID option")
        };
        result.push(id);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_uuid_v4_returns_10_uuids() {
        let uuids = generate_uuid(4);
        assert_eq!(uuids.len(), 10);
        for uuid in uuids {
            assert!(uuid::Uuid::parse_str(&uuid).is_ok());
        }
    }

    #[test]
    #[should_panic(expected = "Invalid available UUID option")]
    fn generate_uuid_invalid_version_panics() {
        generate_uuid(5);
    }
}