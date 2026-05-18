use crate::model::AclType;

pub fn validate_acl_value(acl_type: &AclType, value: &str) -> Result<(), String> {
    match acl_type {
        AclType::Src | AclType::Dst | AclType::MyIp => validate_ip_or_cidr(value),
        AclType::Port | AclType::MyPort => validate_port(value),
        AclType::Time => validate_time(value),
        AclType::Arp => validate_mac(value),
        AclType::Proto => validate_proto(value),
        AclType::Method => validate_method(value),
        AclType::MaxConn => validate_number(value),
        _ => Ok(()),
    }
}

fn validate_ip_or_cidr(value: &str) -> Result<(), String> {
    if let Some((ip, mask)) = value.split_once('/') {
        validate_ip_addr(ip)?;
        let bits: u8 = mask
            .parse()
            .map_err(|_| format!("invalid CIDR mask: {mask}"))?;
        if bits > 128 {
            return Err(format!("CIDR mask too large: {bits}"));
        }
    } else if value.contains('-') {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 2 {
            return Err(format!("invalid IP range: {value}"));
        }
        validate_ip_addr(parts[0])?;
        validate_ip_addr(parts[1])?;
    } else {
        validate_ip_addr(value)?;
    }
    Ok(())
}

fn validate_ip_addr(ip: &str) -> Result<(), String> {
    if ip.contains(':') {
        // IPv6
        ip.parse::<std::net::Ipv6Addr>()
            .map_err(|_| format!("invalid IPv6 address: {ip}"))?;
    } else {
        ip.parse::<std::net::Ipv4Addr>()
            .map_err(|_| format!("invalid IP address: {ip}"))?;
    }
    Ok(())
}

fn validate_port(value: &str) -> Result<(), String> {
    if let Some((start, end)) = value.split_once('-') {
        let s: u16 = start
            .parse()
            .map_err(|_| format!("invalid port: {start}"))?;
        let e: u16 = end.parse().map_err(|_| format!("invalid port: {end}"))?;
        if s > e {
            return Err(format!("port range reversed: {s}-{e}"));
        }
    } else {
        value
            .parse::<u16>()
            .map_err(|_| format!("invalid port: {value}"))?;
    }
    Ok(())
}

fn validate_time(value: &str) -> Result<(), String> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("time format: DAYS HH:MM-HH:MM".into());
    }
    let valid_days = "MTWHFAS";
    for c in parts[0].chars() {
        if !valid_days.contains(c) {
            return Err(format!("invalid day char: {c} (use MTWHFAS)"));
        }
    }
    if let Some((start, end)) = parts[1].split_once('-') {
        validate_hhmm(start)?;
        validate_hhmm(end)?;
    } else {
        return Err("time range format: HH:MM-HH:MM".into());
    }
    Ok(())
}

fn validate_hhmm(s: &str) -> Result<(), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("invalid time: {s}"));
    }
    let h: u8 = parts[0]
        .parse()
        .map_err(|_| format!("invalid hour: {}", parts[0]))?;
    let m: u8 = parts[1]
        .parse()
        .map_err(|_| format!("invalid minute: {}", parts[1]))?;
    if h > 23 {
        return Err(format!("hour out of range: {h}"));
    }
    if m > 59 {
        return Err(format!("minute out of range: {m}"));
    }
    Ok(())
}

fn validate_mac(value: &str) -> Result<(), String> {
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 6 {
        return Err("MAC address must have 6 octets (XX:XX:XX:XX:XX:XX)".into());
    }
    for part in parts {
        if part.len() != 2 || !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("invalid MAC octet: {part}"));
        }
    }
    Ok(())
}

fn validate_proto(value: &str) -> Result<(), String> {
    match value.to_lowercase().as_str() {
        "http" | "https" | "ftp" => Ok(()),
        _ => Err(format!("unknown protocol: {value} (use http/https/ftp)")),
    }
}

fn validate_method(value: &str) -> Result<(), String> {
    let valid = [
        "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];
    let upper = value.to_uppercase();
    if valid.contains(&upper.as_str()) {
        Ok(())
    } else {
        Err(format!("unknown HTTP method: {value}"))
    }
}

fn validate_number(value: &str) -> Result<(), String> {
    value
        .parse::<u32>()
        .map_err(|_| format!("must be a number: {value}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ip() {
        assert!(validate_acl_value(&AclType::Src, "192.168.1.0/24").is_ok());
        assert!(validate_acl_value(&AclType::Src, "10.0.0.1").is_ok());
        assert!(validate_acl_value(&AclType::Src, "10.0.0.1-10.0.0.255").is_ok());
    }

    #[test]
    fn invalid_ip() {
        assert!(validate_acl_value(&AclType::Src, "999.1.2.3").is_err());
        assert!(validate_acl_value(&AclType::Src, "abc").is_err());
    }

    #[test]
    fn valid_port() {
        assert!(validate_acl_value(&AclType::Port, "443").is_ok());
        assert!(validate_acl_value(&AclType::Port, "1024-65535").is_ok());
    }

    #[test]
    fn invalid_port() {
        assert!(validate_acl_value(&AclType::Port, "99999").is_err());
        assert!(validate_acl_value(&AclType::Port, "abc").is_err());
    }

    #[test]
    fn valid_time() {
        assert!(validate_acl_value(&AclType::Time, "MTWHF 08:00-17:00").is_ok());
    }

    #[test]
    fn invalid_time() {
        assert!(validate_acl_value(&AclType::Time, "XY 08:00-17:00").is_err());
        assert!(validate_acl_value(&AclType::Time, "MTWHF 25:00-17:00").is_err());
    }

    #[test]
    fn valid_mac() {
        assert!(validate_acl_value(&AclType::Arp, "00:11:22:33:44:55").is_ok());
    }

    #[test]
    fn invalid_mac() {
        assert!(validate_acl_value(&AclType::Arp, "00:11:22:33:44").is_err());
        assert!(validate_acl_value(&AclType::Arp, "ZZ:11:22:33:44:55").is_err());
    }

    #[test]
    fn domain_types_skip_validation() {
        assert!(validate_acl_value(&AclType::DstDomain, "anything.goes").is_ok());
        assert!(validate_acl_value(&AclType::UrlRegex, ".*\\.exe$").is_ok());
    }
}
