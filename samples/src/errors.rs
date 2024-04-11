fn fallible() -> Result<u32, String> {
    // Ok(0)
    Err("oh no".to_owned())
}

fn infallible(text: &str) -> usize {
    if text.len() > 10 {
        panic!("out of memory for ginormous string");
    }
    text.len()
}

fn deal_with_it() -> Result<String, String> {
    match fallible() {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(e),  // "if err != nil return 0, err"
    }
}

fn deal_with_it_the_easy_way() -> Result<String, String> {
    let value = fallible()?;
    Ok(value.to_string())
}

fn what_could_go_wrong() {
    // std::fs::create_dir("fancy-schmancy");
}
