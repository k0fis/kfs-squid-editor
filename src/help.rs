use crate::model::AclType;

pub fn help_for_acl_type(acl_type: &AclType) -> &'static str {
    match acl_type {
        AclType::Src => {
            "Match client source IP addresses.\nValues: IP, CIDR (10.0.0.0/8), or ranges.\nExample: 192.168.1.0/24"
        }
        AclType::Dst => {
            "Match destination server IP addresses.\nValues: IP, CIDR notation.\nExample: 172.16.0.0/12"
        }
        AclType::SrcDomain => {
            "Match client source domain (reverse DNS).\nValues: domain names, prefix with . for subdomains.\nExample: .internal.corp"
        }
        AclType::DstDomain => {
            "Match destination domain from the URL.\nValues: domain names, prefix with . for subdomains.\nExample: .example.com"
        }
        AclType::SrcDomRegex => {
            "Match client domain using regex.\nValues: regular expressions.\nExample: ^client.*\\.corp$"
        }
        AclType::DstDomRegex => {
            "Match destination domain using regex.\nValues: regular expressions.\nExample: .*\\.ads\\..*"
        }
        AclType::UrlRegex => {
            "Match the full request URL using regex.\nValues: regular expressions (case-sensitive by default, use -i flag).\nExample: ^https://.*\\.exe$"
        }
        AclType::UrlPathRegex => {
            "Match only the URL path (not domain) using regex.\nValues: regular expressions.\nExample: /downloads/.*\\.zip$"
        }
        AclType::Port => {
            "Match destination port number.\nValues: port numbers or ranges (hyphen-separated).\nExample: 443 8080 1025-65535"
        }
        AclType::Proto => "Match request protocol.\nValues: http, ftp, https.\nExample: http ftp",
        AclType::Method => {
            "Match HTTP request method.\nValues: GET, POST, PUT, DELETE, CONNECT, etc.\nExample: CONNECT POST"
        }
        AclType::Time => {
            "Match requests by day and time.\nFormat: MTWHFAS HH:MM-HH:MM\nDays: M=Mon T=Tue W=Wed H=Thu F=Fri A=Sat S=Sun\nExample: MTWHF 08:00-17:00"
        }
        AclType::Arp => {
            "Match client MAC/ethernet address.\nValues: MAC addresses in XX:XX:XX:XX:XX:XX format.\nNote: Only works on local network (Linux/BSD).\nExample: 00:11:22:33:44:55"
        }
        AclType::MyIp => {
            "Match the local IP address the client connected to.\nValues: IP addresses.\nExample: 192.168.1.1"
        }
        AclType::MyPort => {
            "Match the local port the client connected to.\nValues: port numbers.\nExample: 3128 8080"
        }
        AclType::ProxyAuth => {
            "Match authenticated username.\nValues: usernames, or REQUIRED to force authentication.\nRequires auth_param configuration.\nExample: REQUIRED"
        }
        AclType::Browser => {
            "Match User-Agent header using regex.\nValues: regular expressions (case-sensitive).\nExample: Mozilla/.*Firefox"
        }
        AclType::RefererRegex => {
            "Match HTTP Referer header using regex.\nValues: regular expressions.\nExample: ^https://suspicious\\.site/"
        }
        AclType::ReqMimeType => {
            "Match request Content-Type (for uploads).\nValues: MIME type patterns.\nExample: application/x-executable"
        }
        AclType::RepMimeType => {
            "Match response Content-Type.\nValues: MIME type patterns.\nExample: video/mp4 audio/mpeg"
        }
        AclType::MaxConn => {
            "Match when client exceeds N connections.\nValues: connection count (single number).\nExample: 10"
        }
        AclType::External => {
            "Match using an external ACL helper program.\nValues: helper class name and arguments.\nExample: my_helper %SRC"
        }
        AclType::Snmp => {
            "Match SNMP community string.\nValues: community strings.\nExample: public"
        }
    }
}

pub fn help_for_screen(screen: &str) -> &'static str {
    match screen {
        "acl_list" => concat!(
            "ACCESS CONTROL LISTS (ACLs)\n",
            "\n",
            "ACLs define conditions that Squid evaluates for each request.\n",
            "Each ACL has a name, a type, and one or more values.\n",
            "Multiple values within one ACL use OR logic (any match).\n",
            "\n",
            "Keys:\n",
            "  a     - Add new ACL\n",
            "  e/Enter - Edit selected ACL\n",
            "  d     - Delete selected ACL\n",
            "  j/k   - Move selection up/down\n",
            "  Tab   - Next section\n",
        ),
        "access_list" => concat!(
            "HTTP ACCESS RULES\n",
            "\n",
            "Rules are evaluated in order for each request.\n",
            "Each rule combines one or more ACLs with AND logic.\n",
            "Processing stops at the first matching rule.\n",
            "Negation (!) reverses an ACL match.\n",
            "\n",
            "Always end with 'deny all' as a safety fallback.\n",
            "\n",
            "Keys:\n",
            "  a     - Add new rule\n",
            "  e/Enter - Edit selected rule\n",
            "  d     - Delete selected rule\n",
            "  u/J   - Move rule up/down (order matters!)\n",
            "  j/k   - Move selection up/down\n",
            "  Tab   - Next section\n",
        ),
        "auth" => concat!(
            "AUTHENTICATION (auth_param basic)\n",
            "\n",
            "Configures HTTP Basic authentication for the proxy.\n",
            "Requires an external authenticator program (e.g. basic_ncsa_auth).\n",
            "\n",
            "Fields:\n",
            "  Program       - Path to authenticator binary + args\n",
            "  Children      - Number of authenticator helper processes\n",
            "  Realm         - Text shown in the browser auth dialog\n",
            "  Credentials TTL - How long to cache valid credentials\n",
            "\n",
            "To use: create a 'proxy_auth REQUIRED' ACL and reference\n",
            "it in http_access rules.\n",
            "\n",
            "Keys:\n",
            "  Tab     - Next field\n",
            "  Ctrl+s  - Save changes\n",
            "  Esc     - Cancel\n",
        ),
        "direct" => concat!(
            "DIRECT ROUTING (always_direct / never_direct)\n",
            "\n",
            "Controls whether Squid connects directly to origin servers\n",
            "or forwards requests through cache peers (parent proxies).\n",
            "\n",
            "always_direct allow <acl> - Go direct for matching requests\n",
            "never_direct allow <acl>  - Always use a peer for matching\n",
            "\n",
            "Note: 'always_direct deny X' is NOT the same as\n",
            "'never_direct allow X'.\n",
            "\n",
            "Keys:\n",
            "  a     - Add new rule\n",
            "  e/Enter - Edit selected rule\n",
            "  d     - Delete selected rule\n",
            "  j/k   - Move selection up/down\n",
            "  Tab   - Switch between always/never sections\n",
        ),
        _ => "No help available for this screen.",
    }
}
