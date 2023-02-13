# current behavior

- XML Pasrsing (via static/optional struct declaration and serde extension based deserialization)
- field based searching
- in-line diff based extraction
- coloring and human formatting of differences

# crates built on

- **using**: `serde` for deserialization, `xml_serde_rs` for xml pattern of deserializer, `similar` for multi-algorithm text differencing, `console` for formatting of output

- **tentative**: `polars` for easier manipulation of data for statistics calculation, `quick-xml` for faster alternative to xml deserialization should it become an issue, `struct-field-names-as-array` for struct name extration macro -- for onveneince as part of transposing and dataframe construction for `polars`, `roxmltree` should there be critical attribute data separate from child data (re: xml's unusual parse characteristics)

# testing & linting

- currently using [pre-commit hook](https://github.com/ethanmsl/Boilerplate/blob/main/Rust-Boilerplate/rust-pre-commit); will probably add github actions in near future 
