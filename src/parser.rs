use crate::model::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

pub fn parse(input: &str) -> Result<SquidConfig, ParseError> {
    let mut config = SquidConfig::default();

    for (line_num, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "acl" => parse_acl(&tokens, &mut config, line_num + 1)?,
            "http_access" => parse_access_rule(&tokens, &mut config.http_access, line_num + 1)?,
            "always_direct" => {
                parse_direct_rule(&tokens, &mut config.always_direct, line_num + 1)?;
            }
            "never_direct" => {
                parse_direct_rule(&tokens, &mut config.never_direct, line_num + 1)?;
            }
            "auth_param" => parse_auth_param(&tokens, raw_line, &mut config, line_num + 1)?,
            _ => {}
        }
    }

    Ok(config)
}

fn parse_acl(tokens: &[&str], config: &mut SquidConfig, line: usize) -> Result<(), ParseError> {
    if tokens.len() < 4 {
        return Err(ParseError {
            line,
            message: "acl requires at least: acl <name> <type> <value>".into(),
        });
    }

    let name = tokens[1].to_string();
    let acl_type = AclType::from_str(tokens[2]).map_err(|e| ParseError { line, message: e })?;

    let mut idx = 3;
    let mut case_insensitive = false;

    if idx < tokens.len() && tokens[idx] == "-i" {
        case_insensitive = true;
        idx += 1;
    }

    let values: Vec<String> = tokens[idx..].iter().map(|s| (*s).to_string()).collect();

    if let Some(existing) = config.acls.iter_mut().find(|a| a.name == name) {
        if existing.acl_type != acl_type {
            return Err(ParseError {
                line,
                message: format!(
                    "ACL '{}' redefined with different type ({} vs {})",
                    name, existing.acl_type, acl_type
                ),
            });
        }
        existing.values.extend(values);
        existing.case_insensitive |= case_insensitive;
    } else {
        config.acls.push(Acl {
            name,
            acl_type,
            case_insensitive,
            values,
        });
    }

    Ok(())
}

fn parse_acl_refs(tokens: &[&str], start: usize, line: usize) -> Result<Vec<AclRef>, ParseError> {
    if start >= tokens.len() {
        return Err(ParseError {
            line,
            message: "expected at least one ACL name".into(),
        });
    }

    Ok(tokens[start..]
        .iter()
        .map(|t| {
            if let Some(name) = t.strip_prefix('!') {
                AclRef {
                    negated: true,
                    name: name.to_string(),
                }
            } else {
                AclRef {
                    negated: false,
                    name: (*t).to_string(),
                }
            }
        })
        .collect())
}

fn parse_access_rule(
    tokens: &[&str],
    rules: &mut Vec<AccessRule>,
    line: usize,
) -> Result<(), ParseError> {
    if tokens.len() < 3 {
        return Err(ParseError {
            line,
            message: "http_access requires: http_access allow|deny <acl> ...".into(),
        });
    }

    let action = AccessAction::from_str(tokens[1]).map_err(|e| ParseError { line, message: e })?;

    let acl_refs = parse_acl_refs(tokens, 2, line)?;
    rules.push(AccessRule { action, acl_refs });
    Ok(())
}

fn parse_direct_rule(
    tokens: &[&str],
    rules: &mut Vec<DirectRule>,
    line: usize,
) -> Result<(), ParseError> {
    if tokens.len() < 3 {
        return Err(ParseError {
            line,
            message: format!("{} requires: {} allow|deny <acl> ...", tokens[0], tokens[0]),
        });
    }

    let action = AccessAction::from_str(tokens[1]).map_err(|e| ParseError { line, message: e })?;

    let acl_refs = parse_acl_refs(tokens, 2, line)?;
    rules.push(DirectRule { action, acl_refs });
    Ok(())
}

