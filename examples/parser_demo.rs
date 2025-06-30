use bytestr::ByteStr;

/// A simple HTTP request parser example demonstrating how to use ByteStr's convenience methods for zero-copy parsing
fn main() {
    // Simulate HTTP request
    let request = ByteStr::from(
        "GET /api/users?name=john&age=25 HTTP/1.1\r\n\
         Host: example.com\r\n\
         User-Agent: Mozilla/5.0\r\n\
         Accept: application/json\r\n\
         \r\n",
    );

    println!("=== HTTP Request Parsing Example ===");

    // Parse request line
    let (request_line, headers_and_body) = request.split_once("\r\n").unwrap();
    println!("Request line: {}", request_line.as_str());

    // Parse method, path and version
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path_and_query = parts.next().unwrap();
    let version = parts.next().unwrap();

    println!("  Method: {}", method.as_str());
    println!("  HTTP Version: {}", version.as_str());

    // Parse path and query parameters
    if let Some((path, query)) = path_and_query.split_once("?") {
        println!("  Path: {}", path.as_str());
        println!("  Query Parameters:");

        // Parse query parameters
        for param in query.split("&") {
            if let Some((key, value)) = param.split_once("=") {
                println!("    {} = {}", key.as_str(), value.as_str());
            }
        }
    } else {
        println!("  Path: {}", path_and_query.as_str());
    }

    // Parse headers
    let (headers_section, _body) = headers_and_body.split_once("\r\n\r\n").unwrap();
    println!("  Headers:");

    for header_line in headers_section.lines() {
        if let Some((name, value)) = header_line.split_once(": ") {
            println!("    {}: {}", name.as_str(), value.trim().as_str());
        }
    }

    println!("\n=== Configuration File Parsing Example ===");

    // Configuration file parsing example
    let config = ByteStr::from(
        "# Server configuration\n\
         port=8080\n\
         host=localhost\n\
         debug=true\n\
         # Database configuration\n\
         db.host=127.0.0.1\n\
         db.port=5432\n",
    );

    for line in config.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.as_str().starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once("=") {
            println!("Config: {} = {}", key.as_str(), value.as_str());
        }
    }

    println!("\n=== Data Parsing Example ===");

    // Parse CSV-like data
    let csv_data = ByteStr::from("name,age,city\nJohn,25,New York\nJane,30,London\n");

    let mut lines_iter = csv_data.lines();
    let header = lines_iter.next().unwrap();

    println!("CSV Header:");
    for (i, column) in header.split(",").enumerate() {
        println!("  Column {}: {}", i + 1, column.as_str());
    }

    println!("CSV Data:");
    for (row_num, line) in lines_iter.enumerate() {
        println!("  Row {}:", row_num + 1);
        for (i, value) in line.split(",").enumerate() {
            println!("    Column {}: {}", i + 1, value.as_str());
        }
    }

    println!("\n=== Lexical Analysis Example ===");

    // Simple lexical analysis
    let code = ByteStr::from("let x = 42; // a variable");
    let mut remaining = code;

    while !remaining.is_empty() {
        // Skip whitespace characters
        remaining = remaining.skip_while(|c| c.is_whitespace());

        if remaining.is_empty() {
            break;
        }

        // Check for comments
        if remaining.as_str().starts_with("//") {
            let comment = remaining.take_until("\n");
            println!("Comment: {}", comment.as_str());
            break;
        }

        // Parse identifiers
        if remaining.as_str().chars().next().unwrap().is_alphabetic() {
            let (identifier, rest) = remaining.take_while(|c| c.is_alphanumeric() || c == '_');
            println!("Identifier: {}", identifier.as_str());
            remaining = rest;
        }
        // Parse numbers
        else if remaining.as_str().chars().next().unwrap().is_ascii_digit() {
            let (number, rest) = remaining.take_while(|c| c.is_ascii_digit());
            println!("Number: {}", number.as_str());
            remaining = rest;
        }
        // Parse operators
        else {
            let (operator, rest) = remaining.take_while(|c| "=;".contains(c));
            if !operator.is_empty() {
                println!("Operator: {}", operator.as_str());
                remaining = rest;
            } else {
                // Skip one character
                remaining = remaining.skip(1);
            }
        }
    }
}
