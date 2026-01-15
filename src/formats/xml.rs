use std::path::Path;

use anyhow::Result;
use quick_xml::Writer;
use quick_xml::events::Event;

use super::{FormatError, ensure_newline};

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let mut reader = quick_xml::Reader::from_str(text);
    reader.trim_text(true);
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(e) => writer
                .write_event(e)
                .map_err(|e| FormatError::Message(e.to_string()))?,
            Err(e) => return Err(FormatError::Message(format!("XML parse error: {e}"))),
        }
        buf.clear();
    }
    let out =
        String::from_utf8(writer.into_inner()).map_err(|e| FormatError::Message(e.to_string()))?;
    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
