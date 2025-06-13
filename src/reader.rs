/// Reads line by line returning the lines that will be processed, or an error
pub fn store_valid_lines(code: &str) -> Result<Vec<String>, String>{
    // Stores lines that might be important
    let mut valid_lines: Vec<String> = Vec::new();
    for (line_num, line) in code.lines().enumerate(){
        let line = line.trim();
        if is_line_valid(line_num as u16, &line)?{
            valid_lines.push(line.to_string());
        }
    }
    return Ok(valid_lines);
}

fn is_line_valid(line_num: u16, line: &str) -> Result<bool, String>{
    // Ignore empty lines or comments
    if line.is_empty() || line.starts_with("//"){
        Ok(false)
    }
    // Accept only valid declarations, storings or placements ending with ';'
    else if (line.starts_with("let") || line.starts_with("store") || line.starts_with("place")) && line.ends_with(';'){
        Ok(true)
    }
    // Otherwise, there's a syntax error
    else{
        Err(format!("Syntax error, on line {}: {}", line_num + 1, line))
    }
}