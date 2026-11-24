# Changelog

## [0.2.0](https://github.com/terror/unclutter/releases/tag/0.2.0) - 2026-11-24

### Added

- Accept parser options in wasm `parse` ([#126](https://github.com/terror/unclutter/pull/126) by [terror](https://github.com/terror))
- Decode non-UTF-8 input ([#125](https://github.com/terror/unclutter/pull/125) by [terror](https://github.com/terror))
- Add `--json` option ([#123](https://github.com/terror/unclutter/pull/123) by [terror](https://github.com/terror))
- Add `--url` option ([#121](https://github.com/terror/unclutter/pull/121) by [terror](https://github.com/terror))
- Read HTML from standard input ([#117](https://github.com/terror/unclutter/pull/117) by [terror](https://github.com/terror))

### Changed

- Rename `preserved_classes` to `classes_to_preserve` ([#122](https://github.com/terror/unclutter/pull/122) by [terror](https://github.com/terror))
- Make `link_density_bias` an `f64` ([#120](https://github.com/terror/unclutter/pull/120) by [terror](https://github.com/terror))
- Remove unused `debug` option and `InvalidSelector` error ([#116](https://github.com/terror/unclutter/pull/116) by [terror](https://github.com/terror))
- Consume `Parser` in `parse` ([#106](https://github.com/terror/unclutter/pull/106) by [terror](https://github.com/terror))
- Keep default preserved classes in builder ([#103](https://github.com/terror/unclutter/pull/103) by [terror](https://github.com/terror))

### Fixed

- Match JSON-LD `@context` exactly ([#119](https://github.com/terror/unclutter/pull/119) by [terror](https://github.com/terror))
- Extract meta tags after removing `<noscript>` ([#118](https://github.com/terror/unclutter/pull/118) by [terror](https://github.com/terror))
- Match hidden styles with regexes ([#113](https://github.com/terror/unclutter/pull/113) by [terror](https://github.com/terror))
- Walk trees iteratively ([#112](https://github.com/terror/unclutter/pull/112) by [terror](https://github.com/terror))
- Restore extraction state from best attempt ([#109](https://github.com/terror/unclutter/pull/109) by [terror](https://github.com/terror))
- Keep `lang` attribute as written ([#108](https://github.com/terror/unclutter/pull/108) by [terror](https://github.com/terror))
- Treat zero `max_elements` as no limit ([#107](https://github.com/terror/unclutter/pull/107) by [terror](https://github.com/terror))
- Saturate table row and column counts ([#105](https://github.com/terror/unclutter/pull/105) by [terror](https://github.com/terror))
- Skip lazy image fixes for SVG placeholders ([#104](https://github.com/terror/unclutter/pull/104) by [terror](https://github.com/terror))
- Unescape named entities in a single pass ([#102](https://github.com/terror/unclutter/pull/102) by [terror](https://github.com/terror))
- Unwrap `javascript:` links without a base URL ([#101](https://github.com/terror/unclutter/pull/101) by [terror](https://github.com/terror))

### Misc

- Update benchmark chart ([#127](https://github.com/terror/unclutter/pull/127) by [terror](https://github.com/terror))
- Collect `CleanConditionally` counts in one pass ([#124](https://github.com/terror/unclutter/pull/124) by [terror](https://github.com/terror))
- Remove `Context::options` ([#115](https://github.com/terror/unclutter/pull/115) by [terror](https://github.com/terror))
- Keep only the best article attempt ([#114](https://github.com/terror/unclutter/pull/114) by [terror](https://github.com/terror))
- Extract meta title lazily ([#111](https://github.com/terror/unclutter/pull/111) by [terror](https://github.com/terror))
- Reuse `NodeExt` phrasing helpers in `RewriteLineBreaks` ([#110](https://github.com/terror/unclutter/pull/110) by [terror](https://github.com/terror))
- Move rules into `src/rule` ([#100](https://github.com/terror/unclutter/pull/100) by [terror](https://github.com/terror))
- Bump assert-html-eq to `1e4a188` ([#99](https://github.com/terror/unclutter/pull/99) by [terror](https://github.com/terror))
- Use absolute image URLs in readme ([#98](https://github.com/terror/unclutter/pull/98) by [terror](https://github.com/terror))

## [0.1.1](https://github.com/terror/unclutter/releases/tag/0.1.1) - 2026-11-24

### Added

- Add --version flag ([#81](https://github.com/terror/unclutter/pull/81) by [terror](https://github.com/terror))

### Fixed

- Grant release workflow write access to contents ([#80](https://github.com/terror/unclutter/pull/80) by [terror](https://github.com/terror))

### Misc

- Update benchmark chart ([#96](https://github.com/terror/unclutter/pull/96) by [terror](https://github.com/terror))
- Compute candidate link densities in one pass ([#95](https://github.com/terror/unclutter/pull/95) by [terror](https://github.com/terror))
- Measure text without normalizing it ([#94](https://github.com/terror/unclutter/pull/94) by [terror](https://github.com/terror))
- Clone document instead of reparsing for article retries ([#93](https://github.com/terror/unclutter/pull/93) by [terror](https://github.com/terror))
- Avoid allocations in node helpers ([#92](https://github.com/terror/unclutter/pull/92) by [terror](https://github.com/terror))
- Bump url from 2.5.8 to 2.5.9 ([#85](https://github.com/terror/unclutter/pull/85) by [app/dependabot](https://github.com/app/dependabot))
- Bump dom_query from 0.12.0 to 0.28.0 ([#86](https://github.com/terror/unclutter/pull/86) by [app/dependabot](https://github.com/app/dependabot))
- Bump anyhow from 1.0.100 to 1.0.104 ([#88](https://github.com/terror/unclutter/pull/88) by [app/dependabot](https://github.com/app/dependabot))
- Bump clap from 4.5.54 to 4.5.60 ([#89](https://github.com/terror/unclutter/pull/89) by [app/dependabot](https://github.com/app/dependabot))
- Bump assert-html-eq from `b754f20` to `abeb7ce` ([#87](https://github.com/terror/unclutter/pull/87) by [app/dependabot](https://github.com/app/dependabot))
- Add zizmor check to CI ([#90](https://github.com/terror/unclutter/pull/90) by [terror](https://github.com/terror))
- Track coverage with Codecov ([#84](https://github.com/terror/unclutter/pull/84) by [terror](https://github.com/terror))
- Configure dependabot ([#83](https://github.com/terror/unclutter/pull/83) by [terror](https://github.com/terror))
- Add release, crates.io and downloads badges ([#82](https://github.com/terror/unclutter/pull/82) by [terror](https://github.com/terror))

## [0.1.0](https://github.com/terror/unclutter/releases/tag/0.1.0) - 2026-11-24

### Added

- Add `unclutter-wasm` crate ([#77](https://github.com/terror/unclutter/pull/77) by [terror](https://github.com/terror))
- Add benchmarks ([#72](https://github.com/terror/unclutter/pull/72) by [terror](https://github.com/terror))
- Check readerable against the readability test suite ([#64](https://github.com/terror/unclutter/pull/64) by [terror](https://github.com/terror))
- Add article post-processing stages ([#60](https://github.com/terror/unclutter/pull/60) by [terror](https://github.com/terror))
- Add article preparation stages ([#59](https://github.com/terror/unclutter/pull/59) by [terror](https://github.com/terror))
- Add article extraction stages ([#56](https://github.com/terror/unclutter/pull/56) by [terror](https://github.com/terror))
- Prefer JSON-LD headline when it matches the document title ([#53](https://github.com/terror/unclutter/pull/53) by [terror](https://github.com/terror))
- Add `RemoveEmptyContainers` stage ([#50](https://github.com/terror/unclutter/pull/50) by [terror](https://github.com/terror))
- Add `RemoveUnlikelyCandidates` stage ([#47](https://github.com/terror/unclutter/pull/47) by [terror](https://github.com/terror))
- Add `RemoveHiddenNodes` stage ([#46](https://github.com/terror/unclutter/pull/46) by [terror](https://github.com/terror))
- Add `ExtractByline` stage ([#44](https://github.com/terror/unclutter/pull/44) by [terror](https://github.com/terror))
- Add `UnescapeHtmlEntities` stage ([#43](https://github.com/terror/unclutter/pull/43) by [terror](https://github.com/terror))
- Add `ExcerptFallback` stage ([#42](https://github.com/terror/unclutter/pull/42) by [terror](https://github.com/terror))
- Add `Document::attribute` method for attribute extraction ([#41](https://github.com/terror/unclutter/pull/41) by [terror](https://github.com/terror))
- Add `ExtractDir` stage ([#39](https://github.com/terror/unclutter/pull/39) by [terror](https://github.com/terror))
- Add `ExtractLang` stage ([#38](https://github.com/terror/unclutter/pull/38) by [terror](https://github.com/terror))
- Add `ExtractMetaTags` stage ([#36](https://github.com/terror/unclutter/pull/36) by [terror](https://github.com/terror))
- Add `ExtractJsonLd` stage ([#35](https://github.com/terror/unclutter/pull/35) by [terror](https://github.com/terror))
- Add `RewriteLineBreaks` stage ([#32](https://github.com/terror/unclutter/pull/32) by [terror](https://github.com/terror))
- Add `UnwrapNoscriptImages` stage ([#31](https://github.com/terror/unclutter/pull/31) by [terror](https://github.com/terror))
- Set article html and text content ([#30](https://github.com/terror/unclutter/pull/30) by [terror](https://github.com/terror))
- Add `RewriteFontTags` stage ([#27](https://github.com/terror/unclutter/pull/27) by [terror](https://github.com/terror))
- Add `RemoveDisallowedNodes` stage ([#26](https://github.com/terror/unclutter/pull/26) by [terror](https://github.com/terror))
- Copy lazy-loading image sources into standard attributes ([#18](https://github.com/terror/unclutter/pull/18) by [terror](https://github.com/terror))
- Replace javascript links in parsed article ([#17](https://github.com/terror/unclutter/pull/17) by [terror](https://github.com/terror))
- Apply weighting scheme for article stage ([#14](https://github.com/terror/unclutter/pull/14) by [terror](https://github.com/terror))
- Extract `dir` from source article ([#12](https://github.com/terror/unclutter/pull/12) by [terror](https://github.com/terror))
- Fallback to body for missing article candidate and decode metadata HTML entities ([#9](https://github.com/terror/unclutter/pull/9) by [terror](https://github.com/terror))
- Handle whitespace-separated metadata property tokens ([#8](https://github.com/terror/unclutter/pull/8) by [terror](https://github.com/terror))
- Normalize article headings to keep `h1` reserved for the page title ([#7](https://github.com/terror/unclutter/pull/7) by [terror](https://github.com/terror))
- Flatten single-cell tables in extracted article markup ([#6](https://github.com/terror/unclutter/pull/6) by [terror](https://github.com/terror))
- Add additional sanitization passes ([#4](https://github.com/terror/unclutter/pull/4) by [terror](https://github.com/terror))
- Extend byline extraction routine ([#1](https://github.com/terror/unclutter/pull/1) by [terror](https://github.com/terror))

### Fixed

- Match meta name pattern and normalize file URL drive letters ([#63](https://github.com/terror/unclutter/pull/63) by [terror](https://github.com/terror))
- Match noscript image unwrapping and strip comments ([#62](https://github.com/terror/unclutter/pull/62) by [terror](https://github.com/terror))
- Fix title extraction to match Readability.js ([#52](https://github.com/terror/unclutter/pull/52) by [terror](https://github.com/terror))
- Move byline extraction after content removal ([#48](https://github.com/terror/unclutter/pull/48) by [terror](https://github.com/terror))
- Decrease `title` metadata key priority ([#22](https://github.com/terror/unclutter/pull/22) by [terror](https://github.com/terror))
- Ensure metadata extraction captures `og:article:author` bylines ([#21](https://github.com/terror/unclutter/pull/21) by [terror](https://github.com/terror))

### Misc

- Rename `is_probably_readerable` to `is_readable` ([#78](https://github.com/terror/unclutter/pull/78) by [terror](https://github.com/terror))
- Use dark build badge ([#75](https://github.com/terror/unclutter/pull/75) by [terror](https://github.com/terror))
- Rename readability in docs ([#76](https://github.com/terror/unclutter/pull/76) by [terror](https://github.com/terror))
- Remove integration badge ([#74](https://github.com/terror/unclutter/pull/74) by [terror](https://github.com/terror))
- Rename crate to unclutter ([#73](https://github.com/terror/unclutter/pull/73) by [terror](https://github.com/terror))
- Sort tests alphabetically ([#71](https://github.com/terror/unclutter/pull/71) by [terror](https://github.com/terror))
- Move inherent impls above trait impls ([#70](https://github.com/terror/unclutter/pull/70) by [terror](https://github.com/terror))
- Inline pipeline stage lists ([#69](https://github.com/terror/unclutter/pull/69) by [terror](https://github.com/terror))
- Update stale doc comments ([#68](https://github.com/terror/unclutter/pull/68) by [terror](https://github.com/terror))
- Share class and id match string helper ([#67](https://github.com/terror/unclutter/pull/67) by [terror](https://github.com/terror))
- Organize the default pipeline into phases ([#66](https://github.com/terror/unclutter/pull/66) by [terror](https://github.com/terror))
- Group article extraction state into `Extraction` ([#65](https://github.com/terror/unclutter/pull/65) by [terror](https://github.com/terror))
- Run node preparation rules in a single traversal ([#61](https://github.com/terror/unclutter/pull/61) by [terror](https://github.com/terror))
- Sync readability submodule ([#58](https://github.com/terror/unclutter/pull/58) by [terror](https://github.com/terror))
- Fix clippy lints ([#54](https://github.com/terror/unclutter/pull/54) by [terror](https://github.com/terror))
- Remove broken WSL bash workaround ([#51](https://github.com/terror/unclutter/pull/51) by [terror](https://github.com/terror))
- Rename `ExcerptFallback` stage to `ExtractExcerpt` ([#49](https://github.com/terror/unclutter/pull/49) by [terror](https://github.com/terror))
- Lift out `ExtractTitle` stage ([#45](https://github.com/terror/unclutter/pull/45) by [terror](https://github.com/terror))
- Add `Test` fixture for stage unit tests ([#40](https://github.com/terror/unclutter/pull/40) by [terror](https://github.com/terror))
- Add passing metadata integration tests ([#37](https://github.com/terror/unclutter/pull/37) by [terror](https://github.com/terror))
- Add stage documentation ([#34](https://github.com/terror/unclutter/pull/34) by [terror](https://github.com/terror))
- Add `AGENTS.md` file in project root ([#33](https://github.com/terror/unclutter/pull/33) by [terror](https://github.com/terror))
- Add `watch` recipe to justfile ([#29](https://github.com/terror/unclutter/pull/29) by [terror](https://github.com/terror))
- Add `test` macro for stage tests ([#28](https://github.com/terror/unclutter/pull/28) by [terror](https://github.com/terror))
- Reset codebase to use `dom_query` ([#25](https://github.com/terror/unclutter/pull/25) by [terror](https://github.com/terror))
- Put `Regex` definitions in their own module ([#23](https://github.com/terror/unclutter/pull/23) by [terror](https://github.com/terror))
- Split tests for expected metadata and output ([#20](https://github.com/terror/unclutter/pull/20) by [terror](https://github.com/terror))
- Use `assert-html-eq` crate in integration test suite ([#19](https://github.com/terror/unclutter/pull/19) by [terror](https://github.com/terror))
- Add unit tests for document helper ([#16](https://github.com/terror/unclutter/pull/16) by [terror](https://github.com/terror))
- Add passing class-weight based fixtures to integration tests ([#15](https://github.com/terror/unclutter/pull/15) by [terror](https://github.com/terror))
- Inline smaller stages where applicable ([#13](https://github.com/terror/unclutter/pull/13) by [terror](https://github.com/terror))
- Make metadata immutable in collection stage ([#11](https://github.com/terror/unclutter/pull/11) by [terror](https://github.com/terror))
- Add passing metadata and RTL fixtures to integration tests ([#10](https://github.com/terror/unclutter/pull/10) by [terror](https://github.com/terror))
- Test directly against model readability test pages ([#5](https://github.com/terror/unclutter/pull/5) by [terror](https://github.com/terror))
- Add model readability as a submodule ([#3](https://github.com/terror/unclutter/pull/3) by [terror](https://github.com/terror))
- Refactor core readability flow into stages ([#2](https://github.com/terror/unclutter/pull/2) by [terror](https://github.com/terror))
