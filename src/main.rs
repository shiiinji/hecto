#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
  dbg!("aaa".to_string().len());
  dbg!("äää".to_string().len());
  dbg!("y̆y̆y̆".to_string().len());
  dbg!("❤❤❤".to_string().len());
  Editor::default().run();
}
