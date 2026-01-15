use std::path::Path;

use anyhow::Result;
use html5ever::tendril::TendrilSink;
use html5ever::{serialize, serialize::SerializeOpts};
use markup5ever_rcdom::{RcDom, SerializableHandle};

use super::{FormatError, ensure_newline};

pub fn format(_path: &Path, text: &str) -> Result<Option<String>, FormatError> {
    let dom: RcDom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .map_err(|e| FormatError::Message(format!("html parse error: {e:?}")))?;

    let mut out = Vec::new();
    let opts = SerializeOpts {
        traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
        ..Default::default()
    };

    let handle: SerializableHandle = dom.document.clone().into();
    serialize(&mut out, &handle, opts).map_err(|e| FormatError::Message(e.to_string()))?;
    let out = String::from_utf8(out).map_err(|e| FormatError::Message(e.to_string()))?;

    if out == text {
        Ok(None)
    } else {
        Ok(Some(ensure_newline(out)))
    }
}
