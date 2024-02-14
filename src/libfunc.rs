// replace `source: &str` with document typing later
pub fn replace_resources(source: &str, hostname: &str) {
    // This function has 4 parts.

    // 1. Get all occurences of a <link> or <script> in source.
    // 2. Regex href or src to check if it's relative or absolute.
    // 3. If it's relative, replace it with a given hostname.
    // 4. Return modified document.
}