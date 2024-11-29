# current behavior

- XML Pasrsing (via static/optional struct declaration and serde extension based deserialization)
- field based searching
- in-line diff based extraction
- coloring and human formatting of differences

# XML Structure
link: [XML Wikipedia](https://en.wikipedia.org/wiki/XML_tree)
- XML tags form a tree (re: nesting structure)
  - e.g. `|root> |a>..<a| |b>|c>..<c|<b| <root|`
- valid XML docs have a root-node (which may have an arbitrary name)
- XML docs may have another element, outside the root-node, which contains xml info (e.g. `<?xml version="1.0" encoding="UTF-8" ?>`)
- Tag' may have attributes (e.g. `<tag attr="value">`), variables embedded in tag opening.
- Tag structure:
  - `<tag>...</tag>`: opening and closing tag
  - `<tag/>`: self-closing tag
  - `<tag attr="value">...</tag>`: tag with attribute
- Various other items (e.g. CDATA, comments, attribute normalization, DTD specs)

# XML Parsing
Parsing comes in two varieties:
- "DOM" based, which assumes full tree resides in-memory
- 'streaming' based, which allows larger than memory parsing

Rust libraries focus on streaming approach, with Serde support for (~) DOM-like parsing.
`XPATH` is apparently a (JQL-like?) approach common to parsing XML, but is not well represented in the Rust-ecosystem.

# Crates of Interest
## [Quick-XML](https://github.com/tafia/quick-xml)
- decent [Docs](https://docs.rs/quick-xml/latest/quick_xml/) & [Readme](https://docs.rs/quick-xml/latest/quick_xml/), good [Examples](https://github.com/tafia/quick-xml/tree/master/examples)
- streaming
- r/w
- serde-compatible with [`serialize`](https://docs.rs/quick-xml/latest/quick_xml/index.html#serialize) flag
  - [`de`(serialize) module docs](https://docs.rs/quick-xml/latest/quick_xml/de/index.html)
  - *!! there exists a tool to derive quick-xml structs from XML files, [xml_schema_generator](https://github.com/Thomblin/xml_schema_generator) !!*
  - **NOTE**: `serde_json::Value` can accept general value to allow for capture of unspecified values.
    - ```rust
        #[derive(Debug, Serialize, Deserialize)]
        struct XmlStruct {
            // ...
            #[serde(flatten)]
            other_vals: HashMap<String, serde_json::Value>,
        }
        ```
       ```
- no iterator methods (related to being able to return references, e.g. for very large document)
- lower-level parsing; ~manual, if not using serde
  - depth and tag matching are up to programmer
- Key Components:
  - [`Config`](https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Config.html)
    - Configure parser behavior
  - [`Reader`](https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html)
    - main struct that will walk through XML
  - [`Events`](https://docs.rs/quick-xml/latest/quick_xml/events/enum.Event.html)
    - enum of what can be found while reading XML
    - ```rust
      pub enum Event<'a> {
          Start(BytesStart<'a>),
          End(BytesEnd<'a>),
          Empty(BytesStart<'a>),
          Text(BytesText<'a>),
          CData(BytesCData<'a>),
          Comment(BytesText<'a>),
          Decl(BytesDecl<'a>),
          PI(BytesPI<'a>),
          DocType(BytesText<'a>),
          Eof,
      }
      ```


## [RoxmlTree](https://github.com/RazrFalcon/roxmltree)
- decent [examples](https://github.com/RazrFalcon/roxmltree/tree/master/examples), good [readme](https://github.com/RazrFalcon/roxmltree), with nice clarity on abilities, **poor** [docs](https://docs.rs/roxmltree/latest/roxmltree/)
- read-only
- accessory, [serde-roxmltree](https://github.com/adamreichold/serde-roxmltree), offers serde-compatibility
  - [docs](https://docs.rs/serde-roxmltree/latest/serde_roxmltree/) are decent, given simplicity
- iterator methods
- [memory usage](https://github.com/RazrFalcon/roxmltree/blob/master/README.md#memory-overhead) (over)estimate: 10x the size of the XML document
- Key Components:
  - [Document](https://docs.rs/roxmltree/latest/roxmltree/struct.Document.html#)
  - [NodeType](https://docs.rs/roxmltree/latest/roxmltree/enum.NodeType.html#)
    - ```rust
      pub enum NodeType {
          Root,
          Element,
          PI,
          Comment,
          Text,
      }
      ```

## Other
(What I originally wrote uses [xml_serde_rs](https://github.com/RReverser/serde-xml-rs), which clearly works, but is two-years unchanged and probably not the ideal option.)

# crates built on

- **using**: `serde` for deserialization, `xml_serde_rs` for xml pattern of deserializer, `similar` for multi-algorithm text differencing, `console` for formatting of output

- **tentative**: `polars` for easier manipulation of data for statistics calculation, `quick-xml` for faster alternative to xml deserialization should it become an issue, `struct-field-names-as-array` for struct name extraction macro -- for onveneince as part of transposing and dataframe construction for `polars`, `roxmltree` should there be critical attribute data separate from child data (re: xml's unusual parse characteristics)

# testing & linting

- currently using [pre-commit hook](https://github.com/ethanmsl/Boilerplate/blob/main/Rust-Boilerplate/rust-pre-commit); will probably add github actions in near future