fn parse_auth_param(
    tokens: &[&str],
    raw_line: &str,
    config: &mut SquidConfig,
    line: usize,
) -> Result<(), ParseError> {
    if tokens.len() < 3 {
        return Err(ParseError {
            line,
            message: "auth_param requires: auth_param basic <directive> <value>".into(),
        });
    }

    if tokens[1] != "basic" {
        return Ok(());
    }

    if tokens.len() < 4 {
        return Err(ParseError {
            line,
            message: "auth_param basic requires a directive and value".into(),
        });
    }

    let value_start = raw_line.find(tokens[3]).unwrap_or(0);
    let value = raw_line[value_start..].trim().to_string();

    match tokens[2] {
        "program" => config.auth_param.program = Some(value),
        "children" => config.auth_param.children = Some(tokens[3].to_string()),
        "realm" => config.auth_param.realm = Some(value),
        "credentialsttl" => config.auth_param.credentialsttl = Some(value),
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_acls() {
        let input = r#"
acl localnet src 10.0.0.0/8
acl localnet src 172.16.0.0/12
acl localnet src 192.168.0.0/16
acl SSL_ports port 443
acl Safe_ports port 80 443 8080
acl CONNECT method CONNECT
acl blocked dstdomain .facebook.com .twitter.com
"#;
        let config = parse(input).unwrap();
        assert_eq!(config.acls.len(), 5);

        let localnet = &config.acls[0];
        assert_eq!(localnet.name, "localnet");
        assert_eq!(localnet.acl_type, AclType::Src);
        assert_eq!(
            localnet.values,
            vec!["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"]
        );

        let safe = &config.acls[2];
        assert_eq!(safe.name, "Safe_ports");
        assert_eq!(safe.values, vec!["80", "443", "8080"]);

        let blocked = &config.acls[4];
        assert_eq!(blocked.acl_type, AclType::DstDomain);
        assert_eq!(blocked.values, vec![".facebook.com", ".twitter.com"]);
    }

    #[test]
    fn parse_case_insensitive_flag() {
        let input = "acl badurls url_regex -i /porn/\n";
        let config = parse(input).unwrap();
        assert!(config.acls[0].case_insensitive);
        assert_eq!(config.acls[0].values, vec!["/porn/"]);
    }

    #[test]
    fn parse_http_access() {
        let input = r#"
acl localnet src 10.0.0.0/8
acl blocked dstdomain .evil.com
http_access deny blocked
http_access allow localnet
http_access deny all
"#;
        let config = parse(input).unwrap();
        assert_eq!(config.http_access.len(), 3);
        assert_eq!(config.http_access[0].action, AccessAction::Deny);
        assert_eq!(config.http_access[0].acl_refs[0].name, "blocked");
        assert!(!config.http_access[0].acl_refs[0].negated);
        assert_eq!(config.http_access[2].acl_refs[0].name, "all");
    }

    #[test]
    fn parse_negated_acl_refs() {
        let input = "http_access allow localnet !blocked\n";
        let config = parse(input).unwrap();
        let rule = &config.http_access[0];
        assert_eq!(rule.acl_refs.len(), 2);
        assert!(!rule.acl_refs[0].negated);
        assert!(rule.acl_refs[1].negated);
        assert_eq!(rule.acl_refs[1].name, "blocked");
    }

    #[test]
    fn parse_auth_param() {
        let input = r#"
auth_param basic program /usr/lib/squid/basic_ncsa_auth /etc/squid/passwords
auth_param basic children 5
auth_param basic realm Squid Proxy Auth
auth_param basic credentialsttl 2 hours
"#;
        let config = parse(input).unwrap();
        assert_eq!(
            config.auth_param.program.as_deref(),
            Some("/usr/lib/squid/basic_ncsa_auth /etc/squid/passwords")
        );
        assert_eq!(config.auth_param.children.as_deref(), Some("5"));
        assert_eq!(config.auth_param.realm.as_deref(), Some("Squid Proxy Auth"));
        assert_eq!(config.auth_param.credentialsttl.as_deref(), Some("2 hours"));
    }

    #[test]
    fn parse_direct_rules() {
        let input = r#"
acl local dst 192.168.0.0/16
always_direct allow local
never_direct allow !local
"#;
        let config = parse(input).unwrap();
        assert_eq!(config.always_direct.len(), 1);
        assert_eq!(config.always_direct[0].action, AccessAction::Allow);
        assert_eq!(config.always_direct[0].acl_refs[0].name, "local");
        assert_eq!(config.never_direct.len(), 1);
        assert!(config.never_direct[0].acl_refs[0].negated);
    }

    #[test]
    fn parse_comments_skipped() {
        let input = r#"
# This is a comment
acl foo src 1.2.3.4
  # Indented comment
http_access allow foo
"#;
        let config = parse(input).unwrap();
        assert_eq!(config.acls.len(), 1);
        assert_eq!(config.http_access.len(), 1);
    }

    #[test]
    fn parse_type_mismatch_error() {
        let input = "acl foo src 1.2.3.4\nacl foo dst 5.6.7.8\n";
        let result = parse(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.message.contains("different type"));
    }
}
