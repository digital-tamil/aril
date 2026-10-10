#[derive(Debug)]
pub(crate) struct DiffViewer;

impl DiffViewer {
    pub(crate) fn render_preview(original: &str, converted: &str, file_name: Option<&str>) {
        if let Some(name) = file_name {
            println!("\x1b[1;34m--- a/{name}\x1b[0m");
            println!("\x1b[1;32m+++ b/{name}\x1b[0m");
        } else {
            println!("\x1b[1;34m--- original\x1b[0m");
            println!("\x1b[1;32m+++ converted\x1b[0m");
        }

        let orig_lines: Vec<&str> = original.lines().collect();
        let conv_lines: Vec<&str> = converted.lines().collect();
        let max_lines = orig_lines.len().max(conv_lines.len());

        for i in 0..max_lines.min(10) {
            let orig = orig_lines.get(i).copied().unwrap_or("");
            let conv = conv_lines.get(i).copied().unwrap_or("");

            if orig == conv {
                println!("  {orig}");
            } else {
                println!("\x1b[31m- {orig}\x1b[0m");
                println!("\x1b[32m+ {conv}\x1b[0m");
            }
        }

        if max_lines > 10 {
            println!(
                "\x1b[90m... [remaining {} lines omitted from diff preview]\x1b[0m",
                max_lines - 10
            );
        }
    }
}
