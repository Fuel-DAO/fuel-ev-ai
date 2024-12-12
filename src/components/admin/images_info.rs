// images_info.rs
use crate::canister::PROVISION_ID;
use crate::state::asset_manager::*;
use crate::state::canisters::Canisters;
use crate::TEMP_ASSET_CANISTER_ID;
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use gloo_file::Blob;
use leptos::logging::log;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

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
    // Access the Canisters context as RwSignal<Option<Rc<Canisters>>>
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Local signals for upload state
    let uploading = create_rw_signal(false);
    let uploading_progress = create_rw_signal(0);
    let error_asset = create_rw_signal(false);
    let error_logo = create_rw_signal(false);
    let error_message = create_rw_signal(String::new());

    // Determine if input elements should be disabled
    let disabled = move || uploading.get();

    // Handler for file selection (upload)
    // Handler for file selection (upload)
    let on_select = {
        let uploading = uploading.clone();
        let uploading_progress = uploading_progress.clone();
        let data = data.clone();
        let error_asset = error_asset.clone();
        let error_logo = error_logo.clone();
        let error_message = error_message.clone();
        let canisters_signal = canisters_signal.clone();
        let upload_canister_id = upload_canister_id.clone();
        let asset_canister_id = asset_canister_id.clone();

        Rc::new(move |event: Event, file_type: &'static str| {
            // Reset error states based on file type
            if file_type == "logo" {
                error_logo.set(false);
            } else {
                error_asset.set(false);
            }
            error_message.set(String::new());

            // Extract the file input element
            let input = match event.target() {
                Some(target) => match target.dyn_into::<HtmlInputElement>() {
                    Ok(input) => input,
                    Err(_) => return,
                },
                None => return,
            };

            // Retrieve the selected files
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

            uploading.set(true);
            uploading_progress.set(0);

            // Clone necessary variables for async task
            let canisters_signal = canisters_signal.clone();
            let uploading = uploading.clone();
            let uploading_progress = uploading_progress.clone();
            let data = data.clone();
            let error_asset = error_asset.clone();
            let error_logo = error_logo.clone();
            let error_message = error_message.clone();
            let upload_canister_id = upload_canister_id.clone();
            let asset_canister_id = asset_canister_id.clone();

            // Perform the upload asynchronously
            spawn_local(async move {
                // Check if canisters are available
                let canisters_option = canisters_signal.get();
                if canisters_option.is_none() {
                    log::error!("Canisters not available.");
                    error_message.set("Canisters not available.".to_string());
                    if file_type == "logo" {
                        error_logo.set(true);
                    } else {
                        error_asset.set(true);
                    }
                    uploading.set(false);
                    return;
                }
                let canisters = canisters_option.unwrap();

                let manager = canisters.asset_manager();

                // Iterate over each selected file
                for i in 0..files.length() {
                    let file = match files.get(i) {
                        Some(f) => f,
                        None => continue,
                    };

                    let file_name = file.name();
                    let blob = Blob::from(file.clone());

                    // Read the file data
                    let file_data = match read_as_bytes(&blob).await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            log::error!("Failed to read file data: {:?}", e);
                            error_message.set("Failed to read file data.".to_string());
                            if file_type == "logo" {
                                error_logo.set(true);
                            } else {
                                error_asset.set(true);
                            }
                            continue;
                        }
                    };


                let future =  manager.store(StoreArg{key: format!("/{}", &file_name), content: file_data, sha256: None, content_type: "image/png".to_string(), content_encoding: "identity".to_string() });
                    // Upload the file
                    match future.await {
                        Ok(ret) => {
                            match ret {
    crate::canister::asset_proxy::StoreRet::Ok(_) => {
        if file_type == "logo" {
            data.update(|d| d.logo = format!("/{}", &file_name));
        } else {
            data.update(|d| d.images.push( format!("/{}", &file_name)));
        }
        uploading_progress.set(100);
    },
    crate::canister::asset_proxy::StoreRet::Err(e) => {
        log::error!("Upload failed: {}", e);
                            error_message.set(format!("Upload failed: {}", e));
                            if file_type == "logo" {
                                error_logo.set(true);
                            } else {
                                error_asset.set(true);
                            }
    },
}
                        }
                        Err(e) => {
                            log::error!("Upload failed: {}", e);
                            error_message.set(format!("Upload failed: {}", e));
                            if file_type == "logo" {
                                error_logo.set(true);
                            } else {
                                error_asset.set(true);
                            }
                        }
                    }
                }

                uploading.set(false);
                input.set_value("");
            });
        }) as Rc<dyn Fn(Event, &'static str) + 'static>
    };

    // Handler for removing images or logo
    let remove_image = {
        let data = data.clone();
        let error_asset = error_asset.clone();
        let error_logo = error_logo.clone();
        let error_message = error_message.clone();
        let canisters_signal = canisters_signal.clone();
        let upload_canister_id = upload_canister_id.clone();
        let asset_canister_id = asset_canister_id.clone();

        Rc::new(move |path: String, file_type: &'static str| {
            let data = data.clone();
            let error_asset = error_asset.clone();
            let error_logo = error_logo.clone();
            let error_message = error_message.clone();
            let canisters_signal = canisters_signal.clone();
            let upload_canister_id = upload_canister_id.clone();
            let asset_canister_id = asset_canister_id.clone();
            let file_type = file_type.to_string();

            spawn_local(async move {
                // Check if canisters are available
                let canisters_option = canisters_signal.get();
                if canisters_option.is_none() {
                    log::error!("Canisters not available.");
                    error_message.set("Canisters not available.".to_string());
                    if file_type == "logo" {
                        error_logo.set(true);
                    } else {
                        error_asset.set(true);
                    }
                    return;
                }
                let canisters = canisters_option.unwrap();

                let manager = canisters.asset_manager();

                // Delete the asset
                match manager.delete(path.clone()).await {
                    Ok(_) => {
                        if file_type == "logo" {
                            data.update(|d| d.logo.clear());
                        } else {
                            data.update(|d| d.images.retain(|img| img != &path));
                        }
                    }
                    Err(e) => {
                        log::error!("Deletion failed: {}", e);
                        error_message.set(format!("Deletion failed: {}", e));
                        if file_type == "logo" {
                            error_logo.set(true);
                        } else {
                            error_asset.set(true);
                        }
                    }
                }
            });
        }) as Rc<dyn Fn(String, &'static str) + 'static>
    };

    // Function to construct the full asset path
    // let asset_path = move |path: &str| format!("{}/{}", asset_canister_id, path);
    let asset_path = move |path: &str| format!("https://{}.icp0.io{}", TEMP_ASSET_CANISTER_ID,  &path);

    // Clone necessary variables for rendering
    let data_clone = data.clone();
    let remove_image_clone = remove_image.clone();
    let disabled_clone = disabled.clone();
    let asset_path_clone = asset_path.clone();
    let absolute_logo_path_clone = absolute_logo_path;

    view! {
        <div class="flex flex-col -mt-8 gap-4">
            <span class="text-sm font-medium leading-6 text-gray-900 mt-8">"Thumbnail/Logo:"</span>
            <div class="h-[14rem] w-[14rem] p-2 border rounded relative">
                {move || {
                    let data = data_clone.get();
                    let remove_image = remove_image_clone.clone();
                    let disabled = disabled_clone();
                    let asset_path = asset_path_clone.clone();
                    let absolute_logo_path = absolute_logo_path_clone;
                    if !data.logo.is_empty() {
                        let logo = data.logo.clone();
                        view! {
                            <div>
                                <button
                                    disabled=disabled
                                    on:click={
                                        let remove_image = remove_image.clone();
                                        let logo_clone = logo.clone();
                                        move |_| { (remove_image)(logo_clone.clone(), "logo") }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                    aria-label="Remove logo"
                                >
                                    "X"
                                </button>
                                <img
                                    src=if absolute_logo_path {
                                        logo.clone()
                                    } else {
                                        asset_path(&logo).to_string()
                                    }
                                    class="h-full w-full rounded-md object-contain"
                                    alt="Logo"
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
                        disabled=disabled()
                        on:change={
                            let on_select_clone = on_select.clone();
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
                        let remove_image_clone = remove_image.clone();
                        let disabled_clone = disabled.clone();
                        let asset_path_clone = asset_path.clone();
                        let path_clone = path.clone();
                        view! {
                            <div class=move || {
                                format!(
                                    "p-1 shrink-0 border rounded-md w-52 h-52 relative transition-opacity {}",
                                    if uploading.get() { "opacity-50" } else { "" },
                                )
                            }>
                                <button
                                    disabled=disabled_clone()
                                    on:click={
                                        let remove_image_clone = remove_image_clone.clone();
                                        let path_clone_inner = path_clone.clone();
                                        move |_| {
                                            (remove_image_clone)(path_clone_inner.clone(), "images")
                                        }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                    aria-label=format!("Remove image {}", path_clone)
                                >
                                    "X"
                                </button>
                                <img
                                    src=asset_path(&path_clone)
                                    class="h-full w-full rounded-md object-contain"
                                    alt=format!("Image {}", path_clone)
                                />
                            </div>
                        }
                    }
                </For>
                <Show when=move || data.get().images.is_empty()>
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
                        disabled=disabled()
                        on:change={
                            let on_select_clone = on_select.clone();
                            move |e| (on_select_clone)(e, "images")
                        }
                        type="file"
                        accept="image/*"
                        multiple=true
                        class="sr-only"
                    />
                </label>
            </div>

            <Show when=move || !error_message.get().is_empty()>
                <div class="text-red-500 text-sm mt-2">{error_message.get()}</div>
            </Show>
        </div>
    }
}
