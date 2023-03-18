use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::extractor::{Dependent, ExtractResult, Extractor};

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchChunk {
    first_row: Dependent,
    next_page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub base_url: String,
    pub chunks: Vec<FetchChunk>,
    pub dependents: Vec<Dependent>,
}

impl State {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            chunks: vec![],
            dependents: vec![],
        }
    }
}

pub struct Fetcher {
    client: Client,
    extractor: Extractor,
}

impl Fetcher {
    pub fn new() -> anyhow::Result<Self> {
        let client = Client::builder().http2_prior_knowledge().build()?;
        let extractor = Extractor::new();
        Ok(Self { client, extractor })
    }

    pub fn fetch_pages(&self, state: &mut State, num_pages: usize) -> anyhow::Result<()> {
        // The `dependents` page uses cursor-based pagination.
        // I have no idea by what it is sorted, but we can assume it has some
        // form of stable sorting, and new items are added at the beginning.
        // We will always start by fetching the first page, but will then
        // automatically merge this, and continue at, a previously run fetching
        // session (`chunk`).
        // This *will* fail (and double-fetch) when the exact `first_row` was
        // removed from the dataset. And it also does not consider data updates.
        let mut next_page = Some(state.base_url.clone());
        let mut first_row = None;

        macro_rules! commit_in_progress_chunk {
            () => {
                if let Some(first_row) = first_row {
                    state.chunks.push(FetchChunk {
                        first_row,
                        next_page,
                    });
                }
            };
        }

        for _ in 0..num_pages {
            let Some(url) = next_page.as_deref() else {
                tracing::debug!("nothing more to fetch");
                break;
            };

            let result = match self.try_fetch_page(url) {
                Ok(result) => result,
                Err(err) => {
                    commit_in_progress_chunk!();
                    return Err(err);
                }
            };

            if first_row.is_none() {
                first_row = result.dependents.first().cloned()
            }

            if let Some(chunk) = state.chunks.pop() {
                let existing_first_row = &chunk.first_row;
                let current_page_first_row_idx = result
                    .dependents
                    .iter()
                    .enumerate()
                    .find_map(|(idx, dep)| (dep == existing_first_row).then_some(idx));

                // we fetched enough to arrive at the next chunk
                if let Some(idx) = current_page_first_row_idx {
                    // this means we only want to push the *new* dependents on the top
                    state
                        .dependents
                        .extend_from_slice(&result.dependents[..idx]);
                    // and continue with the next page of that chunk
                    next_page = chunk.next_page;
                    tracing::debug!("continuing previous chunk");
                    continue;
                } else {
                    state.chunks.push(chunk);
                }
            }

            // otherwise, if we did not merge with an existing chunk,
            // add all the results and continue paging
            state.dependents.extend(result.dependents);
            next_page = result.next_page;
        }

        commit_in_progress_chunk!();

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn try_fetch_page(&self, url: &str) -> anyhow::Result<ExtractResult> {
        let response = self.client.get(url).send()?;
        anyhow::ensure!(response.status().is_success(), "{}", response.status());

        let html = response.text()?;
        Ok(self.extractor.extract_from_html(&html))
    }
}
