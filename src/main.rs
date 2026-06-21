use std::{any::Any, env, fs::File, io::Write, vec};

use docx_rs::{DocumentChild, Header, Paragraph, ParagraphChild, Run, RunChild, Table, TableRow, read_docx};

//use crate::Node::Paragraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = env::args().collect();
    let mut file = File::open(&input[0])?;
    let output_path = &input[1];
    
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    let docx = read_docx(&buf)?;

    let mut md_writer = MarkdownWriter::new();
    let mut otf_doc = MdTable::new();

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
                                                docx_rs::TableCellContent::Paragraph(paragraph) => {
                                                    let text = extract_paragraph_text(paragraph);
                                                    let r = Run::new();
                                                    r.clone().add_text(&text);
                                                    let ru = &r.to_owned();

                                                    let par = Paragraph::new();
                                                    par.clone().add_run(ru.clone());
                                                    

                                                    let header = Header::new();

                                                    let paru = par.to_owned();
                                                    header.clone().add_paragraph(paru);

                                                    let headers = Some(header);

                                                    if let Some(style) = &paragraph.property.style {
                                                        match style.val.as_str() {
                                                            "Heading1" => outtable.write_header(1, headers.as_ref()),
                                                            "Heading2" => outtable.write_header(2, headers.as_ref()),
                                                            "Heading3" => outtable.write_header(3, headers.as_ref()),
                                                            _ => outtable.write_paragraph(&par.clone()), // Пишем все что не попало в фильтр как обычный текст
                                                        }
                                                    } else {
                                                        md_writer.write_paragraph(&text);
                                                    }
                                                },
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

    pub fn write_paragraph(&mut self, paragraphd: &Paragraph) {
        for rows in &self.rows {
            for cells in &rows.cells  {
                for child in &cells.children {
                    match child {
                        Node::Paragraph(paragraph) => {
                            Node::Paragraph((paragraphd.clone()));
                        },
                        Node::Header(header) => todo!(),
                        Node::Table(table) => todo!(),
                    }
                }
            }
        }
    }
    
    pub fn write_header(&mut self, hashes: usize, headerd: Option<&Header>) {
        for rows in &self.rows {
            for cells in &rows.cells  {
                for child in &cells.children {
                    match child {
                        Node::Paragraph(paragraph) => todo!(),
                        Node::Header(header) => {
                            Node::Header(headerd.cloned());
                        },
                        Node::Table(table) => todo!(),
                    }
                }
            }
        }
    }

    pub fn write_table(&mut self, tabled: Option<&Table>) {
        for rows in &self.rows {
            for cells in &rows.cells  {
                for child in &cells.children {
                    match child {
                        Node::Paragraph(paragraph) => todo!(),
                        Node::Header(header) => todo!(),
                        Node::Table(table) => {
                            Node::Table(tabled.cloned());
                        },
                    }
                }
            }
        }
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
        Self { children: Vec::new() }
    }
}

#[derive(Clone)]
enum Node {
    Paragraph(Paragraph),
    Header(Option<Header>),
    Table(Option<Table>)
}

impl Node {
    pub fn new() -> Self {
        Self::Paragraph((Paragraph::new()));
        Self::Header((None));
        Self::Table((None))
    }

    pub fn write_content(&mut self, content: Node) {
        match content {
            Node::Paragraph(paragraph) => {
                Self::Paragraph(paragraph);
            },
            Node::Header(header) => {
                Self::Header(header);
            },
            Node::Table(table) => {
                Self::Table(table);
            },
        }
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