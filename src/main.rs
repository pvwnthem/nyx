mod editor;
mod terminal;
fn main() {
    let mut editor: editor::Editor = editor::Editor::default();
    editor.run();
}
