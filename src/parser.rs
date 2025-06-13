use std::collections::HashMap;

/// Processes the lines and returns the HTML code in a String, or an error
pub fn parse_lines(input_lines: Vec<String>) -> Result<String, String>{
    let mut html_code =  Vec::new();
    // Maps values by it's name (id)
    let mut stored_elements: HashMap<String, String> = HashMap::new();
    html_code.push(String::from("<!DOCTYPE html>\n<html>\n<head><meta charset=\"UTF-8\"></head>\n<body>"));
    pushing_converted_code(input_lines, &mut html_code, &mut stored_elements)?;
    html_code.push(String::from("</body>\n</html>"));
    Ok(html_code.join("\n"))
}

fn pushing_converted_code(input_lines: Vec<String>, html_code: &mut Vec<String>, stored_elements: &mut HashMap<String, String>) -> Result<(), String>{
    for (line_num, line) in input_lines.into_iter().enumerate(){
        if line.starts_with("let"){
            // Element will be stored and placed
            let (decl_type, decl_name, value) = declaration_tokens(line_num as u16, &line, &stored_elements)?;
            if stored_elements.contains_key(decl_name){
                return Err(format!("Multiple elements with same name, on line {}: {}", line_num+1, line));
            }
            html_code.push(convert_declaration_to_html(decl_type, decl_name, &value, line_num as u16, &line)?);
            stored_elements.insert(decl_name.to_string(), value.to_string());
        }
        else if line.starts_with("store"){
            // Element will be only stored
            let (store_name, value) = storing_tokens(line_num as u16, &line, &stored_elements)?;
            if stored_elements.contains_key(store_name){
                return Err(format!("Multiple elements with same name, on line {}: {}", line_num+1, line));
            }
            stored_elements.insert(store_name.to_string(), value.to_string());
        }
        else if line.starts_with("place"){
            // Element will be only placed
            let (place_type, value) = placement_tokens(line_num as u16, &line, &stored_elements)?;
            html_code.push(convert_placement_to_html(place_type, &value, line_num as u16, &line)?);
        }
    }
    Ok(())
}

fn split_line(line_num: u16, line: &str) -> Result<(&str, &str), String>{
    return match line.split_once('='){
        Some((left, right)) => Ok((left, right)),
        _ => Err(format!("Failed to split line {}: {}", line_num, &line)),
    }
}

fn resolve_value(right: &str, line_num: u16, line: &str, stored_elements: &HashMap<String, String>) -> Result<String, String>{
    // Checking if it's a raw string
    if right.contains('"'){
        return Ok(right.trim_matches('"').to_string());
    }
    // If it's a reference to a stored element
    else{
        return match stored_elements.get(right).cloned(){
            Some(value) => Ok(value),
            None => Err(format!("Failed to find value of element on line {}: {}", line_num+1, line)),
        };
    }
}

fn extract_tokens<'a>(expected_tokens: usize, line_num: u16, line: &'a str, stored_elements: &'a HashMap<String, String>) -> Result<(Vec<&'a str>, String), String>{
    let (left, mut right) = split_line(line_num, line)?;
    let left_tokens: Vec<&str> = left.split_whitespace().collect();
    // Checking the number of tokens on the left side
    if left_tokens.len() < expected_tokens{
        return Err(format!("Line {} is missing tokens: {}", line_num + 1, line));
    }
    else if left_tokens.len() > expected_tokens{
        return Err(format!("Line {} has too many tokens: {}", line_num + 1, line));
    }
    right = right.trim_end_matches(';').trim();
    let value = resolve_value(right, line_num, line, stored_elements)?;
    // Returns a tuple with information about the element and its value
    Ok((left_tokens, value))
}

fn placement_tokens<'a>(line_num: u16, line: &'a str, stored_elements: &'a HashMap<String, String>) -> Result<(&'a str, String), String>{
    let (tokens, value) = extract_tokens(2, line_num, line, stored_elements)?;
    // Placements only take type and value
    Ok((tokens[1], value))
}

fn storing_tokens<'a>(line_num: u16, line: &'a str, stored_elements: &'a HashMap<String, String>) -> Result<(&'a str, String), String>{
    let (tokens, value) = extract_tokens(2, line_num, line, stored_elements)?;
    // Storings only take name and value
    Ok((tokens[1], value))
}

fn declaration_tokens<'a>(line_num: u16, line: &'a str, stored_elements: &'a HashMap<String, String>) -> Result<(&'a str, &'a str, String), String>{
    let (tokens, value) = extract_tokens(3, line_num, line, stored_elements)?;
    // Declarations take type, name and value
    Ok((tokens[1], tokens[2], value))
}

fn convert_declaration_to_html(decl_type: &str, decl_name: &str, value: &str, line_num: u16, line: &str) -> Result<String, String>{
    // Check if type contains only a tag
    if !decl_type.contains('.'){
        let type_converted = type_conversion(decl_type, line_num, line)?;
        Ok(format!("<{type_converted} id=\"{decl_name}\">{value}</{type_converted}>"))
    }
    // Else, it also contains a class
    else{
        let (tag, class) = analyze_class(decl_type, line_num, line)?;
        let type_converted = type_conversion(tag, line_num, line)?;
        Ok(format!("<{type_converted} class=\"{class}\" id=\"{decl_name}\">{value}</{type_converted}>"))
    }
}

fn convert_placement_to_html(place_type: &str, value: &str, line_num: u16, line: &str) -> Result<String, String>{
    // Checking if type contains only a tag
    if !place_type.contains('.'){
        let type_converted = type_conversion(place_type, line_num, line)?;
        Ok(format!("<{type_converted}>{value}</{type_converted}>"))
    }
    // Else, it also contains a class
    else{
        let (tag, class) = analyze_class(place_type, line_num, line)?;
        let type_converted = type_conversion(tag, line_num, line)?;
        Ok(format!("<{type_converted} class=\"{class}\">{value}</{type_converted}>"))
    } 
}

fn analyze_class<'a>(input_type: &'a str, line_num: u16, line: &'a str) -> Result<(&'a str, &'a str), String>{
    match input_type.split_once('.'){
        Some((t, c)) => Ok((t, c)),
        _ => Err(format!("Failed to convert declaration, on line {}: {line}\nMake sure you are declaring with 'tag.class' format.", line_num+1)),
    }
}

fn type_conversion(input_type: &str, line_num: u16, line: &str) -> Result<String, String>{
    // Match LMCL code tag with its equivalent in HTML
    let type_converted = match input_type{
        "paragraph" | "p" => "p",
        "title" | "h1" => "h1",
        "subtitle" | "h2" => "h2",
        "h3" => "h3",
        "h4" => "h4",
        "h5" => "h5",
        "h6" => "h6",
        "div" => "div",

        _ => return Err(format!("Line {} uses non-existent tag: {line}", line_num + 1)),
    };
    Ok(format!("{}", type_converted))
}