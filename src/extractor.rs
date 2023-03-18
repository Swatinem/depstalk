use scraper::{Html, Selector};
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
        let user = Selector::parse(r#"[data-hovercard-type="user"]"#).unwrap();
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
        let selectors = Extractor::new();

        let dependents = html
            .select(&selectors.rows)
            .flat_map(|row| {
                let owner = row
                    .select(&selectors.user)
                    .next()?
                    .text()
                    .collect::<String>();

                let repo = row
                    .select(&selectors.repo)
                    .next()?
                    .text()
                    .collect::<String>();

                let stars = row
                    .select(&selectors.star)
                    .next()?
                    .next_sibling()?
                    .value()
                    .as_text()?
                    .trim()
                    .parse()
                    .ok()?;

                let forks = row
                    .select(&selectors.fork)
                    .next()?
                    .next_sibling()?
                    .value()
                    .as_text()?
                    .trim()
                    .parse()
                    .ok()?;

                Some(Dependent {
                    owner,
                    repo,
                    stars,
                    forks,
                })
            })
            .collect();

        let next_page = html
            .select(&selectors.next_page)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(ToOwned::to_owned);

        ExtractResult {
            dependents,
            next_page,
        }
    }
}
