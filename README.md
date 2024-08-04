# depstalk

GitHub shows you the dependents of your repo:

![](https://github.com/Swatinem/depstalk/blob/master/used-by.png?raw=true)

Though this is not available as an API, nor is it possible to sort the page by stars.

As a workaround, this paginates through all the pages and scrapes the data,
showing you your most-starred dependents:

```
> depstalk Swatinem/rust-cache -n 20
Top 20 dependents of `Swatinem/rust-cache` (out of 24408 scraped):
- facebook/react (225930 stars)
- vercel/next.js (123720 stars)
- tauri-apps/tauri (80410 stars)
- ChatGPTNextWeb/ChatGPT-Next-Web (73621 stars)
- rustdesk/rustdesk (69359 stars)
- FuelLabs/sway (62878 stars)
- FuelLabs/fuel-core (58258 stars)
- lencx/ChatGPT (51885 stars)
- rust-lang/rustlings (51745 stars)
- AppFlowy-IO/AppFlowy (50950 stars)
- meilisearch/meilisearch (45723 stars)
- rust-unofficial/awesome-rust (44982 stars)
- FuelLabs/fuels-rs (44340 stars)
- starship/starship (43597 stars)
- parcel-bundler/parcel (43317 stars)
- zed-industries/zed (43141 stars)
- dani-garcia/vaultwarden (35378 stars)
- lapce/lapce (33981 stars)
- sharkdp/fd (32893 stars)
- helix-editor/helix (31984 stars)
```
