use std::collections::HashMap;

type Extras = HashMap<String, HashMap<String, String>>;

pub fn get_extras() -> Extras {
    let mut extras: Extras = HashMap::new();
    let mut display_options: HashMap<String, String> = HashMap::new();
    display_options.insert("contentType".to_string(), "text/markdown".to_string());
    extras.insert("client::display".to_string(), display_options);
    extras
}

pub fn fix_newlines(message: String) -> String {
    // fix escaped \n for real newlines with two white spaces for Markdown
    // in Markdown, "  \n" is needed for a line break
    message.replace(r"\n", "  \n")
}

pub fn clean_url(server_url: &String, app_key: &String) -> String {
    // build clean url
    let url_out: String = format!(
        "{}/message?token={}",
        server_url.trim_end_matches('/'),
        app_key
    );
    url_out
}
