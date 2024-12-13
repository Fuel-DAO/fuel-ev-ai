pub fn get_content_type(file_name: &str) -> &str {
    // Extract the extension from the file name
    match file_name.rsplit('.').next() {
        Some(ext) => match ext.to_lowercase().as_str() {
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "pdf" => "application/pdf",
            "txt" => "text/plain",
            "mp3" => "audio/mpeg",
            "mp4" => "video/mp4",
            "zip" => "application/zip",
            "csv" => "text/csv",
            _ => "application/octet-stream", // Default fallback
        },
        None => "application/octet-stream", // No extension found
    }
}