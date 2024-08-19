use leptos::*;

#[component]
pub fn ULIDPage() -> impl IntoView {
    let (ulids, set_ulids) = create_signal(Vec::<String>::new());

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">
                "ULID Generator"
            </h2>
            <div class="flex-col justify-center p-5">
                <button
                    class="bg-satin-linen text-black py-2 px-4 rounded hover:scale-110"
                    on:click=move |_| set_ulids.update(|vec| {
                        vec.clear();
                        vec.extend(generate_ulid());
                    })>
                    "Generate ULID - 10 ids"
                </button>
                <ul class="py-2">
                    { move || ulids.get().into_iter()
                            .map(|id| view! { <li class="p-2">{id}</li> })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        </div>
    }
}

fn generate_ulid() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for _ in 0..10 {
        result.push(ulid::Ulid::new().to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_ulid_returns_10_ulids() {
        let ulids = generate_ulid();
        assert_eq!(ulids.len(), 10);
        for ulid in ulids {
            assert!(ulid::Ulid::from_string(&ulid).is_ok());
        }
    }

    #[test]
    fn ulid_list_is_cleared_before_generating_new_ulids() {
        let (ulids, set_ulids) = create_signal(vec!["existing_ulid".to_string()]);
        set_ulids.update(|vec| {
            vec.clear();
            vec.extend(generate_ulid());
        });
        assert_eq!(ulids.get().len(), 10);
        assert!(!ulids.get().contains(&"existing_ulid".to_string()));
    }
}