# depstalk

GitHub shows you the dependents of your repo:

![](https://github.com/Swatinem/depstalk/blob/master/usey-by.png?raw=true)

Though this is not available as an API, not is it possible to sort the page by stars.

As a workaround, this paginates through all the pages and scrapes the data,
showing you your most-starred dependents:

```
> depstalk Swatinem/rust-cache -n 20
Top 20 dependents of `Swatinem/rust-cache` (out of 9581 scraped):
- tauri-apps/tauri (60724 stars)
- parcel-bundler/parcel (42130 stars)
- rustdesk/rustdesk (39478 stars)
- meilisearch/meilisearch (34759 stars)
- rust-unofficial/awesome-rust (33260 stars)
- starship/starship (33201 stars)
- AppFlowy-IO/AppFlowy (32801 stars)
- yewstack/yew (26806 stars)
- sharkdp/fd (26759 stars)
- swc-project/swc (26626 stars)
- lapce/lapce (24525 stars)
- nushell/nushell (23851 stars)
- dani-garcia/vaultwarden (23658 stars)
- lencx/ChatGPT (23045 stars)
- rome/tools (22923 stars)
- vercel/turbo (20264 stars)
- SergioBenitez/Rocket (20111 stars)
- helix-editor/helix (19914 stars)
- tokio-rs/tokio (19742 stars)
- actix/actix-web (16934 stars)
```
