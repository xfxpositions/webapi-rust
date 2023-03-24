use crate::types::http_methods::HttpMethod;

pub fn parse_headers(
    request_string: String,
) -> Result<
    (
        (String, String, String),
        Vec<(String, String)>,
        String,
        String,
    ),
    String,
> {
    let mut parts = request_string.splitn(2, "\r\n\r\n");
    let headers_str = parts.next().ok_or("")?;
    let body_str = parts.next().unwrap_or("");

    let mut headers: Vec<(String, String)> = vec![];
    let mut first_line = None;

    for (i, line) in headers_str.lines().enumerate() {
        if i == 0 {
            first_line = Some(line.to_owned());
            continue;
        }
        let mut parts = line.splitn(2, ": ");
        let key = parts
            .next()
            .ok_or(format!("Invalid header: {}", line))?
            .to_owned();
        let value = parts
            .next()
            .ok_or(format!("Invalid header: {}", line))?
            .to_owned();
        headers.push((key, value));
    }
    let first_line = first_line.ok_or("Invalid request: no first line found")?;
    let mut first_line_parts = first_line.split_whitespace();

    let method = first_line_parts
        .next()
        .ok_or("Invalid request: no method found")?
        .to_owned();
    let path = first_line_parts
        .next()
        .ok_or("Invalid request: no path found")?
        .to_owned();
    let http_version = first_line_parts
        .next()
        .ok_or("Invalid request: no HTTP version found")?
        .to_owned();

    println!("first_line: {}", first_line);
    println!("headers: {:?}", headers);
    println!("body: {}", body_str);

    println!(
        "http_method: {}, path:{}, version:{}",
        method, path, http_version
    );
    return Ok((
        (method.to_string(), path.to_string(), http_version),
        headers,
        headers_str.to_string(),
        body_str.to_string(),
    ));
}

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: HttpMethod,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub headers_raw: String,
    pub body: String,
    pub raw_string: String,
}
impl Request {
    pub fn new(request_string: String) -> Request {
        let (first_line, headers, headers_str, body) =
            parse_headers(request_string.to_string()).unwrap();
        return Request {
            method: HttpMethod::from_string(first_line.0.as_str()),
            path: first_line.1,
            version: first_line.2,
            headers: headers,
            headers_raw: headers_str,
            body: body,
            raw_string: request_string,
        };
    }
    
}
