use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependent {
    pub owner: String,
    pub repo: String,
    pub stars: usize,
    pub forks: usize,
}

impl PartialEq for Dependent {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.repo == other.repo
    }
}

#[derive(Debug)]
pub struct ExtractResult {
    pub dependents: Vec<Dependent>,
    pub next_page: Option<String>,
}

pub struct Extractor {
    next_page: Selector,
    rows: Selector,
    user: Selector,
    repo: Selector,
    star: Selector,
    fork: Selector,
}

impl Extractor {
    pub fn new() -> Self {
        let next_page = Selector::parse(".paginate-container a:last-child").unwrap();
        let rows = Selector::parse(r#"[data-test-id="dg-repo-pkg-dependent"]"#).unwrap();
        let user = Selector::parse(
            r#"[data-hovercard-type="user"], [data-hovercard-type="organization"]"#,
        )
        .unwrap();
        let repo = Selector::parse(r#"[data-hovercard-type="repository"]"#).unwrap();
        let star = Selector::parse(".octicon-star").unwrap();
        let fork = Selector::parse(".octicon-repo-forked").unwrap();

        Self {
            next_page,
            rows,
            user,
            repo,
            star,
            fork,
        }
    }

    pub fn extract_from_html(&self, html: &str) -> ExtractResult {
        let html = Html::parse_document(html);

        let dependents = html
            .select(&self.rows)
            .flat_map(|row| {
                let dependent = self.extract_from_row(row);
                if dependent.is_none() {
                    tracing::warn!(html = row.html(), "unable to extract dependent from row");
                }
                dependent
            })
            .collect();

        let next_page = html
            .select(&self.next_page)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(ToOwned::to_owned);

        ExtractResult {
            dependents,
            next_page,
        }
    }

    fn extract_from_row(&self, row: ElementRef) -> Option<Dependent> {
        let owner = row
            .select(&self.user)
            .next()
            .map(|el| el.text().collect::<String>());

        let repo = row
            .select(&self.repo)
            .next()
            .map(|el| el.text().collect::<String>());

        let stars = row
            .select(&self.star)
            .next()
            .and_then(|el| el.next_sibling())
            .and_then(|el| el.value().as_text())
            .and_then(|s| s.trim().replace(',', "").parse().ok());

        let forks = row
            .select(&self.fork)
            .next()
            .and_then(|el| el.next_sibling())
            .and_then(|el| el.value().as_text())
            .and_then(|s| s.trim().replace(',', "").parse().ok());

        Some(Dependent {
            owner: owner?,
            repo: repo?,
            stars: stars?,
            forks: forks?,
        })
    }
}
