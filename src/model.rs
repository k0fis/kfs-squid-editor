use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AclType {
    Src,
    Dst,
    SrcDomain,
    DstDomain,
    SrcDomRegex,
    DstDomRegex,
    UrlRegex,
    UrlPathRegex,
    Port,
    Proto,
    Method,
    Time,
    Arp,
    MyIp,
    MyPort,
    ProxyAuth,
    Browser,
    RefererRegex,
    ReqMimeType,
    RepMimeType,
    MaxConn,
    External,
    Snmp,
}

impl fmt::Display for AclType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Src => "src",
            Self::Dst => "dst",
            Self::SrcDomain => "srcdomain",
            Self::DstDomain => "dstdomain",
            Self::SrcDomRegex => "srcdom_regex",
            Self::DstDomRegex => "dstdom_regex",
            Self::UrlRegex => "url_regex",
            Self::UrlPathRegex => "urlpath_regex",
            Self::Port => "port",
            Self::Proto => "proto",
            Self::Method => "method",
            Self::Time => "time",
            Self::Arp => "arp",
            Self::MyIp => "myip",
            Self::MyPort => "myport",
            Self::ProxyAuth => "proxy_auth",
            Self::Browser => "browser",
            Self::RefererRegex => "referer_regex",
            Self::ReqMimeType => "req_mime_type",
            Self::RepMimeType => "rep_mime_type",
            Self::MaxConn => "maxconn",
            Self::External => "external",
            Self::Snmp => "snmp_community",
        };
        write!(f, "{s}")
    }
}

impl FromStr for AclType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "src" => Ok(Self::Src),
            "dst" => Ok(Self::Dst),
            "srcdomain" => Ok(Self::SrcDomain),
            "dstdomain" => Ok(Self::DstDomain),
            "srcdom_regex" => Ok(Self::SrcDomRegex),
            "dstdom_regex" => Ok(Self::DstDomRegex),
            "url_regex" => Ok(Self::UrlRegex),
            "urlpath_regex" => Ok(Self::UrlPathRegex),
            "port" => Ok(Self::Port),
            "proto" => Ok(Self::Proto),
            "method" => Ok(Self::Method),
            "time" => Ok(Self::Time),
            "arp" => Ok(Self::Arp),
            "myip" => Ok(Self::MyIp),
            "myport" => Ok(Self::MyPort),
            "proxy_auth" => Ok(Self::ProxyAuth),
            "browser" => Ok(Self::Browser),
            "referer_regex" => Ok(Self::RefererRegex),
            "req_mime_type" => Ok(Self::ReqMimeType),
            "rep_mime_type" => Ok(Self::RepMimeType),
            "maxconn" => Ok(Self::MaxConn),
            "external" => Ok(Self::External),
            "snmp_community" => Ok(Self::Snmp),
            _ => Err(format!("unknown ACL type: {s}")),
        }
    }
}

impl AclType {
    pub const ALL: &'static [AclType] = &[
        Self::Src,
        Self::Dst,
        Self::SrcDomain,
        Self::DstDomain,
        Self::SrcDomRegex,
        Self::DstDomRegex,
        Self::UrlRegex,
        Self::UrlPathRegex,
        Self::Port,
        Self::Proto,
        Self::Method,
        Self::Time,
        Self::Arp,
        Self::MyIp,
        Self::MyPort,
        Self::ProxyAuth,
        Self::Browser,
        Self::RefererRegex,
        Self::ReqMimeType,
        Self::RepMimeType,
        Self::MaxConn,
        Self::External,
        Self::Snmp,
    ];
}

#[derive(Clone, Debug)]
pub struct Acl {
    pub name: String,
    pub acl_type: AclType,
    pub case_insensitive: bool,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessAction {
    Allow,
    Deny,
}

impl fmt::Display for AccessAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Deny => write!(f, "deny"),
        }
    }
}

impl FromStr for AccessAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err(format!("expected 'allow' or 'deny', got: {s}")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AclRef {
    pub negated: bool,
    pub name: String,
}

impl fmt::Display for AclRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negated {
            write!(f, "!{}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Clone, Debug)]
pub struct AccessRule {
    pub action: AccessAction,
    pub acl_refs: Vec<AclRef>,
}

#[derive(Clone, Debug)]
pub struct DirectRule {
    pub action: AccessAction,
    pub acl_refs: Vec<AclRef>,
}

#[derive(Clone, Debug, Default)]
pub struct AuthParamBasic {
    pub program: Option<String>,
    pub children: Option<String>,
    pub realm: Option<String>,
    pub credentialsttl: Option<String>,
}

impl AuthParamBasic {
    pub fn is_empty(&self) -> bool {
        self.program.is_none()
            && self.children.is_none()
            && self.realm.is_none()
            && self.credentialsttl.is_none()
    }
}

#[derive(Clone, Debug, Default)]
pub struct SquidConfig {
    pub acls: Vec<Acl>,
    pub http_access: Vec<AccessRule>,
    pub auth_param: AuthParamBasic,
    pub always_direct: Vec<DirectRule>,
    pub never_direct: Vec<DirectRule>,
}

pub const PREDEFINED_ACLS: &[&str] = &[
    "all",
    "manager",
    "localhost",
    "to_localhost",
    "to_linklocal",
    "CONNECT",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acl_type_roundtrip() {
        for t in AclType::ALL {
            let s = t.to_string();
            let parsed: AclType = s.parse().unwrap();
            assert_eq!(&parsed, t);
        }
    }

    #[test]
    fn acl_ref_display() {
        let r = AclRef {
            negated: false,
            name: "foo".into(),
        };
        assert_eq!(r.to_string(), "foo");
        let r = AclRef {
            negated: true,
            name: "bar".into(),
        };
        assert_eq!(r.to_string(), "!bar");
    }

    #[test]
    fn access_action_roundtrip() {
        assert_eq!(
            "allow".parse::<AccessAction>().unwrap(),
            AccessAction::Allow
        );
        assert_eq!("Deny".parse::<AccessAction>().unwrap(), AccessAction::Deny);
    }
}
