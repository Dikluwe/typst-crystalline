fn decode_url_path(raw: &str) -> Option<String> {
    let raw = raw.split('?').next()?;
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hi = (bytes.get(index + 1)? as &u8).to_ascii_lowercase();
            let lo = (bytes.get(index + 2)? as &u8).to_ascii_lowercase();
            let digit = |value: u8| match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                _ => None,
            };
            decoded.push(digit(hi)? * 16 + digit(lo)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    let decoded = String::from_utf8(decoded).ok()?;
    if !decoded.starts_with('/') || decoded.contains('\\') || decoded.contains('\0') {
        return None;
    }
    let mut parts = Vec::new();
    for part in decoded.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    let mut path = format!("/{}", parts.join("/"));
    if decoded.ends_with('/') && path != "/" {
        path.push('/');
    }
    Some(path)
}

fn candidate_decode_url_path(raw: &str) -> Option<String> {
    let raw = raw.split('?').next()?;
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hi = (bytes.get(index + 1)? as &u8).to_ascii_lowercase();
            let lo = (bytes.get(index + 2)? as &u8).to_ascii_lowercase();
            let digit = |value: u8| char::from(value).to_digit(16).map(|digit| digit as u8);
            decoded.push(digit(hi)? * 16 + digit(lo)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    let decoded = String::from_utf8(decoded).ok()?;
    if !decoded.starts_with('/') || decoded.contains('\\') || decoded.contains('\0') {
        return None;
    }
    let mut parts = Vec::new();
    for part in decoded.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    let mut path = format!("/{}", parts.join("/"));
    if decoded.ends_with('/') && path != "/" {
        path.push('/');
    }
    Some(path)
}


fn main() {
    let original_digit = |value: u8| match value.to_ascii_lowercase() {
        b'0'..=b'9' => Some(value.to_ascii_lowercase() - b'0'),
        b'a'..=b'f' => Some(value.to_ascii_lowercase() - b'a' + 10),
        _ => None,
    };
    let candidate_digit = |value: u8| char::from(value.to_ascii_lowercase()).to_digit(16).map(|digit| digit as u8);
    let mut digits=0;
    for value in 0..=255u8 {assert_eq!(original_digit(value),candidate_digit(value));if original_digit(value).is_some(){digits+=1;}}
    let mut pairs=0;let mut accepted=0;
    for hi in 0..=255u8 {for lo in 0..=255u8 {
        let a=original_digit(hi).and_then(|h|original_digit(lo).map(|l|h*16+l));
        let b=candidate_digit(hi).and_then(|h|candidate_digit(lo).map(|l|h*16+l));
        assert_eq!(a,b);if a.is_some(){accepted+=1;}
        let path=format!("/a%{}{}z",char::from(hi),char::from(lo));
        assert_eq!(decode_url_path(&path),candidate_decode_url_path(&path));pairs+=1;
    }}
    let controls=[("/caf%C3%A9",Some("/café")),("/a%2Fb",Some("/a/b")),("/a/b/../c?d=1",Some("/a/c")),("/%2e%2e/private",None),("/x/%2E%2e/y",Some("/y")),("/%",None),("/%A",None),("/%GG",None),("/%00",None),("/%5c",None),("/%C0%AF",None),("/%FF",None),("/a/",Some("/a/"))];
    for (input,want) in controls {assert_eq!(decode_url_path(input).as_deref(),want);assert_eq!(candidate_decode_url_path(input).as_deref(),want);}
    assert_eq!(digits,22);assert_eq!(accepted,484);
    println!("{{\"bytes_compared\":256,\"accepted_bytes\":{digits},\"pairs_compared\":{pairs},\"accepted_pairs\":{accepted},\"route_controls\":{}}}",controls.len());
}
