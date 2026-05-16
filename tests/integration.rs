use lineify::Lineifier;

#[test]
fn buffers_until_newline() {
    let mut l = Lineifier::new();
    let r: Vec<String> = vec![];
    assert_eq!(l.push("hel"), r);
    assert!(l.pending() > 0);
    assert_eq!(l.push("lo\n"), vec!["hello".to_string()]);
}

#[test]
fn multiple_lines_one_push() {
    let mut l = Lineifier::new();
    assert_eq!(
        l.push("one\ntwo\nthree\n"),
        vec!["one".to_string(), "two".to_string(), "three".to_string()]
    );
}

#[test]
fn crlf_stripped() {
    let mut l = Lineifier::new();
    assert_eq!(l.push("hello\r\nworld\r\n"), vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn flush_returns_partial_line() {
    let mut l = Lineifier::new();
    l.push("tail");
    assert_eq!(l.flush(), "tail");
    assert_eq!(l.pending(), 0);
}

#[test]
fn empty_chunk_is_safe() {
    let mut l = Lineifier::new();
    let r: Vec<String> = vec![];
    assert_eq!(l.push(""), r);
}

#[test]
fn empty_line_is_emitted_as_empty_string() {
    let mut l = Lineifier::new();
    assert_eq!(l.push("\n"), vec!["".to_string()]);
}
