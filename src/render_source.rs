/// Normalize fenced blocks for the intentionally plain v0.1 renderer.
///
/// Fenced `math` becomes display math; other language identifiers are removed
/// so no syntax definition is requested.
#[must_use]
pub fn normalize_for_rendering(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut active: Option<(char, usize, bool)> = None;

    for segment in source.split_inclusive('\n') {
        let (line, ending) = segment.strip_suffix("\r\n").map_or_else(
            || {
                segment
                    .strip_suffix('\n')
                    .map_or((segment, ""), |line| (line, "\n"))
            },
            |line| (line, "\r\n"),
        );
        let trimmed = line.trim_start_matches(' ');
        let indent = line.len() - trimmed.len();

        if let Some((fence_char, fence_len, is_math)) = active {
            if indent <= 3 && is_closing_fence(trimmed, fence_char, fence_len) {
                if is_math {
                    result.push_str(&" ".repeat(indent));
                    result.push_str("$$");
                } else {
                    result.push_str(line);
                }
                result.push_str(ending);
                active = None;
            } else {
                result.push_str(line);
                result.push_str(ending);
            }
            continue;
        }

        if indent <= 3
            && let Some((fence_char, fence_len, info)) = opening_fence(trimmed)
        {
            let is_math = info.eq_ignore_ascii_case("math");
            result.push_str(&" ".repeat(indent));
            if is_math {
                result.push_str("$$");
            } else {
                result.extend(std::iter::repeat_n(fence_char, fence_len));
            }
            result.push_str(ending);
            active = Some((fence_char, fence_len, is_math));
        } else {
            result.push_str(line);
            result.push_str(ending);
        }
    }

    result
}

fn opening_fence(line: &str) -> Option<(char, usize, &str)> {
    let fence_char = line.chars().next()?;
    if fence_char != '`' && fence_char != '~' {
        return None;
    }
    let fence_len = line.chars().take_while(|ch| *ch == fence_char).count();
    if fence_len < 3 {
        return None;
    }
    let info = line[fence_len..].trim();
    if fence_char == '`' && info.contains('`') {
        return None;
    }
    Some((fence_char, fence_len, info))
}

fn is_closing_fence(line: &str, fence_char: char, minimum_len: usize) -> bool {
    let len = line.chars().take_while(|ch| *ch == fence_char).count();
    len >= minimum_len && line[len..].trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_code_language_without_touching_content() {
        assert_eq!(
            normalize_for_rendering("```rust\nfn main() {}\n```\n"),
            "```\nfn main() {}\n```\n"
        );
    }

    #[test]
    fn converts_math_fences() {
        assert_eq!(
            normalize_for_rendering("before\n~~~math\nx^2\n~~~\nafter"),
            "before\n$$\nx^2\n$$\nafter"
        );
    }

    #[test]
    fn leaves_inline_code_alone() {
        assert_eq!(normalize_for_rendering("Use ``` here."), "Use ``` here.");
    }
}
