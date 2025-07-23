use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    ctx.set_contents(text.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}