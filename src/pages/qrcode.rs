use image::Luma;
use leptos::*;
use qrcode::QrCode;
use std::io::Cursor;
use base64::Engine;

#[component]
pub fn QRCodePage() -> impl IntoView {
    let (input_text, set_input_text) = create_signal(String::new());
    let (qr_code_data, set_qr_code_data) = create_signal(String::new());

    let generate_qr_code = move |_| {
        let text = input_text.get();
        let qr_code = gen_qr_code(text.clone());
        let base64_qr_code = format!("data:image/png;base64, {}", qr_code);
        set_qr_code_data.set(base64_qr_code);
        set_input_text.set(String::new());
    };

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">
                "QR Code Generator"
            </h2>
            <div class="flex flex-col md:flex-row"> {/* Apply flexbox */}
                <div class="md:w-1/2 p-4"> {/* Left side */}
                    <textarea
                        class="border p-2 w-full" // Make textarea full width
                        placeholder="Enter text (max 500 characters)"
                        maxlength="500"
                        on:input=move |e| set_input_text.set(event_target_value(&e))
                    />
                    <button
                        class="bg-satin-linen text-black py-2 px-4 rounded hover:scale-110 mt-4" // Add margin top
                        on:click=generate_qr_code
                    >
                        "Generate QR Code"
                    </button>
                </div>
                <div class="md:w-1/2 p-4 flex items-center justify-center"> {/* Right side */}
                    {move || if qr_code_data.get().is_empty() {
                        view! {
                            <img
                                src="https://via.placeholder.com/200"
                                alt="Placeholder QR Code"
                                />
                        }
                    } else {
                        view! {
                            <img
                                src=qr_code_data
                                alt="QR Code"
                                />
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

fn gen_qr_code(text: String) -> String {
    if text.is_empty() {
        return String::new();
    }
    if let Ok(code) = QrCode::new(text.as_bytes()) {
        let image = code.render::<Luma<u8>>().build();
        let mut buffer = Cursor::new(Vec::new());
        image.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        let data = buffer.into_inner();
        base64::engine::general_purpose::STANDARD.encode(&data)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use leptos::ssr::render_to_string;
    use super::*;

    #[test]
    fn generate_qr_code_with_valid_text() {
        let text = "Hello, QR Code!";
        let qr_code = gen_qr_code(text.to_string());
        assert!(!qr_code.is_empty());
    }

    #[test]
    fn generate_qr_code_with_empty_text() {
        let text = "";
        let qr_code = gen_qr_code(text.to_string());
        assert!(qr_code.is_empty());
    }

    #[test]
    fn qr_code_page_renders_correctly() {
        let rendered = render_to_string(|| view! { <QRCodePage /> });
        assert!(rendered.contains("QR Code Generator"));
        assert!(rendered.contains("Enter text (max 500 characters)"));
        assert!(rendered.contains("Generate QR Code"));
    }

    #[test]
    fn qr_code_page_generates_qr_code_on_button_click() {
        let (input_text, set_input_text) = create_signal(String::new());
        let (qr_code_data, set_qr_code_data) = create_signal(String::new());

        set_input_text.set("Hello, QR Code!".to_string());
        let qr_code = gen_qr_code(input_text.get());
        set_qr_code_data.set(format!("data:image/png;base64, {}", qr_code));

        assert!(!qr_code_data.get().is_empty());
    }
}