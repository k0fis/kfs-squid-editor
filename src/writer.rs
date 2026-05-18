use crate::model::*;
use std::fmt::Write;

pub fn write_config(config: &SquidConfig) -> String {
    let mut out = String::new();

    write_auth_param(&config.auth_param, &mut out);
    write_acls(&config.acls, &mut out);
    write_access_rules("http_access", &config.http_access, &mut out);
    write_direct_rules("always_direct", &config.always_direct, &mut out);
    write_direct_rules("never_direct", &config.never_direct, &mut out);

    out.trim_end().to_string() + "\n"
}

fn write_auth_param(auth: &AuthParamBasic, out: &mut String) {
    if auth.is_empty() {
        return;
    }

    if let Some(ref program) = auth.program {
        writeln!(out, "auth_param basic program {program}").unwrap();
    }
    if let Some(ref children) = auth.children {
        writeln!(out, "auth_param basic children {children}").unwrap();
    }
    if let Some(ref realm) = auth.realm {
        writeln!(out, "auth_param basic realm {realm}").unwrap();
    }
    if let Some(ref ttl) = auth.credentialsttl {
        writeln!(out, "auth_param basic credentialsttl {ttl}").unwrap();
    }
    out.push('\n');
}

fn write_acls(acls: &[Acl], out: &mut String) {
    if acls.is_empty() {
        return;
    }

    for acl in acls {
        if PREDEFINED_ACLS.contains(&acl.name.as_str()) {
            continue;
        }
        write!(out, "acl {} {}", acl.name, acl.acl_type).unwrap();
        if acl.case_insensitive {
            write!(out, " -i").unwrap();
        }
        for val in &acl.values {
            write!(out, " {val}").unwrap();
        }
        out.push('\n');
    }
    out.push('\n');
}

fn write_access_rules(directive: &str, rules: &[AccessRule], out: &mut String) {
    if rules.is_empty() {
        return;
    }

    for rule in rules {
        write!(out, "{directive} {}", rule.action).unwrap();
        for acl_ref in &rule.acl_refs {
            write!(out, " {acl_ref}").unwrap();
        }
        out.push('\n');
    }
    out.push('\n');
}

fn write_direct_rules(directive: &str, rules: &[DirectRule], out: &mut String) {
    if rules.is_empty() {
        return;
    }

    for rule in rules {
        write!(out, "{directive} {}", rule.action).unwrap();
        for acl_ref in &rule.acl_refs {
            write!(out, " {acl_ref}").unwrap();
        }
        out.push('\n');
    }
    out.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn roundtrip_basic() {
        let input = r#"auth_param basic program /usr/lib/squid/basic_ncsa_auth /etc/squid/passwords
auth_param basic children 5
auth_param basic realm Squid Proxy
auth_param basic credentialsttl 2 hours

acl localnet src 10.0.0.0/8 172.16.0.0/12 192.168.0.0/16
acl SSL_ports port 443
acl blocked dstdomain -i .facebook.com .twitter.com

http_access deny blocked
http_access allow localnet
http_access deny all

always_direct allow localnet

never_direct allow !localnet
"#;
        let config = parser::parse(input).unwrap();
        let output = write_config(&config);
        let reparsed = parser::parse(&output).unwrap();

        assert_eq!(config.acls.len(), reparsed.acls.len());
        assert_eq!(config.http_access.len(), reparsed.http_access.len());
        assert_eq!(config.always_direct.len(), reparsed.always_direct.len());
        assert_eq!(config.never_direct.len(), reparsed.never_direct.len());

        for (a, b) in config.acls.iter().zip(reparsed.acls.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.acl_type, b.acl_type);
            assert_eq!(a.values, b.values);
            assert_eq!(a.case_insensitive, b.case_insensitive);
        }
    }

    #[test]
    fn predefined_acls_not_written() {
        let mut config = SquidConfig::default();
        config.acls.push(Acl {
            name: "all".into(),
            acl_type: AclType::Src,
            case_insensitive: false,
            values: vec!["0.0.0.0/0".into()],
        });
        config.acls.push(Acl {
            name: "myacl".into(),
            acl_type: AclType::Src,
            case_insensitive: false,
            values: vec!["10.0.0.0/8".into()],
        });

        let output = write_config(&config);
        assert!(!output.contains("acl all"));
        assert!(output.contains("acl myacl"));
    }

    #[test]
    fn empty_config() {
        let config = SquidConfig::default();
        let output = write_config(&config);
        assert_eq!(output.trim(), "");
    }
}
