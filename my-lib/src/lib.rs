pub fn echo(message: &str) -> Result<String, String> {
    if message == "foobar" {
        return Err(String::from(message));
    }

    Ok(String::from(message))
}
