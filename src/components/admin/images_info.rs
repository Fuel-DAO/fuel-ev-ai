use leptos::*;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileList};

// Define the data structure for ImagesInfoData
#[derive(Default, Debug, Clone)]
pub struct ImagesInfoData {
    pub images: Vec<String>,
    pub logo: String,
}

#[component]
pub fn ImagesInfo(
    data: RwSignal<ImagesInfoData>,
    absolute_logo_path: bool,
    upload_canister_id: String,
    asset_canister_id: String,
) -> impl IntoView {
    // Wrap IDs in Arc for cloning
    let upload_canister_id = Arc::new(upload_canister_id);
    let asset_canister_id = Arc::new(asset_canister_id);

    // Access the loading state from context
    let loading =
        use_context::<ReadSignal<bool>>().unwrap_or_else(|| create_rw_signal(false).read_only());
    // State variables
    let uploading = create_rw_signal(false);
    let uploading_progress = create_rw_signal(0);
    let error_asset = create_rw_signal(false);
    let error_logo = create_rw_signal(false);
    // Images Info Data Signal
    let images_info_data = create_rw_signal(ImagesInfoData {
    images: vec![
        "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
    ],
    logo: "https://plus.unsplash.com/premium_photo-1664303847960-586318f59035?q=80&w=2874&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D".to_string(),
});

    // Computed signal for disabled state
    let disabled = Arc::new(move || uploading.get() || loading.get())
        as Arc<dyn Fn() -> bool + Send + Sync + 'static>;

    // Event handler for file selection
    let on_select = {
        let uploading = uploading.clone();
        let uploading_progress = uploading_progress.clone();
        let data = data.clone();
        let error_asset = error_asset.clone();
        let error_logo = error_logo.clone();
        let upload_canister_id = Arc::clone(&upload_canister_id);
        Arc::new(move |event: Event, file_type: &'static str| {
            if file_type == "logo" {
                error_logo.set(false);
            } else {
                error_asset.set(false);
            }

            let input = event
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            let files = input.files();
            if files.is_none() {
                uploading.set(false);
                return;
            }
            let files = files.unwrap();
            if files.length() == 0 {
                uploading.set(false);
                return;
            }
            let file = files.get(0).unwrap();
            uploading.set(true);
            uploading_progress.set(0);
            upload_file(
                file,
                file_type.to_string(),
                uploading.clone(),
                uploading_progress.clone(),
                data.clone(),
                error_asset.clone(),
                error_logo.clone(),
                Arc::clone(&upload_canister_id),
            );
            // Clear the input value
            input.set_value("");
        }) as Arc<dyn Fn(Event, &'static str) + Send + Sync + 'static>
    };

    // Function to handle file upload
    fn upload_file(
        file: web_sys::File,
        file_type: String,
        uploading: RwSignal<bool>,
        uploading_progress: RwSignal<u32>,
        data: RwSignal<ImagesInfoData>,
        error_asset: RwSignal<bool>,
        error_logo: RwSignal<bool>,
        upload_canister_id: Arc<String>,
    ) {
        spawn_local(async move {
            // Implement your actual upload logic here.
            // For this example, we'll simulate an upload with a delay.
            use gloo_timers::future::TimeoutFuture;
            for i in 1..=100 {
                uploading_progress.set(i);
                TimeoutFuture::new(20).await;
            }

            // Simulate successful upload
            let res = format!("uploaded_file_url_{}", file.name());
            if file_type == "images" {
                data.update(|d| {
                    d.images.push(res.clone());
                });
            } else {
                data.update(|d| {
                    d.logo = res.clone();
                });
            }
            uploading.set(false);
        });
    }

    // Function to remove an image
    let remove_image = {
        let data = data.clone();
        let upload_canister_id = Arc::clone(&upload_canister_id);
        Arc::new(move |path: String, file_type: &'static str| {
            let data = data.clone();
            let upload_canister_id = Arc::clone(&upload_canister_id);
            spawn_local(async move {
                // Implement your actual delete logic here.
                // For this example, we'll simulate deletion with a delay.
                use gloo_timers::future::TimeoutFuture;
                TimeoutFuture::new(500).await;

                if file_type == "logo" {
                    data.update(|d| {
                        d.logo.clear();
                    });
                } else {
                    data.update(|d| {
                        d.images.retain(|img| img != &path);
                    });
                }
                // Simulate deletion from server
            });
        }) as Arc<dyn Fn(String, &'static str) + Send + Sync + 'static>
    };

    // Function to get asset path
    let asset_path = {
        let asset_canister_id = Arc::clone(&asset_canister_id);
        Arc::new(move |path: &str| {
            // Implement your logic to generate the asset path
            format!("{}/{}", asset_canister_id, path)
        }) as Arc<dyn Fn(&str) -> String + Send + Sync + 'static>
    };

    // Clone variables before the closure
    let data_clone = data.clone();
    let remove_image_clone = Arc::clone(&remove_image);
    let disabled_clone = Arc::clone(&disabled);
    let asset_path_clone = Arc::clone(&asset_path);
    let absolute_logo_path_clone = absolute_logo_path;

    view! {
        <div class="flex flex-col -mt-8 gap-4">
            <span class="text-sm font-medium leading-6 text-gray-900 mt-8">"Thumbnail/Logo:"</span>
            <div class="h-[14rem] w-[14rem] p-2 border rounded relative">
                {move || {
                    let data = data_clone.clone();
                    let remove_image = Arc::clone(&remove_image_clone);
                    let disabled = Arc::clone(&disabled_clone);
                    let asset_path = Arc::clone(&asset_path_clone);
                    let absolute_logo_path = absolute_logo_path_clone;
                    if !data.get().logo.is_empty() {
                        let logo = data.get().logo.clone();
                        let logo_clone = logo.clone();
                        view! {
                            <div>
                                <button
                                    disabled=disabled()
                                    on:click={
                                        let remove_image_clone = Arc::clone(&remove_image);
                                        let logo_clone_inner = logo_clone.clone();
                                        move |_| {
                                            let logo = logo_clone_inner.clone();
                                            (remove_image_clone)(logo, "logo")
                                        }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                ></button>
                                <img
                                    src=if absolute_logo_path {
                                        logo_clone.clone()
                                    } else {
                                        logo_clone
                                    }
                                    class="h-full w-full rounded-md object-contain"
                                    alt="logo"
                                />
                            </div>
                        }
                    } else {
                        view! {
                            <div class="w-full h-full flex items-center justify-center text-sm">
                                "No logo uploaded"
                            </div>
                        }
                    }
                }}
            </div>
            <div class="mt-4 flex flex-col gap-2">
                <label class=move || {
                    format!(
                        "w-min transition-opacity {}",
                        if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                    )
                }>
                    <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                        <div>
                            {move || {
                                if uploading.get() {
                                    format!("Uploading... ({}%)", uploading_progress.get())
                                } else if !data.get().logo.is_empty() {
                                    "Change logo".to_string()
                                } else {
                                    "Upload logo".to_string()
                                }
                            }}
                        </div>
                    </div>

                    <input
                        // disabled=disabled_clone()
                        on:change={
                            let on_select_clone = Arc::clone(&on_select);
                            move |e| (on_select_clone)(e, "logo")
                        }
                        type="file"
                        accept="image/*"
                        class="sr-only"
                    />
                </label>
            </div>
            <span class="text-sm font-medium leading-6 text-gray-900">"Images:"</span>
            <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                <For each=move || data.get().images.clone() key=|path| path.clone() let:path>
                    {
                        let remove_image_clone = Arc::clone(&remove_image);
                        let disabled_clone = Arc::clone(&disabled);
                        let asset_path_clone = Arc::clone(&asset_path);
                        let uploading_clone = uploading.clone();
                        let path_clone = path.clone();
                        view! {
                            <div class=move || {
                                format!(
                                    "p-1 shrink-0 border rounded-md w-52 h-52 relative transition-opacity {}",
                                    if uploading_clone.get() { "opacity-50" } else { "" },
                                )
                            }>
                                <button
                                    // disabled=disabled_clone()
                                    on:click={
                                        let remove_image_clone = Arc::clone(&remove_image_clone);
                                        let path_clone_inner = path_clone.clone();
                                        move |_| {
                                            let path = path_clone_inner.clone();
                                            (remove_image_clone)(path, "images")
                                        }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                ></button>
                                <img
                                    src=&path_clone
                                    class="h-full w-full rounded-md object-contain"
                                    alt=&format!("image {}", path_clone)
                                />
                            </div>
                        }
                    }
                </For>
                <Show when=move || data.get().images.is_empty() fallback=|| ()>
                    <div class="flex flex-1 text-sm items-center justify-center">
                        "No images added yet"
                    </div>
                </Show>
            </div>
            <div class="mt-4 flex flex-col gap-2">
                <label class=move || {
                    format!(
                        "w-min transition-opacity {}",
                        if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                    )
                }>
                    <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                        <div>
                            {move || {
                                if uploading.get() {
                                    format!("Uploading... ({}%)", uploading_progress.get())
                                } else {
                                    "Upload image".to_string()
                                }
                            }}
                        </div>
                    </div>

                    <input
                        // disabled=disabled_clone()
                        on:change={
                            let on_select_clone = Arc::clone(&on_select);
                            move |e| (on_select_clone)(e, "images")
                        }
                        type="file"
                        accept="image/*"
                        class="sr-only"
                    />
                </label>
            </div>
        </div>
    }
}
