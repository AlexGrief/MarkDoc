use std::{env, fs::File, io::Write};

use docx_rs::{DocumentChild, Paragraph, ParagraphChild, RunChild, read_docx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = env::args().collect();
    let mut file = File::open(input)?;
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    let docx = read_docx(&buf)?;

    let mut md_writer = MarkdownWriter::new();

    for child in &docx.document.children {
        match child {
            DocumentChild::Paragraph(paragraph) => {
                let text = extract_paragraph_text(paragraph);

                if let Some(style) = &paragraph.property.style {
                    match style.val.as_str() {
                        "Heading1" => md_writer.write_header(1, &text),
                        "Heading2" => md_writer.write_header(2, &text),
                        "Heading3" => md_writer.write_header(3, &text),
                        _ => md_writer.write_paragraph(&text), // Пишем все что не попало в фильтр как обычный текст
                    }
                } else {
                    md_writer.write_paragraph(&text);
                }
            }
            DocumentChild::Table(table) => todo!(),
            DocumentChild::BookmarkStart(bookmark_start) => todo!(),
            DocumentChild::BookmarkEnd(bookmark_end) => todo!(),
            DocumentChild::CommentStart(comment_range_start) => todo!(),
            DocumentChild::CommentEnd(comment_range_end) => todo!(),
            DocumentChild::StructuredDataTag(structured_data_tag) => todo!(),
            DocumentChild::TableOfContents(table_of_contents) => todo!(),
            DocumentChild::Section(section) => todo!(),
        }
    }

    Ok(())
}

struct MarkdownWriter {
    buffer: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    pub fn write_header(&mut self, level: usize, text: &str) {
        let hashes = "#".repeat(level);
        self.buffer.push_str(&format!("{} {}\n\n", hashes, text));
    }

    pub fn write_paragraph(&mut self, text: &str) {
        if !text.trim().is_empty() {
            self.buffer.push_str(&format!("{}\n\n", text));
        }
    }

    pub fn write_hr(&mut self) {
        self.buffer.push_str(&format!("---\n\n"));
    }

    pub fn finish(self, output_path: &str) -> std::io::Result<()> {
        let mut output = File::create(output_path)?;
        output.write_all(self.buffer.as_bytes())?;
        Ok(())
    }
}

fn extract_paragraph_text(paragraph: &docx_rs::Paragraph) -> String {
    let mut paragraph_text = String::new();

    for parchild in &paragraph.children {
        if let ParagraphChild::Run(run) = parchild {
            let mut text = String::new();

            for run_child in &run.children {
                if let RunChild::Text(t) = run_child {
                    text.push_str(&t.text);
                }
            }

            if run.run_property.bold.is_some() {
                text = format!("**{}**", text)
            }

            if run.run_property.italic.is_some() {
                text = format!("*{}*", text)
            }

            paragraph_text.push_str(&text);
        }
    }
    paragraph_text
}