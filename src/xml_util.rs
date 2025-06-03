/// A utility struct for XML operations.
pub struct XmlUtil;

impl XmlUtil {
    /// Removes all occurrences of the specified tag and its content from the input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process
    /// * `tag_name` - The name of the tag to remove (without angle brackets)
    ///
    /// # Returns
    ///
    /// An Option containing a new String with all occurrences of the specified tag and its content removed,
    /// or None if no tags were found and removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::xml_util::XmlUtil;
    ///
    /// let input = "Hello <div>unwanted content</div> world!";
    /// let result = XmlUtil::remove_tag(input, "div");
    /// assert_eq!(result, Some("Hello  world!".to_string()));
    ///
    /// let input_no_tags = "Hello world!";
    /// let result = XmlUtil::remove_tag(input_no_tags, "div");
    /// assert_eq!(result, None);
    /// ```
    pub fn remove_tag(input: &str, tag_name: &str) -> Option<String> {
        let mut result = input.to_string();
        let mut removed_any = false;

        // Create the opening and closing tag patterns
        let opening_tag = format!("<{}", tag_name);
        let closing_tag = format!("</{}>", tag_name);

        loop {
            // Find the opening tag
            let start_pos = match result.find(&opening_tag) {
                Some(pos) => pos,
                None => break, // No more opening tags found
            };

            // Find the end of the opening tag (could have attributes)
            let tag_end = match result[start_pos..].find('>') {
                Some(pos) => start_pos + pos + 1,
                None => break, // Malformed tag, stop processing
            };

            // Find the corresponding closing tag
            let end_pos = match result[tag_end..].find(&closing_tag) {
                Some(pos) => tag_end + pos + closing_tag.len(),
                None => break, // No matching closing tag found
            };

            // Remove the entire tag and its content
            result.replace_range(start_pos..end_pos, "");
            removed_any = true;
        }

        if removed_any { Some(result) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_simple_tag() {
        let input = "Hello <div>unwanted content</div> world!";
        let result = XmlUtil::remove_tag(input, "div");
        assert_eq!(result, Some("Hello  world!".to_string()));
    }

    #[test]
    fn test_remove_tag_with_attributes() {
        let input = "Hello <span class=\"highlight\">unwanted content</span> world!";
        let result = XmlUtil::remove_tag(input, "span");
        assert_eq!(result, Some("Hello  world!".to_string()));
    }

    #[test]
    fn test_remove_multiple_tags() {
        let input = "Hello <div>first</div> and <div>second</div> world!";
        let result = XmlUtil::remove_tag(input, "div");
        assert_eq!(result, Some("Hello  and  world!".to_string()));
    }

    #[test]
    fn test_remove_nested_tags() {
        let input = "Hello <div><span>nested content</span></div> world!";
        let result = XmlUtil::remove_tag(input, "div");
        assert_eq!(result, Some("Hello  world!".to_string()));
    }

    #[test]
    fn test_no_matching_tag() {
        let input = "Hello world!";
        let result = XmlUtil::remove_tag(input, "div");
        assert_eq!(result, None);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let result = XmlUtil::remove_tag(input, "div");
        assert_eq!(result, None);
    }

    #[test]
    fn test_self_closing_tag_ignored() {
        let input = "Hello <img src=\"test.jpg\" /> world!";
        let result = XmlUtil::remove_tag(input, "img");
        assert_eq!(result, None);
    }
}
