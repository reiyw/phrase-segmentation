use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

use clap::Parser;
use phrase_segmentation::collect_phrases;
use phrase_segmentation::IndexedDocument;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DocumentSet {
    documents: Vec<Vec<u16>>,
    relevant_document_ids: Vec<Vec<usize>>,
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

    let document_set: DocumentSet = serde_json::from_reader(File::open(&cli.input_path)?)?;
    let indexed_documents: Vec<_> = document_set
        .documents
        .iter()
        .map(|d| IndexedDocument::from_tokens(d.clone()))
        .collect();
    let relevant_documents: Vec<Vec<&IndexedDocument>> = document_set
        .relevant_document_ids
        .iter()
        .map(|ids| {
            ids.iter()
                .map(|i| &indexed_documents[*i])
                .collect::<Vec<&IndexedDocument>>()
        })
        .collect();
    let phrases = collect_phrases(
        indexed_documents.iter().zip(relevant_documents),
        cli.min_phrase_len,
        cli.max_phrase_len,
    );

    let file = File::create(cli.output_path)?;
    serde_json::to_writer(file, &phrases)?;

    Ok(())
}
