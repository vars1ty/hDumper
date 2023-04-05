# hDumper
> **Warning**: hDumper may or may not expose telephone number(s).
>
> It's only intended for educational purposes and as a PoC (proof-of-concept) of how easy it is to make a record searcher.

***
A public citizen record searcher.

## Supported Countries
The only supported country for now is **Sweden**.

## Usage
There are (so far) no proper arguments present in hDumper, so you'll have to rely on environment variables.

- `SEARCH` | Who do you want to find? It can be either the first, middle or last name of a person.
- `END_PAGE` | How many pages should be searched through?
   - RT (Ratsit) only allows for **3** pages, which means that the amount of pages you specify will automatically be clamped to 3 for this particular source.
   - EN (Eniro) doesn't have a clear max-amount of pages that you can search through, so it isn't affected by the clamp operation made by RT.
- `AREA` | In what approximate area do you want to find records within? Note that this only **attempts** to narrow your search down, it may not succeed 100% of the time.

### Example
```sh
SEARCH="Jonathan" END_PAGE=5 AREA="" ./hDumper
```

## Sources
hDumper utilizes 2 sources and their undocumented APIs to grab and display data.

1. Ratsit (a.k.a. RT) - Usually provides you with more accurate and up-to-date information, at the expense of a 3-pages-only rule.
2. Eniro (a.k.a. EN) - Allows for broader searches which may or may not have inaccurate results, at the expense of not sharing the 3-pages-only rule as RT.

Whenever you search through public records, hDumper creates several threads to process the data. Approximate time to finish a search is `2.5s` for 5 pages.

## Why?
- "It may give scammers access to data, and it puts a privacy/security-risk for others."
   - Yeah it might, but none of this data is non-public, everything is grabbed from **public websites**, no data has been manually supplied.
   - It's labelled as a PoC for a reason, to showcase how incredibly exposed your data is if someone knows a few things about you.
   - hDumper does nothing that you can't already do yourself using a web browser. In fact, it may sometimes even be more efficient to do your own research at times.
