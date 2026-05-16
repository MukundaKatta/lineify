//! # lineify
//!
//! Turn a token-by-token stream into stable line events. Buffers
//! partial lines and emits complete ones as `\n` arrives.
//!
//! Use it to gate log lines, side effects, or persistence on whole
//! lines instead of partial tokens.
//!
//! ## Example
//!
//! ```
//! use lineify::Lineifier;
//! let mut l = Lineifier::new();
//! assert_eq!(l.push("hel"), vec![] as Vec<String>);
//! assert_eq!(l.push("lo\nwor"), vec!["hello".to_string()]);
//! assert_eq!(l.push("ld\n"), vec!["world".to_string()]);
//! assert_eq!(l.flush(), "" );
//! ```

#![deny(missing_docs)]

/// Streaming line-buffer.
#[derive(Debug, Default, Clone)]
pub struct Lineifier {
    buf: String,
}

impl Lineifier {
    /// Build an empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push next chunk. Returns every complete line produced.
    pub fn push(&mut self, chunk: &str) -> Vec<String> {
        self.buf.push_str(chunk);
        let mut out = Vec::new();
        while let Some(nl) = self.buf.find('\n') {
            let line: String = self.buf.drain(..nl).collect();
            self.buf.drain(..1); // drop the newline
            // Strip a trailing CR if present (CRLF support).
            let line = line.trim_end_matches('\r').to_string();
            out.push(line);
        }
        out
    }

    /// Flush any buffered partial line as a string.
    pub fn flush(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }

    /// Bytes currently buffered.
    pub fn pending(&self) -> usize {
        self.buf.len()
    }
}
