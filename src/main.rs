use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

use clap::Parser;
use phrase_segmentation::collect_phrases;
use phrase_segmentation::IndexedDocument;
use serde::Deserialize;
use serde_jsonlines::json_lines;

#[derive(Debug, Deserialize)]
struct DocumentSet {
    document: Vec<u16>,
    relevant_documents: Vec<Vec<u16>>,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long)]
    input_path: PathBuf,

    #[arg(long)]
    output_path: PathBuf,

    #[arg(long)]
    min_phrase_len: usize,

    #[arg(long)]
    max_phrase_len: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let document_set = json_lines(&cli.input_path)?.collect::<Result<Vec<DocumentSet>>>()?;
    let document_set: Vec<(IndexedDocument, Vec<IndexedDocument>)> = document_set
        .into_iter()
        .map(|d| {
            (
                IndexedDocument::from_tokens(d.document),
                d.relevant_documents
                    .into_iter()
                    .map(|d| IndexedDocument::from_tokens(d))
                    .collect::<Vec<IndexedDocument>>(),
            )
        })
        .collect();
    let phrases = collect_phrases(
        document_set.iter().map(|(d, r)| (d, r.as_slice())),
        cli.min_phrase_len,
        cli.max_phrase_len,
    );

    let file = File::create(cli.output_path)?;
    serde_json::to_writer(file, &phrases)?;

    Ok(())
}
