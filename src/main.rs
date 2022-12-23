use clap::Parser;
use std::fs;

use docx_rs::*;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct SearchAndReplace {
    pub search: String,
    pub replace: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchesAndReplacements {
    pub snps: Vec<SearchAndReplace>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub inf: String,

    #[arg(short, long)]
    pub outf: String,

    /// Number of times to greet
    #[arg(short, long)]
    replacements: String,
}

pub fn main() -> Result<(), DocxError> {
    let args = Args::parse();
    let inpath = std::path::Path::new(&args.inf);
    // let infile = std::fs::File::create(&inpath).unwrap();

    let mut file = File::open(inpath).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let mut dx = read_docx(&buf).unwrap();
    // let mut file = File::create("./hello.json")?;

    let repf = File::open(&args.replacements).unwrap();
    let replacements = serde_json::from_reader(repf).unwrap();

    search_and_replace(&mut dx, &replacements);

    let outfile = std::fs::File::create(args.outf).unwrap();
    dx.build().pack(outfile);

    Ok(())
}

pub fn search_and_replace(mut dx: &mut Docx, snr: &SearchesAndReplacements) {
    for mut dc in &mut dx.document.children {
        snr_dc(&mut dc, snr);
    }
}
pub fn snr_dc(dc: &mut DocumentChild, snr: &SearchesAndReplacements) {
    match dc {
        DocumentChild::Paragraph(paragraph) => snr_p(paragraph, &snr),
        DocumentChild::Table(table) => snr_t(table, &snr),
        DocumentChild::BookmarkStart(bookmark_start) => (),
        DocumentChild::BookmarkEnd(bookmark_end) => (),
        DocumentChild::CommentStart(comment_range_start) => (),
        DocumentChild::CommentEnd(comment_range_end) => (),
        DocumentChild::StructuredDataTag(structured_data_tag) => (),
        DocumentChild::TableOfContents(table_of_contents) => snr_tc(&mut **table_of_contents, snr),
    }
}

pub fn snr_t(mut t: &mut Table, snr: &SearchesAndReplacements) {
    // for tc in &mut t.rows {
    //     match tc {
    //         TableChild::TableRow(tr) => {
    //             for c in &mut tr.cells {
    //                 match c {
    //                     TableRowChild::TableCell(tc) => {
    //                         for mut tcc in &mut tc.children {
    //                             match tcc {
    //                                 TableCellContent::Paragraph(mut paragraph) => {
    //                                     snr_p(&mut paragraph, &snr)
    //                                 }
    //                                 TableCellContent::Table(mut table) => snr_t(&mut table, &snr),
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

pub fn search_n_replace(mut s: &mut String, snr: &SearchesAndReplacements) {
    for sar in &snr.snps {
        let ns = s.replace(sar.search.as_str(), sar.replace.as_str());
        s.clear();
        s.push_str(ns.as_str());
    }
}

pub fn snr_tc(mut tc: &mut TableOfContents, snr: &SearchesAndReplacements) {
    for mut i in &mut tc.items {
        search_n_replace(&mut i.text, &snr);
        search_n_replace(&mut i.toc_key, &snr);
        // pub instr: InstrToC,
        // pub text: String,
        // pub toc_key: String,
        // pub level: usize,
        // pub dirty: bool,
        // pub page_ref: Option<String>,
    }
}

pub fn snr_p(mut p: &Paragraph, snr: &SearchesAndReplacements) {
    for c in &p.children {
        snr_pc(&c, &snr)
    }
}

pub fn snr_pc(mut pc: &ParagraphChild, snr: &SearchesAndReplacements) {
    match pc {
        ParagraphChild::Run(run) => {
            for c in &run.children {
                snr_rc(&c, &snr)
            }
        }
        ParagraphChild::Insert(insert) => (),
        ParagraphChild::Delete(delete) => (),
        ParagraphChild::BookmarkStart(bookmarkStart) => (),
        ParagraphChild::Hyperlink(hyperlink) => (),
        ParagraphChild::BookmarkEnd(bookmarkEnd) => (),
        ParagraphChild::CommentStart(commentRangeStart) => (),
        ParagraphChild::CommentEnd(commentRangeEnd) => (),
        ParagraphChild::StructuredDataTag(structuredDataTag) => (),
    }
}

pub fn snr_rc(mut pc: &RunChild, snr: &SearchesAndReplacements) {
    match pc {
        RunChild::Text(text) => (),
        RunChild::DeleteText(deleteText) => (),
        RunChild::Tab(tab) => (),
        RunChild::Break(break_) => (),
        RunChild::Drawing(drawing) => (),
        RunChild::Shape(shape) => (),
        RunChild::CommentStart(commentRangeStart) => (),
        RunChild::CommentEnd(commentRangeEnd) => (),
        RunChild::FieldChar(fieldChar) => (),
        RunChild::InstrText(instrText) => (),
        // For reader
        RunChild::InstrTextString(string) => (),
    }
}

// pub fn main() {
//     let mut file = File::open("./invoice.docx").unwrap();
//     let mut buf = vec![];
//     file.read_to_end(&mut buf).unwrap();

//     let mut file = File::create("./hello.json").unwrap();
//     let res = read_docx(&buf).unwrap().json();
//     file.write_all(res.as_bytes()).unwrap();
//     file.flush().unwrap();
// }
