use crate::state::asset_manager::AssetManager;
use crate::state::canisters::Canisters;
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use web_sys::{Event, File, HtmlInputElement};

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
    let canisters_resource =
        use_context::<Rc<RefCell<Canisters>>>().expect("Canisters not provided");
    let loading =
        use_context::<ReadSignal<bool>>().unwrap_or_else(|| create_rw_signal(false).read_only());

    let uploading = create_rw_signal(false);
    let uploading_progress = create_rw_signal(0);
    let error_asset = create_rw_signal(false);
    let error_logo = create_rw_signal(false);

    let disabled = move || uploading.get() || loading.get();

    let on_select = {
        let uploading = uploading.clone();
        let uploading_progress = uploading_progress.clone();
        let data = data.clone();
        let error_asset = error_asset.clone();
        let error_logo = error_logo.clone();
        let canisters_resource = canisters_resource.clone();
        let upload_canister_id = upload_canister_id.clone();
        let asset_canister_id = asset_canister_id.clone();

        Arc::new(move |event: Event, file_type: &'static str| {
            if file_type == "logo" {
                error_logo.set(false);
            } else {
                error_asset.set(false);
            }

            let input = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
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

            let canisters_resource = canisters_resource.clone();
            let uploading = uploading.clone();
            let uploading_progress = uploading_progress.clone();
            let data = data.clone();
            let error_asset = error_asset.clone();
            let error_logo = error_logo.clone();
            let upload_canister_id = upload_canister_id.clone();
            let asset_canister_id = asset_canister_id.clone();

            spawn_local(async move {
                let canisters = canisters_resource.borrow();
                let agent = &canisters.agent;
                let upload_principal = Principal::from_text(&upload_canister_id).unwrap();
                let asset_principal = Principal::from_text(&asset_canister_id).unwrap();
                let manager = AssetManager::new(upload_principal, asset_principal, agent);

                let file_name = file.name();
                let file_data = match read_as_bytes(&file).await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        log::error!("Failed to read file data: {:?}", e);
                        if file_type == "logo" {
                            error_logo.set(true);
                        } else {
                            error_asset.set(true);
                        }
                        uploading.set(false);
                        return;
                    }
                };

                match manager.store(file_data, file_name.clone()).await {
                    Ok(url) => {
                        if file_type == "images" {
                            data.update(|d| d.images.push(url.clone()));
                        } else {
                            data.update(|d| d.logo = url.clone());
                        }
                        uploading_progress.set(100);
                    }
                    Err(e) => {
                        log::error!("Upload failed: {}", e);
                        if file_type == "logo" {
                            error_logo.set(true);
                        } else {
                            error_asset.set(true);
                        }
                    }
                }

                uploading.set(false);
                input.set_value("");
            });
        }) as Arc<dyn Fn(Event, &'static str) + Send + Sync + 'static>
    };

    let remove_image = {
        let data = data.clone();
        let error_asset = error_asset.clone();
        let error_logo = error_logo.clone();
        let canisters_resource = canisters_resource.clone();
        let upload_canister_id = upload_canister_id.clone();
        let asset_canister_id = asset_canister_id.clone();

        Arc::new(move |path: String, file_type: &'static str| {
            let data = data.clone();
            let error_asset = error_asset.clone();
            let error_logo = error_logo.clone();
            let canisters_resource = canisters_resource.clone();
            let upload_canister_id = upload_canister_id.clone();
            let asset_canister_id = asset_canister_id.clone();
            let file_type = file_type.to_string();

            spawn_local(async move {
                let canisters = canisters_resource.borrow();
                let agent = &canisters.agent;
                let upload_principal = Principal::from_text(&upload_canister_id).unwrap();
                let asset_principal = Principal::from_text(&asset_canister_id).unwrap();
                let manager = AssetManager::new(upload_principal, asset_principal, agent);

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
                        if file_type == "logo" {
                            error_logo.set(true);
                        } else {
                            error_asset.set(true);
                        }
                    }
                }
            });
        }) as Arc<dyn Fn(String, &'static str) + Send + Sync + 'static>
    };

    let asset_path = move |path: &str| format!("{}/{}", asset_canister_id, path);

    let data_clone = data.clone();
    let remove_image_clone = Arc::clone(&remove_image);
    let disabled_clone = disabled.clone();
    let asset_path_clone = asset_path.clone();
    let absolute_logo_path_clone = absolute_logo_path;

    view! {
        <div class="flex flex-col -mt-8 gap-4">
            <span class="text-sm font-medium leading-6 text-gray-900 mt-8">"Thumbnail/Logo:"</span>
            <div class="h-[14rem] w-[14rem] p-2 border rounded relative">
                {move || {
                    let data = data_clone.get();
                    let remove_image = Arc::clone(&remove_image_clone);
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
                                        let remove_image = Arc::clone(&remove_image);
                                        let logo_clone = logo.clone();
                                        move |_| { (remove_image)(logo_clone.clone(), "logo") }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                >
                                    "X"
                                </button>
                                <img
                                    src=if absolute_logo_path {
                                        logo.clone()
                                    } else {
                                        asset_path(&logo)
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
                        disabled=disabled()
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
                                        let remove_image_clone = Arc::clone(&remove_image_clone);
                                        let path_clone_inner = path_clone.clone();
                                        move |_| {
                                            (remove_image_clone)(path_clone_inner.clone(), "images")
                                        }
                                    }
                                    class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                >
                                    "X"
                                </button>
                                <img
                                    src=asset_path(&path_clone)
                                    class="h-full w-full rounded-md object-contain"
                                    alt=format!("image {}", path_clone)
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
                        disabled=disabled()
                        on:change={
                            let on_select_clone = Arc::clone(&on_select);
                            move |e| (on_select_clone)(e, "images")
                        }
                        type="file"
                        accept="image/*"
                        multiple=true
                        class="sr-only"
                    />
                </label>
            </div>
        </div>
    }
}
