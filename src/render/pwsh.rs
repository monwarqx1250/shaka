use indexmap::IndexMap;

pub fn render(entries: &IndexMap<String, String>, conflict_mode: bool) -> String {
    let mut out = String::new();

    for (name, command) in entries {
        if !conflict_mode {
            out.push_str("Remove-Item Alias:");
            out.push_str(name);
            out.push_str(" -ErrorAction Ignore\n");
        }
        out.push_str("function ");
        out.push_str(name);
        out.push_str(" { ");
        out.push_str(command);
        out.push_str(" @args }\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::render;
    use indexmap::IndexMap;

    #[test]
    fn renders_pwsh_without_conflicts() {
        let mut entries = IndexMap::new();
        entries.insert("dc".to_string(), "docker compose".to_string());
        assert_eq!(
            render(&entries, false),
            "Remove-Item Alias:dc -ErrorAction Ignore\nfunction dc { docker compose @args }\n"
        );
    }

    #[test]
    fn renders_pwsh_conflict_mode() {
        let mut entries = IndexMap::new();
        entries.insert("dc".to_string(), "docker compose".to_string());
        assert_eq!(
            render(&entries, true),
            "function dc { docker compose @args }\n"
        );
    }
}
