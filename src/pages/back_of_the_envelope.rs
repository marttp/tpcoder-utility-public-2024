use leptos::*;

const SECOND_IN_MINUTE: u64 = 60;
const MINUTE_IN_HOUR: u64 = 60;
const HOUR_IN_DAY: u64 = 24;
const SECOND_IN_DAY: u64 = SECOND_IN_MINUTE * MINUTE_IN_HOUR * HOUR_IN_DAY; // 86400
const DAY_IN_YEAR: u64 = 365; // Assume 365 days in a year

#[component]
pub fn SystemDesignPage() -> impl IntoView {
    // Inputs
    let (daily_active_user, set_daily_active_user) = create_signal(0_u64);
    let (read_write_ratio, set_read_write_ratio) = create_signal(String::new());
    let (data_size, set_data_size) = create_signal(0_u64);
    // Results
    let (read_per_second, set_read_per_second) = create_signal(0_f64);
    let (write_per_second, set_write_per_second) = create_signal(0_f64);
    let (storage_used_per_year, set_storage_used_per_year) = create_signal(0_u64);

    // Calculation
    let calculate = move |_| {
        let dau = daily_active_user.get();
        let binding = read_write_ratio.get();
        let size = data_size.get() as f64;
        let (read_per_second_result, write_per_second_result, storage_used_per_year_result) =
            back_of_the_envelope(dau, binding, size);
        set_read_per_second.set(read_per_second_result);
        set_write_per_second.set(write_per_second_result);
        set_storage_used_per_year.set(storage_used_per_year_result as u64);
    };

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">
                "Back of the envelope calculations"
            </h2>
            <ul class="text-left">
                <li>"Assume DAU (Daily Active User)"</li>
                <li>"Adjust read:write ratio - one of them need to be 1 for based calculation"</li>
                <li>"Adjust number you want to calculate read/write per seconds"</li>
                <li>"Assume data size of interest payload"</li>
            </ul>
            <hr class="my-8" />
            <div class="flex flex-col md:flex-row">
                <div class="md:w-1/2 p-4">
                    <label class="text-left">"Daily Active User"</label>
                    <input
                        class="border p-2 w-full"
                        placeholder="Daily Active User"
                        on:input=move |e| set_daily_active_user.set(event_target_value(&e).parse::<u64>().unwrap_or(0))
                    />
                    <label class="text-left">"Read:Write Ratio"</label>
                    <input
                        class="border p-2 w-full"
                        placeholder="Read:Write Ratio"
                        on:input=move |e| set_read_write_ratio.set(event_target_value(&e))
                    />
                    <label class="text-left">"Data size of interest payload (Byte)"</label>
                    <input
                        class="border p-2 w-full"
                        placeholder="Data size of interest payload in byte"
                        on:input=move |e| set_data_size.set(event_target_value(&e).parse::<u64>().unwrap_or(0))
                    />
                    <button
                        class="bg-satin-linen text-black py-2 px-4 rounded hover:scale-110 mt-4"
                        on:click=calculate
                    >
                        "Calculate"
                    </button>
                </div>
                <div class="md:w-1/2 p-4 flex items-center justify-center">
                    <div class="text-left">
                        <p>"Read per second"</p>
                        <p>{read_per_second} " rps"</p>
                        <hr class="my-4"/>
                        <p>"Write per second"</p>
                        <p>{write_per_second} " tps"</p>
                        <hr class="my-4" />
                        <p class="mb-4">"Storage used per year in roughly calculated from Write per second (Byte, KB, MB, TB, PB)"</p>
                        <p>{storage_used_per_year} " Byte"</p>
                        { move || {
                            let kb = storage_used_per_year.get() / 1024;
                            let mb = kb / 1024;
                            let gb = mb / 1024;
                            let tb = gb / 1024;
                            let pb = tb / 1024;
                            view! {
                                <div>
                                    <p>{kb} " KB"</p>
                                    <p>{mb} " MB"</p>
                                    <p>{gb} " GB"</p>
                                    <p>{tb} " TB"</p>
                                    <p>{pb} " PB"</p>
                                </div>
                            }
                        }}
                        <hr class="my-4" />
                        <p class="text-lg">"The rest, Sum/Multiply them by yourself, you already got foundation value"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn back_of_the_envelope(dau: u64, ratio_input: String, size: f64) -> (f64, f64, f64) {
    let ratio = ratio_input
        .split(":")
        .collect::<Vec<_>>();
    if ratio.len() != 2 {
        panic!("Invalid ratio format");
    }
    let read_ratio = ratio[0].parse::<f64>().unwrap_or(0.0);
    let write_ratio = ratio[1].parse::<f64>().unwrap_or(0.0);
    let read_per_second_result = (dau as f64 * read_ratio) / SECOND_IN_DAY as f64;
    let write_per_second_result = (dau as f64 * write_ratio) / SECOND_IN_DAY as f64;
    let storage_used_per_year_result = (size * write_per_second_result) * DAY_IN_YEAR as f64;
    (read_per_second_result, write_per_second_result, storage_used_per_year_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_of_the_envelope_with_valid_inputs() {
        let dau = 1000_u64;
        let ratio = "1:1".to_string();
        let size = 1_f64;
        let (read_per_second, write_per_second, storage_used_per_year) = back_of_the_envelope(dau, ratio, size);
        assert_eq!(read_per_second, 0.011574074074074073);
        assert_eq!(write_per_second, 0.011574074074074073);
        assert_eq!(storage_used_per_year as u64 , 4);
    }

    #[test]
    fn back_of_the_envelope_with_zero_dau() {
        let dau = 0_u64;
        let ratio = "1:1".to_string();
        let size = 1024_f64;
        let (read_per_second, write_per_second, storage_used_per_year) = back_of_the_envelope(dau, ratio, size);
        assert_eq!(read_per_second, 0.0);
        assert_eq!(write_per_second, 0.0);
        assert_eq!(storage_used_per_year, 0.0);
    }

    #[test]
    #[should_panic(expected = "Invalid ratio format")]
    fn back_of_the_envelope_with_invalid_ratio() {
        let dau = 1000_u64;
        let ratio = "invalid".to_string();
        let size = 1_f64;
        back_of_the_envelope(dau, ratio, size);
    }

    #[test]
    fn back_of_the_envelope_with_large_data_size() {
        let dau = 1000_u64;
        let ratio = "1:1".to_string();
        let size = 1_000_000_f64;
        let (read_per_second, write_per_second, storage_used_per_year) = back_of_the_envelope(dau, ratio, size);
        assert_eq!(read_per_second, 0.011574074074074073);
        assert_eq!(write_per_second, 0.011574074074074073);
        assert_eq!(storage_used_per_year as u64, 4224537);
    }

    #[test]
    fn back_of_the_envelope_with_high_read_ratio() {
        let dau = 1000_u64;
        let ratio = "10:1".to_string();
        let size = 1_f64;
        let (read_per_second, write_per_second, storage_used_per_year) = back_of_the_envelope(dau, ratio, size);
        assert_eq!(read_per_second, 0.11574074074074074);
        assert_eq!(write_per_second, 0.011574074074074073);
        assert_eq!(storage_used_per_year as u64, 4);
    }
}