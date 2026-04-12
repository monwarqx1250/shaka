use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum ConfigForm {
    Map(BTreeMap<String, String>),
    Pairs(Vec<(String, String)>),
}

pub fn load_merged_config() -> Result<IndexMap<String, String>, String> {
    load_from_paths(config_paths())
}

fn load_from_paths(paths: Vec<PathBuf>) -> Result<IndexMap<String, String>, String> {
    let mut merged = IndexMap::new();

    for path in paths {
        if !path.exists() {
            continue;
        }

        let pairs = parse_config_file(&path)?;
        merge_pairs(&mut merged, pairs);
    }

    Ok(merged)
}

fn parse_config_file(path: &Path) -> Result<Vec<(String, String)>, String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed reading {}: {err}", path.display()))?;

    if path.extension().is_some_and(|ext| ext == "yaml") {
        parse_yaml_pairs(&content)
            .map_err(|err| format!("failed parsing YAML {}: {err}", path.display()))
    } else {
        parse_jsonc_pairs(&content)
            .map_err(|err| format!("failed parsing JSONC {}: {err}", path.display()))
    }
}

fn parse_yaml_pairs(content: &str) -> Result<Vec<(String, String)>, serde_yaml_ng::Error> {
    let parsed: ConfigForm = serde_yaml_ng::from_str(content)?;
    Ok(normalize(parsed))
}

fn parse_jsonc_pairs(content: &str) -> Result<Vec<(String, String)>, String> {
    let value = jsonc_parser::parse_to_serde_value(content, &Default::default())
        .map_err(|err| err.to_string())?
        .ok_or_else(|| "empty JSON input".to_string())?;

    let parsed: ConfigForm = serde_json::from_value(value).map_err(|err| err.to_string())?;
    Ok(normalize(parsed))
}

fn normalize(parsed: ConfigForm) -> Vec<(String, String)> {
    match parsed {
        ConfigForm::Map(map) => map.into_iter().collect(),
        ConfigForm::Pairs(pairs) => pairs,
    }
}

fn merge_pairs(merged: &mut IndexMap<String, String>, pairs: Vec<(String, String)>) {
    for (key, value) in pairs {
        if merged.contains_key(&key) {
            merged.shift_remove(&key);
        }
        merged.insert(key, value);
    }
}

fn config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(6);

    if let Some(home) = home_dir() {
        paths.push(home.join(".config").join("shaka.yaml"));
        paths.push(home.join(".config").join("shaka.json"));
        paths.push(home.join(".shaka.yaml"));
        paths.push(home.join(".shaka.json"));
    }

    if let Ok(current_dir) = std::env::current_dir() {
        paths.push(current_dir.join(".shaka.yaml"));
        paths.push(current_dir.join(".shaka.json"));
    }

    paths
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

#[cfg(test)]
mod tests {
    use super::{load_from_paths, merge_pairs, parse_jsonc_pairs, parse_yaml_pairs};
    use indexmap::IndexMap;
    use std::fs;

    fn unique_dir() -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("shaka-test-{nanos}"))
    }

    #[test]
    fn parses_yaml_map() {
        let pairs = parse_yaml_pairs("dc: docker compose\nls: eza\n").unwrap();
        assert_eq!(
            pairs,
            vec![
                ("dc".to_string(), "docker compose".to_string()),
                ("ls".to_string(), "eza".to_string())
            ]
        );
    }

    #[test]
    fn parses_yaml_pairs_array() {
        let pairs = parse_yaml_pairs("- [dc, docker compose]\n- [ls, eza]\n").unwrap();
        assert_eq!(
            pairs,
            vec![
                ("dc".to_string(), "docker compose".to_string()),
                ("ls".to_string(), "eza".to_string())
            ]
        );
    }

    #[test]
    fn parses_jsonc_map() {
        let pairs =
            parse_jsonc_pairs("{\n // comment\n \"dc\": \"docker compose\",\n \"ls\": \"eza\"\n}")
                .unwrap();
        assert_eq!(
            pairs,
            vec![
                ("dc".to_string(), "docker compose".to_string()),
                ("ls".to_string(), "eza".to_string())
            ]
        );
    }

    #[test]
    fn parses_jsonc_pairs_array() {
        let pairs = parse_jsonc_pairs("[[\"dc\",\"docker compose\"],[\"ls\",\"eza\"]]").unwrap();
        assert_eq!(
            pairs,
            vec![
                ("dc".to_string(), "docker compose".to_string()),
                ("ls".to_string(), "eza".to_string())
            ]
        );
    }

    #[test]
    fn duplicate_key_moves_to_latest_position() {
        let mut merged = IndexMap::new();
        merge_pairs(
            &mut merged,
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
            ],
        );
        merge_pairs(
            &mut merged,
            vec![
                ("a".to_string(), "3".to_string()),
                ("c".to_string(), "4".to_string()),
            ],
        );

        let items: Vec<_> = merged.into_iter().collect();
        assert_eq!(
            items,
            vec![
                ("b".to_string(), "2".to_string()),
                ("a".to_string(), "3".to_string()),
                ("c".to_string(), "4".to_string()),
            ]
        );
    }

    #[test]
    fn later_files_override_earlier_files() {
        let dir = unique_dir();
        fs::create_dir_all(&dir).unwrap();

        let global = dir.join("global.yaml");
        let project = dir.join("project.json");

        fs::write(&global, "dc: docker compose\nls: eza\n").unwrap();
        fs::write(&project, "{\"dc\":\"docker compose -f dev.yml\"}").unwrap();

        let merged = load_from_paths(vec![global, project]).unwrap();
        let items: Vec<_> = merged.into_iter().collect();

        assert_eq!(
            items,
            vec![
                ("ls".to_string(), "eza".to_string()),
                ("dc".to_string(), "docker compose -f dev.yml".to_string())
            ]
        );

        fs::remove_dir_all(dir).unwrap();
    }
}
