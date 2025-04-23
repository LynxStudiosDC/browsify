use anyhow::{bail, Result};
use glob::glob;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{Instant, SystemTime};
use tantivy::schema::{Schema, FAST, INDEXED, STORED, STRING, TEXT};
use tantivy::{doc, Index};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::info;

const COMMIT_THRESHOLD: usize = 1000;

#[derive(Debug, Deserialize)]
struct JsonlEntry {
    url: String,
    title: Option<String>,
    content_text: Option<String>,
    meta_content: Option<String>,
    language: Option<String>,
}

fn generate_preview(text: &str, max_len: usize) -> String {
    let text = text.trim();
    if text.len() <= max_len {
        text.to_string()
    } else {
        let mut preview = text.chars().take(max_len).collect::<String>();
        preview.push_str("...");
        preview
    }
}

async fn create_search_index() -> Result<Index> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let index_path = PathBuf::from("pulse_indexes").join(format!("index_{}", timestamp));

    std::fs::create_dir_all(&index_path)?;
    info!("Creating index at: {}", index_path.display());

    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("url", TEXT | STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("preview", STRING | STORED);
    schema_builder.add_text_field("language", STRING | STORED | FAST);
    schema_builder.add_text_field("meta_tags", TEXT | STORED);
    schema_builder.add_bool_field("nsfw", INDEXED | STORED | FAST);

    let schema = schema_builder.build();
    let index = Index::create_in_dir(&index_path, schema)?;
    Ok(index)
}

async fn check_files_exist(pattern: &str) -> Result<usize> {
    let mut count = 0;
    for entry in glob(pattern)? {
        match entry {
            Ok(_) => count += 1,
            Err(e) => tracing::warn!("Error matching pattern: {}", e),
        }
    }

    if count == 0 {
        bail!("No files found matching pattern: {}", pattern);
    }

    info!("Found {} files to process", count);
    Ok(count)
}

async fn index_documents(
    analyses_pattern: &str,
    index: &Index,
    nsfw_domains: &HashSet<String>,
) -> Result<()> {
    let start_time = Instant::now();
    let schema = index.schema();
    let mut total_processed = 0;

    let mut index_writer = index.writer_with_num_threads(4, 4 * 1024 * 1024 * 1024)?;

    info!("Starting to process files...");
    let mut file_count = 0;

    for entry in glob(analyses_pattern)? {
        match entry {
            Ok(path) => {
                file_count += 1;
                info!("Processing file [{}]: {}", file_count, path.display());
                let file_start_time = Instant::now();
                let mut line_count = 0;

                let file = File::open(&path).await?;
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await? {
                    line_count += 1;
                    match serde_json::from_str::<JsonlEntry>(&line) {
                        Ok(entry_data) => {
                            let content = entry_data.content_text.as_deref().unwrap_or_default();
                            let preview = generate_preview(content, 500);
                            let language = entry_data.language.unwrap_or_else(|| "en".to_string());

                            let title = entry_data.title.as_deref().unwrap_or_default();
                            let meta = entry_data.meta_content.as_deref().unwrap_or_default();

                            let is_nsfw_content = is_nsfw(content, nsfw_domains)
                                || is_nsfw(title, nsfw_domains)
                                || is_nsfw(meta, nsfw_domains)
                                || is_nsfw(&entry_data.url, nsfw_domains)
                                || is_nsfw_domain(&entry_data.url, nsfw_domains);

                            index_writer.add_document(doc!(
                                schema.get_field("url").unwrap() => entry_data.url,
                                schema.get_field("title").unwrap() => entry_data.title.unwrap_or_default(),
                                schema.get_field("content").unwrap() => content,
                                schema.get_field("preview").unwrap() => preview,
                                schema.get_field("language").unwrap() => language,
                                schema.get_field("meta_tags").unwrap() => entry_data.meta_content.unwrap_or_default(),
                                schema.get_field("nsfw").unwrap() => is_nsfw_content
                            ))?;

                            total_processed += 1;

                            if total_processed % COMMIT_THRESHOLD == 0 {
                                if let Ok(_) = index_writer.commit() {
                                    let elapsed = start_time.elapsed().as_secs_f64();
                                    let rate = total_processed as f64 / elapsed;
                                    info!(
                                        total_processed,
                                        rate = rate,
                                        "Processing at {:.2} docs/second",
                                        rate
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse JSON line {} in file {}: {}",
                                line_count,
                                path.display(),
                                e
                            );
                        }
                    }
                }

                info!(
                    "Finished file {} ({} lines) in {:.2}s",
                    path.display(),
                    line_count,
                    file_start_time.elapsed().as_secs_f64()
                );
            }
            Err(e) => tracing::error!("Error matching glob pattern: {}", e),
        }
    }

    info!("Performing final commit...");
    index_writer.commit()?;

    let total_duration = start_time.elapsed();
    info!(
        total_processed,
        total_files = file_count,
        duration = format!("{:?}", total_duration),
        "Indexing completed"
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    info!("Starting search indexer from JSONL files");

    let analyses_pattern = "analyses/partition=*/*.jsonl";
    info!("Looking for files matching: {}", analyses_pattern);

    // Check for files before creating index
    check_files_exist(analyses_pattern).await?;

    let nsfw_domains = load_nsfw_domains().unwrap_or_else(|_| {
        info!("Could not load NSFW domains list, continuing without it");
        HashSet::new()
    });

    let index = create_search_index().await?;
    info!("Search index created");

    index_documents(analyses_pattern, &index, &nsfw_domains).await?;

    info!("Search indexing completed successfully");
    info!("You can use the latest index in the 'pulse_indexes' directory for search operations");
    Ok(())
}

// THE CONTENT BELOW CONTAINS NSFW KEYWORDS
// DISCRETION ADVISED
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
// ====================================================
fn load_nsfw_domains() -> Result<HashSet<String>> {
    let domains = std::fs::read_to_string("top_1m_nsfw_sites.txt")?
        .lines()
        .map(|line| line.trim().to_lowercase())
        .collect();
    Ok(domains)
}

fn extract_domain(url: &str) -> Option<String> {
    url.trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("www.")
        .split('/')
        .next()
        .map(|s| s.to_lowercase())
}

fn is_nsfw_domain(url: &str, nsfw_domains: &HashSet<String>) -> bool {
    if let Some(domain) = extract_domain(url) {
        nsfw_domains.contains(&domain)
    } else {
        false
    }
}

fn is_nsfw(text: &str, nsfw_domains: &HashSet<String>) -> bool {
    is_nsfw_domain(text, nsfw_domains)
}
