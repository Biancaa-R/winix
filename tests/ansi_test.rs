use winix::ansi::{AnsiEvent, AnsiParser}; // Adjust based on your module path

#[test]
fn test_ls_color_output_parses_color_events() {
    // Create ANSI output directly instead of relying on external commands
    let ansi_output = "\x1b[31mHello\x1b[0m"; // Red "Hello"

    // Parse the ANSI output
    let events = AnsiParser::parse(ansi_output.as_bytes());

    // Check if SetColor was detected
    let has_color_event = events.iter().any(|e| matches!(e, AnsiEvent::SetColor(_)));

    assert!(
        has_color_event,
        "Expected at least one SetColor event in ANSI output"
    );
}
