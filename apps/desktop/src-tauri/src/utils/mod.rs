use std::path::PathBuf;

/// Generate a safe filename from a title
pub fn title_to_filename(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '-',
            _ => '_',
        })
        .collect::<String>()
        .to_lowercase()
}

/// Generate a markdown filename with ID and slug
pub fn generate_markdown_filename(id: &str, title: &str) -> String {
    let slug = title_to_filename(title);
    format!("{}-{}.md", id, slug)
}

/// Get the data directory for a specific entity type
pub fn get_entity_directory(base_path: &PathBuf, entity_type: &str) -> PathBuf {
    match entity_type {
        "area" => base_path.join("areas"),
        "goal" => base_path.join("goals"),
        "project" => base_path.join("projects"),
        "task" => base_path.join("tasks"),
        _ => panic!("Unknown entity type: {}", entity_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_filename() {
        assert_eq!(title_to_filename("Hello World"), "hello-world");
        assert_eq!(title_to_filename("Test 123"), "test-123");
        assert_eq!(title_to_filename("Special!@#$%^&*()"), "special__________");
    }

    #[test]
    fn test_generate_markdown_filename() {
        let filename = generate_markdown_filename("abc123", "My Test File");
        assert_eq!(filename, "abc123-my-test-file.md");
    }
}