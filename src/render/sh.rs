use indexmap::IndexMap;

pub fn render(entries: &IndexMap<String, String>) -> String {
    let mut out = String::new();

    for (name, command) in entries {
        out.push_str("alias ");
        out.push_str(name);
        out.push_str("='");
        out.push_str(&escape_single_quotes(command));
        out.push_str("'\n");
    }

    out
}

fn escape_single_quotes(input: &str) -> String {
    input.replace('\'', "'\\''")
}

#[cfg(test)]
mod tests {
    use super::render;
    use indexmap::IndexMap;

    #[test]
    fn renders_sh_aliases() {
        let mut entries = IndexMap::new();
        entries.insert("dc".to_string(), "docker compose".to_string());
        assert_eq!(render(&entries), "alias dc='docker compose'\n");
    }
}
