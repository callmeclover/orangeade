use lol_html::{element, HtmlRewriter, Settings};

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

pub fn is_url_absolute(url: &str) -> bool {
    url.starts_with("//") || (url.contains("://") && url.contains(".") && url.contains("/") && url.find(":") < url.find("/"))
}

fn do_replink_logic(link: &str, hostname: &str) -> String {
    if !is_url_absolute(link) {
        if link.starts_with("/") {
            if hostname.ends_with("/") {
                return rem_last(hostname).to_owned() + link;
            }
            return hostname.to_owned() + link;
        } else {
            if hostname.ends_with("/") {
                return hostname.to_owned() + link;
            } else {
                return hostname.to_owned() + "/" + link;
            }
        }
    } else {
        return link.to_string();
    }
}

pub async fn replace_resources(source: &str, hostname: &str) -> String {
    // This function has 4 parts.

    // 1. Get all occurences of a <link> or <script> in source.
    // 2. Regex href or src to check if it's relative or absolute.
    // 3. If it's relative, replace it with a given hostname.
    // 4. Return modified document.

    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("link[href]", |el| {
                    let href = el
                        .get_attribute("href")
                        .expect("href was required");
                    
                    el.set_attribute("href", &do_replink_logic(&href, hostname))?;

                    Ok(())
                }),
                element!("a[href]", |el| {
                    // TODO: convert '/test' to '/plsgrab?link=hostname+test'
                    let href = el
                        .get_attribute("href")
                        .expect("href was required");
                    
                    el.set_attribute("href", &("/plsgrab?link=".to_owned() + &do_replink_logic(&href, hostname)))?;

                    Ok(())
                }),
                element!("*[src]", |el| {
                    let src = el
                        .get_attribute("src")
                        .expect("src was required");
                    
                    el.set_attribute("src", &do_replink_logic(&src, hostname))?;

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    let _ = rewriter.write(source.as_bytes());
    rewriter.end();

    return String::from_utf8(output).ok().unwrap();
}