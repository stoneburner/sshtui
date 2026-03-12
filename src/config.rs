//! Parse OpenSSH client config (~/.ssh/config) into host entries.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct HostEntry {
    /// Host pattern(s) from the config (first is used as display/connection name)
    pub names: Vec<String>,
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    /// Other notable options for display (e.g. Compression, LocalForward)
    pub extra: Vec<(String, String)>,
}

impl HostEntry {
    /// Display name for the list (first Host pattern)
    pub fn display_name(&self) -> &str {
        self.names.first().map(|s| s.as_str()).unwrap_or("?")
    }

    /// Name to pass to `ssh` (first pattern)
    pub fn ssh_name(&self) -> &str {
        self.display_name()
    }

    /// One-line summary: hostname, user, port, key
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if let Some(ref h) = self.hostname {
            parts.push(h.clone());
        } else {
            parts.push("(no hostname)".to_string());
        }
        if let Some(ref u) = self.user {
            parts.push(format!("user={}", u));
        }
        if let Some(p) = self.port {
            parts.push(format!("port={}", p));
        }
        if let Some(ref k) = self.identity_file {
            parts.push(format!("key={}", k));
        }
        parts.join(" ")
    }

    /// True if this entry looks like a wildcard-only host (e.g. "Host *" or "Host nf* 10.*")
    fn is_wildcard_only(names: &[String]) -> bool {
        names.iter().all(|n| n.contains('*') || n.contains('?'))
    }
}

/// Parse an SSH config file and return host entries.
/// Skips "Host *" and other wildcard-only blocks so only connectable hosts are listed.
pub fn parse_config(path: &Path) -> std::io::Result<Vec<HostEntry>> {
    let content = fs::read_to_string(path)?;
    Ok(parse_config_str(&content))
}

pub fn parse_config_str(content: &str) -> Vec<HostEntry> {
    let mut entries = Vec::new();
    let mut current: Option<HostEntry> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.to_lowercase().starts_with("host ") {
            if let Some(entry) = current.take() {
                if !HostEntry::is_wildcard_only(&entry.names) {
                    entries.push(entry);
                }
            }
            let value = trimmed[5..].trim();
            let names: Vec<String> = value
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            if names.is_empty() {
                current = None;
                continue;
            }
            current = Some(HostEntry {
                names,
                ..HostEntry::default()
            });
            continue;
        }

        if let Some(ref mut entry) = current {
            if let Some((key, value)) = split_key_value(line) {
                let key_lower = key.to_lowercase();
                match key_lower.as_str() {
                    "hostname" => entry.hostname = Some(value.to_string()),
                    "user" => entry.user = Some(value.to_string()),
                    "port" => {
                        if let Ok(p) = value.parse::<u16>() {
                            entry.port = Some(p);
                        }
                    }
                    "identityfile" => entry.identity_file = Some(value.to_string()),
                    _ => entry.extra.push((key.to_string(), value.to_string())),
                }
            }
        }
    }

    if let Some(entry) = current {
        if !HostEntry::is_wildcard_only(&entry.names) {
            entries.push(entry);
        }
    }

    entries
}

/// Split first word (key) and rest (value). Keys are case-insensitive in SSH config.
fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let line = line.trim_start();
    let first_ws = line.find(char::is_whitespace)?;
    let key = line[..first_ws].trim_end();
    let value = line[first_ws..].trim_start();
    if key.is_empty() {
        None
    } else {
        Some((key, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_host_block() {
        let s = r#"
Host host-a
	Hostname 192.168.1.10
	User pi
	IdentityFile ~/.ssh/id_ed25519_sk
"#;
        let entries = parse_config_str(s);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].display_name(), "host-a");
        assert_eq!(entries[0].hostname.as_deref(), Some("192.168.1.10"));
        assert_eq!(entries[0].user.as_deref(), Some("pi"));
        assert_eq!(entries[0].identity_file.as_deref(), Some("~/.ssh/id_ed25519_sk"));
    }

    #[test]
    fn test_skip_wildcard() {
        let s = r#"
Host *
ForwardAgent yes

Host host-a
	Hostname 192.168.1.10
"#;
        let entries = parse_config_str(s);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].display_name(), "host-a");
    }

    #[test]
    fn test_parse_example_config() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("example_config");
        let entries = parse_config(&path).expect("example_config must exist and be readable");
        assert!(!entries.is_empty(), "example_config must contain at least one connectable host");
        assert_eq!(entries[0].display_name(), "host-a");
        assert_eq!(entries.last().map(|e| e.display_name()), Some("minimal"));
        assert!(
            entries.iter().all(|e| !e.display_name().contains('*')),
            "wildcard-only Host blocks must be skipped"
        );
    }
}
