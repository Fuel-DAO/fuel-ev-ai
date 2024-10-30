use leptos::*;

use crate::canister::token::CollectionMetaData;

#[component]
pub fn CollectionImages(
    metadata: CollectionMetaData,
    asset_can_id: String,  // Canister ID for asset paths
) -> impl IntoView {

    let placeholder = "/public/icons/placeholder.png".to_string();
    let _images = if  metadata.images.is_empty() { vec!["/public/icons/placeholder.png".into(),"/public/icons/placeholder.png".into(),]  } else {metadata.images.clone()};
    let images =  _images
    .iter()
    .enumerate()
    .skip(1)
    .step_by(2)
    .map(|(i, _)| {
        let first = _images.get(i).cloned();
        let second = _images.get(i + 1).cloned();
        (first, second)
    })
    .collect::<Vec<_>>();
    view! { 
        <div class="flex flex-col lg:flex-row gap-2 lg:h-[40rem] w-full overflow-hidden overflow-x-auto relative">
            
                // If metadata.logo is present, display the logo
                <div class="absolute h-16 lg:h-28 top-4 shadow-md z-[2] left-4 w-16 lg:w-28 rounded-full overflow-hidden">
                        <img
                            alt="Collection logo"
                            class="h-full w-full object-cover object-center"
                            src={metadata.logo.clone()}
                        />
                </div>
            
            // Main image (uses _images[0] if present, otherwise placeholder)
            <img
                alt="Collection"
                src={_images.get(0).map(|img| asset_path(&asset_can_id, img)).unwrap_or_else(|| placeholder.clone())}
                class="rounded-xl lg:h-full lg:grow object-cover"
            />
            // Additional images displayed in pairs
            {images.iter().enumerate().map(|(i, (im1, im2))| view! { 
                <div class="flex basis-[25rem] shrink-0 flex-col gap-2 lg:h-full">
                    <img
                        alt={format!("Collection preview {}", i)}
                        src={im1.as_ref().map(|img| asset_path(&asset_can_id, img)).unwrap_or_else(|| placeholder.clone())}
                        class="rounded-xl h-1/2 object-cover"
                    />
                    {
                        // If im2 is present, display the second image in the pair
                        im2.as_ref().map(|img| view! { 
                            <img
                                alt={format!("Collection preview {}", i)}
                                src={asset_path(&asset_can_id, img)}
                                class="rounded-xl h-1/2 object-cover"
                            />
                        })
                    }
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

// Helper function to generate the asset path
fn asset_path(canister_id: &String, img: &String) -> String {
	 format!("https://{canister_id}.icp0.io{img}")
}
