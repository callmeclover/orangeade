use lol_html::{element, HtmlRewriter, Settings};

pub fn is_url_absolute(url: &str) -> bool {
    url.starts_with("//") || (url.contains("://") && url.contains(".") && url.contains("/") && url.find(":") < url.find("/"))
}

// replace `source: &str` with document typing later
pub fn replace_resources(source: &str, hostname: &str) {
    // This function has 4 parts.
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("link[href]", |el| {
                    let href = el
                        .get_attribute("href")
                        .expect("href was required");
                    
                    println!(href)

                    //el.set_attribute("href", &href)?;

                    Ok(())
                }),
                element!("script[src]", |el| {
                    let src = el
                        .get_attribute("src")
                        .expect("src was required");
                    
                    println!(src)

                    //el.set_attribute("src", &src)?;

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    // 1. Get all occurences of a <link> or <script> in source.
    // 2. Regex href or src to check if it's relative or absolute.
    // 3. If it's relative, replace it with a given hostname.
    // 4. Return modified document.
}