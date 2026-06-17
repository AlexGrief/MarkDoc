use std::{env, fs::File, io::Write, vec};

use docx_rs::{DocumentChild, Header, Paragraph, ParagraphChild, RunChild, Table, TableRow, read_docx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = env::args().collect();
    let mut file = File::open(&input[0])?;
    let output_path = &input[1];
    
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
            DocumentChild::Table(table) => {
                let mut outtable = MdTable::new();

                for table_child in &table.rows {
                    match table_child {
                        docx_rs::TableChild::TableRow(row) => {
                            for cell in &row.cells {
                                match cell {
                                    docx_rs::TableRowChild::TableCell(table_cell) => {
                                        for cell_children in &table_cell.children {
                                            match cell_children {
                                                docx_rs::TableCellContent::Paragraph(paragraph) => todo!(),
                                                docx_rs::TableCellContent::Table(table) => todo!(),
                                                docx_rs::TableCellContent::StructuredDataTag(structured_data_tag) => todo!(),
                                                docx_rs::TableCellContent::TableOfContents(table_of_contents) => todo!(),
                                            }
                                        }
                                    },
                                }
                            }
                        },
                    }
                }
            },
            DocumentChild::BookmarkStart(bookmark_start) => todo!(),
            DocumentChild::BookmarkEnd(bookmark_end) => todo!(),
            DocumentChild::CommentStart(comment_range_start) => todo!(),
            DocumentChild::CommentEnd(comment_range_end) => todo!(),
            DocumentChild::StructuredDataTag(structured_data_tag) => todo!(),
            DocumentChild::TableOfContents(table_of_contents) => todo!(),
            DocumentChild::Section(section) => todo!(),
        }
    }

    md_writer.finish(&output_path.to_string())?;

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

    pub fn finish(self, output_path: &str) -> std::io::Result<()> {
        let mut output = File::create(output_path)?;
        output.write_all(self.buffer.as_bytes())?;
        Ok(())
    }
}

struct MdTable {
    rows: Vec<Row>,
}

impl MdTable {
    pub fn new() -> Self {
        Self { rows: vec![Row::new()] }
    }
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Self {
        Self { cells: vec![Cell::new()] }
    }
}

struct Cell {
    children: Vec<Node>,
}

impl Cell {
    pub fn new() -> Self {
        todo!()
    }
}

enum Node {
    Paragraph(Paragraph),
    Header(Header),
    Table(Table),
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