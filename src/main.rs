mod libfunc;
use libfunc::*;

#[tokio::main]
async fn main() {
    let hostname: &str = "https://example.com";
    let source: &str = 
r#"<html>
        <head>
            <link rel="stylesheet" href="/dummy.css" />
            <script src="/dummy.js"></script>
        </head>
        <body>
        </body>
    </html>
    "#;

    // TODO: see that <link> and <script> in the source? we need to replace it's href (and src) to an absolute link (via resource injection and replacement)
    // see libfunc's replace_resources for this implementation
    let mod_source = replace_resources(source, hostname).await;
    println!("{}", mod_source)
}
