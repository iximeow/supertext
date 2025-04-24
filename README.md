# supertext

hypertext, a bit less hyper

i want to construct HTML documents in some Rust code. i don't want a full
templating solution. i just want to append some tags and not forget to escape
user-provided input.

you probably don't want to use this, but if you do it should work. public
interfaces are basically expected to break. it is 0.0.1 for a reason.

## example

```rust
pub fn example(items: &[(String, String)]) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    let mut writer = HtmlWriter::new(&mut out);

    let mut html = HtmlSink::root(&mut writer);

    {
        let mut style = html.open_tag(style());
        style.rule(".table { font-family: monospace; border: 1px solid black; }")?;
        style.rule(".row-item { padding-left: 4px; padding-right: 4px; border-right: 1px solid black; }")?;
        style.rule(".odd-row { background: #eee; }")?;
        style.rule(".even-row { background: #ddd; }")?;
    }

    {
        let mut header = html.open_tag(h1());
        header.text("an example page that is super cool")?;
    }

    {
        let mut description = html.open_tag(p());
        match items.len() {
            0 => description.text("no items")?,
            1 => description.text("one item")?,
            o => description.text(&format!("{} items", o))?,
        };
    }

    let mut table = html.open_tag(table().class("table"));
    {
        let mut header = table.header();
        let th = th().class("row-item");
        let mut add_cell = |name: &'static str| header.open_tag(th.clone()).text(name);
        add_cell("key")?;
        add_cell("value")?;
    }

    let td = td().class("row-item");

    for (i, item) in items.iter().enumerate() {
        let mut row = table.row(tr().class(["even-row", "odd-row"][i % 2]));

        {
            let mut elem = row.open_tag(td.clone());
            elem.open_tag(a().href(&format!("/{}", item.0))).text(&item.0)?;
        }
        {
            row.open_tag(td.clone()).text(&item.1)?;
        }
    }

    table.close();

    html.close();

    Ok(out)
}
```

## non-goals

this crate is intended to avoid trivially injectable HTML document
construction. it is not intended to prevent semantically incoherent HTML
documents. yet?

for example, this crate allows creating a table with twelve columns in its
header, but varying (and not-twelve) rows afterwards. this crate allows
tags to reference styles that are undefined. this crate allows intra-page
links that are invalid. this crate allows syntactically invalid CSS.

## TODO

- [ ] element nesting could be more principled: sections like `phrasing content`
      in the HTML spec draw a clearer line about what elements can be nested.
- [ ] really want to fuzz document construction and ensure no inputs produce
      different trees when parsed than when constructed
- [ ] really want to be able to check if strings need escaping at compile time
      rather than runtime
- [ ] for strings that need runtime escaping, would really like to not reallocate
      for replaced strings - retaining borrows of original data and not
      producing interim replaced copies would be nice. maybe something like a
      rope instead? or views on the underlying string with buffers of
      replacements to be evaluated when rendered out.
- [ ] what to do with css rules? each rule gets a newline currently, is that ok?
- [ ] controlling indentation for paragraphs? `p.text()` lines always just
      appends escaped text to the writer, maybe those should be spaced with
      newlines? should `text()` automatically append a line break?

## related work (and things this is not)

[build\_html](https://docs.rs/build_html/latest/build_html/) looks good,
almost is what i want! unfortunately: 

> Note that escaping strings is also not automatic. You should use the
> escape\_html function if you are displaying untrusted text.

the API was already wanted more ceremony than i was happy with, but this moves
it out of the running.

[html](https://docs.rs/html/latest/html/) is comprehensive, but perhaps too
much so.

the docs surface area is roughly the product of all kinds of tags by all kinds
of tags, to the point i have a bit of a difficult time with navigation. it
includes lots of HTML spec that is probably interesting and meaningful if
i was doing interesting things with HTML, but i'm not.

needing to set `#![recursion_limit = "512"]` when depending on this crate is
another papercut; between that and its idleness, i'm OK passing here.

[maud](https://docs.rs/maud/latest/maud/) ([book](https://maud.lambda.xyz/))
seems good, but i don't want a template engine, i just want to put a few divs
and spans together. more importantly, i want to be able to render elements of
a page potentially independently. having to construct an entire HTML document
in one go is not great, and having to learn a new syntax for control flow in
the document is the opposite direction of what i want.
