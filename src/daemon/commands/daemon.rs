pub fn handle(
    command: &str,
    _argument: Option<&str>,
    shutdown: &mut bool,
) -> Result<String, String> {
    match command {
        "stop" => {
            *shutdown = true;
            Ok("Daemon shutting down.".into())
        }

        _ => Err("Unknown daemon command.".into()),
    }
}
