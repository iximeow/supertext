//! an html rendering crate. you probably shouldn't use it, but if you do want to use it it would
//! look something like this:
//! ```
//! use supertext::{HtmlWriter, HtmlSink, a, tr, td, th, table, p, h1, style};
//! use supertext::Sink;
//!
//! pub fn example(items: &[(String, String)]) -> Result<String, std::fmt::Error> {
//!     let mut out = String::new();
//!     let mut writer = HtmlWriter::new(&mut out);
//!
//!     let mut html = HtmlSink::root(&mut writer);
//!
//!     {
//!         let mut style = html.open_tag(style());
//!         style.rule(".table { font-family: monospace; border: 1px solid black; }")?;
//!         style.rule(".row-item { padding-left: 4px; padding-right: 4px; border-right: 1px solid black; }")?;
//!         style.rule(".odd-row { background: #eee; }")?;
//!         style.rule(".even-row { background: #ddd; }")?;
//!     }
//!
//!     {
//!         let mut header = html.open_tag(h1());
//!         header.text("an example page that is super cool")?;
//!     }
//!
//!     {
//!         let mut description = html.open_tag(p());
//!         match items.len() {
//!             0 => description.text("no items")?,
//!             1 => description.text("one item")?,
//!             o => description.text(&format!("{} items", o))?,
//!         };
//!     }
//!
//!     let mut table = html.open_tag(table().class("table"));
//!     {
//!         let mut header = table.header();
//!         let th = th().class("row-item");
//!         let mut add_cell = |name: &'static str| header.open_tag(th.clone()).text(name);
//!         add_cell("key")?;
//!         add_cell("value")?;
//!     }
//!
//!     let td = td().class("row-item");
//!
//!     for (i, item) in items.iter().enumerate() {
//!         let mut row = table.row(tr().class(["even-row", "odd-row"][i % 2]));
//!
//!         {
//!             let mut elem = row.open_tag(td.clone());
//!             elem.open_tag(a().href(&format!("/{}", item.0))).text(&item.0)?;
//!         }
//!         {
//!             row.open_tag(td.clone()).text(&item.1)?;
//!         }
//!     }
//!
//!     table.close();
//!
//!     html.close();
//!
//!     Ok(out)
//! }
//! ```
//!

// it'd be nice if these didn't require reallocating the string even if no characters were escaped.
// that's hard and i want to cry.
fn css_escape(s: &str) -> String {
    s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

fn attribute_escape(s: &str) -> String {
    s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

fn html_escape(s: &str) -> String {
    s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

mod tag {
    pub mod p {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagP {}

        impl SinkableTag for TagP {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                t.write_str("<p>")?;
                t.indent();
                Ok(())
            }
        }

        pub fn p() -> TagP {
            TagP {}
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagP {
            type Sink<'a> = PSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                PSink {
                    sink: out,
                }
            }
        }

        pub struct PSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> PSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for PSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.write_str("</p>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for PSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                self.sink.indent();
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod style {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::css_escape;

        #[derive(Clone)]
        pub struct TagStyle {}

        impl SinkableTag for TagStyle {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                t.write_str("<style>")?;
                t.indent();
                Ok(())
            }
        }

        pub fn style() -> TagStyle {
            TagStyle {}
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagStyle {
            type Sink<'a> = StyleSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("can write opening tag");
                StyleSink {
                    sink: out,
                }
            }
        }

        pub struct StyleSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> StyleSink<'a, W> {
            pub fn rule(&mut self, rule: &str) -> std::fmt::Result {
                self.sink.newline()?;
                self.sink.write_str(&css_escape(rule))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for StyleSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }

        impl<'a, W: HtmlWriter> Drop for StyleSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.newline().expect("newline");
                self.sink.write_str("</style>").expect("can write closing tag");
            }
        }
    }

    pub mod h1 {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagH1 {}

        impl SinkableTag for TagH1 {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                t.write_str("<hi>")?;
                t.indent();
                Ok(())
            }
        }

        pub fn h1() -> TagH1 {
            TagH1 {}
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagH1 {
            type Sink<'a> = H1Sink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                H1Sink {
                    sink: out,
                }
            }
        }

        pub struct H1Sink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> H1Sink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for H1Sink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.write_str("</h1>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for H1Sink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }

    }

    pub mod table {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagTable {
            class: Option<String>,
        }

        impl TagTable {
            pub fn class(mut self, text: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.class = Some(attribute_escape(text));
                self
            }
        }

        impl SinkableTag for TagTable {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<table class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<table>")?;
                }
                t.indent();
                Ok(())
            }
        }

        pub fn table() -> TagTable {
            TagTable {
                class: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagTable {
            type Sink<'a> = TableSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TableSink {
                    sink: out,
                }
            }
        }

        pub struct TableSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> TableSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn row<'b>(&'b mut self, tr: crate::tag::tr::TagTr) -> crate::tag::tr::TrSink<'b, W> {
                tr.prepare_sink(self.sink)
            }

            pub fn header<'b>(&'b mut self) -> HeaderSink<'b, W> {
                self.sink.newline().expect("can write opening tag");
                self.sink.write_str("<tr>").expect("str");
                self.sink.indent();
                HeaderSink {
                    sink: &mut self.sink,
                }
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for TableSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.newline().expect("newline");
                self.sink.write_str("</table>").expect("can write closing tag");
            }
        }

        pub struct HeaderSink<'a, T: HtmlWriter + ?Sized> {
            sink: &'a mut T,
        }

        impl<'a, T: HtmlWriter + ?Sized> HeaderSink<'a, T> {
            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, T: HtmlWriter + ?Sized> Drop for HeaderSink<'a, T> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.newline().expect("newline");
                self.sink.write_str("</tr>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for HeaderSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod th {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagTh {
            class: Option<String>,
        }

        impl SinkableTag for TagTh {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<th class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<th>")?;
                }
                t.indent();
                Ok(())
            }
        }

        impl TagTh {
            pub fn class(mut self, text: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.class = Some(attribute_escape(text));
                self
            }
        }

        pub fn th() -> TagTh {
            TagTh {
                class: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagTh {
            type Sink<'a> = ThSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                ThSink {
                    sink: out,
                }
            }
        }

        pub struct ThSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> ThSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for ThSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.write_str("</th>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for ThSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod tr {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::attribute_escape;
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagTr {
            class: Option<String>,
        }

        impl SinkableTag for TagTr {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<tr class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<tr>")?;
                }
                t.indent();
                Ok(())
            }
        }

        impl TagTr {
            pub fn class(mut self, text: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.class = Some(attribute_escape(text));
                self
            }
        }

        pub fn tr() -> TagTr {
            TagTr {
                class: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagTr {
            type Sink<'a> = TrSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TrSink {
                    sink: out,
                }
            }
        }

        pub struct TrSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> TrSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for TrSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.newline().expect("newline");
                self.sink.write_str("</tr>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for TrSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod td {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::attribute_escape;
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagTd {
            class: Option<String>,
        }

        impl SinkableTag for TagTd {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<td class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<td>")?;
                }
                t.indent();
                Ok(())
            }
        }

        impl TagTd {
            pub fn class(mut self, text: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.class = Some(attribute_escape(text));
                self
            }
        }

        pub fn td() -> TagTd {
            TagTd {
                class: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagTd {
            type Sink<'a> = TdSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TdSink {
                    child_tag: false,
                    sink: out,
                }
            }
        }

        pub struct TdSink<'a, W: HtmlWriter> {
            child_tag: bool,
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> TdSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for TdSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                if self.child_tag {
                    self.sink.newline().expect("newline");
                }
                self.sink.write_str("</td>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for TdSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                self.child_tag = true;
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod a {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagA {
            href: Option<String>,
        }

        impl SinkableTag for TagA {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(href) = self.href.as_ref() {
                    t.write_str("<a href=\"").expect("ok");
                    t.write_str(href).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<a>")?;
                }
                t.indent();
                Ok(())
            }
        }

        impl TagA {
            pub fn href(mut self, link: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.href = Some(attribute_escape(link));
                self
            }
        }

        pub fn a() -> TagA {
            TagA {
                href: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagA {
            type Sink<'a> = ASink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                ASink {
                    sink: out,
                }
            }
        }

        pub struct ASink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> ASink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for ASink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.write_str("</a>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for ASink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod span {
        use crate::tag::HtmlWriter;
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagSpan {
            style: Option<String>,
        }

        impl SinkableTag for TagSpan {
            fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result {
                t.newline()?;
                if let Some(style) = self.style.as_ref() {
                    t.write_str("<span style=\"").expect("ok");
                    t.write_str(style).expect("ok");
                    t.write_str("\">")?;
                } else {
                    t.write_str("<span>")?;
                }
                t.indent();
                Ok(())
            }
        }

        impl TagSpan {
            pub fn style(mut self, rule: &str) -> Self {
                // TODO: what to do to check style here. obviously `</style>` would be bogus. more
                // importantly, what to do on an error? returning `Result<Self, Error>` has the
                // unfortunate consequence of dropping `self` and closing the span. maybe that's
                // fine. maybe it should come with a poison on the underlying writer?
                self.style = Some(rule.to_owned());
                self
            }
        }

        pub fn span() -> TagSpan {
            TagSpan {
                style: None,
            }
        }

        impl<W: HtmlWriter> EnterableTag<W> for TagSpan {
            type Sink<'a> = SpanSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                SpanSink {
                    sink: out,
                }
            }
        }

        pub struct SpanSink<'a, W: HtmlWriter> {
            sink: &'a mut W,
        }

        impl<'a, W: HtmlWriter> SpanSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: HtmlWriter> Drop for SpanSink<'a, W> {
            fn drop(&mut self) {
                self.sink.dedent();
                self.sink.write_str("</span>").expect("can write closing tag");
            }
        }

        impl<'a, W: HtmlWriter> Sink<W> for SpanSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub trait SinkableTag {
        fn write_opening_tag(&self, t: &mut impl HtmlWriter) -> std::fmt::Result;
    }

    pub struct HtmlSink<'a, T: HtmlWriter + ?Sized> {
        sink: &'a mut T,
    }

    impl<'a, T: HtmlWriter + ?Sized> HtmlSink<'a, T> {
        pub fn root(sink: &'a mut T) -> Self {
            sink.write_str("<html>").expect("str");
            sink.indent();
            HtmlSink {
                sink,
            }
        }
    }

    impl<'a, T: HtmlWriter + ?Sized> Drop for HtmlSink<'a, T> {
        fn drop(&mut self) {
            self.sink.dedent();
            self.sink.newline().expect("newline");
            self.sink.write_str("</html>").expect("can write closing tag");
        }
    }

    impl<'a, T: HtmlWriter + ?Sized> HtmlSink<'a, T> {
        pub fn close(self) {
            std::mem::drop(self);
        }
    }


    // TODO: stop using std::fmt::Write
    pub trait HtmlWriter: std::fmt::Write {
        fn current_indent(&self) -> u64;
        fn indent(&mut self);
        fn dedent(&mut self);
        fn newline(&mut self) -> std::fmt::Result {
            self.write_str("\n")?;

            for _ in 0..self.current_indent() {
                self.write_str("    ")?;
            }

            Ok(())
        }
    }

    pub trait Sink<W: HtmlWriter> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b>;
    }

    pub trait EnterableTag<W: HtmlWriter> {
        type Sink<'a> /*: Sink<W>*/ where W: 'a;

        fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a>;
    }

    impl<'a, W: HtmlWriter> Sink<W> for HtmlSink<'a, W> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
            tag.prepare_sink(&mut self.sink)
        }
    }

}

pub use tag::{
    style::style, a::a, p::p, h1::h1, td::td,
    th::th, tr::tr, span::span,
    table::table, HtmlSink
};
pub use tag::Sink;

#[derive(Clone, Copy)]
enum BuildResult {
    Pass,
    Fail,
    InProgress,
}

impl BuildResult {
    fn as_str(&self) -> &'static str {
        match self {
            BuildResult::Pass => "pass",
            BuildResult::Fail => "fail",
            BuildResult::InProgress => "in progress",
        }
    }
}

struct Repo {
    name: String,
    commit: String,
    date: String,
    result: BuildResult,
}

/// a general adapter of [`std::fmt::Write`] implementations for HTML formatting. this produces
/// text that uses four space indentation, and tracks indentation levels based on subtag nesting.
pub struct HtmlWriter<'out, W: std::fmt::Write> {
    out: &'out mut W,
    current_indent: u64,
}

impl<'out, W: std::fmt::Write> HtmlWriter<'out, W> {
    pub fn new(out: &'out mut W) -> Self {
        HtmlWriter {
            out,
            current_indent: 0
        }
    }
}

impl<'out, W: std::fmt::Write> std::fmt::Write for HtmlWriter<'out, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.out.write_str(s)
    }
}

impl<'out, W: std::fmt::Write> crate::tag::HtmlWriter for HtmlWriter<'out, W> {
    fn current_indent(&self) -> u64 {
        self.current_indent
    }

    fn indent(&mut self) {
        self.current_indent += 1;
    }

    fn dedent(&mut self) {
        self.current_indent -= 1;
    }
}

/// a general adapter of [`std::fmt::Write`] implementations for HTML formatting. this does not add
/// unnecessary whitespace.
pub struct HtmlMiniWriter<'out, W: std::fmt::Write> {
    out: &'out mut W,
}

impl<'out, W: std::fmt::Write> HtmlMiniWriter<'out, W> {
    pub fn new(out: &'out mut W) -> Self {
        HtmlMiniWriter {
            out,
        }
    }
}

impl<'out, W: std::fmt::Write> std::fmt::Write for HtmlMiniWriter<'out, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.out.write_str(s)
    }
}

impl<'out, W: std::fmt::Write> crate::tag::HtmlWriter for HtmlMiniWriter<'out, W> {
    fn current_indent(&self) -> u64 {
        0
    }

    fn indent(&mut self) {
    }

    fn dedent(&mut self) {
    }

    fn newline(&mut self) -> std::fmt::Result { Ok(()) }
}

pub fn example() -> Result<String, std::fmt::Error> {
    let mut repos: Vec<Repo> = Vec::new();
    repos.push(Repo {
        name: "yaxpeax-avr".to_string(),
        commit: "50254b46dc2daad0cbe23b999b733e0e989e81ab".to_string(),
        date: "Wed, 21 Dec 2022 08:21:17 +0000".to_string(),
        result: BuildResult::Pass,
    });
    repos.push(Repo {
        name: "yaxpeax-zvm".to_string(),
        commit: "2b2175d771afb478f30b176118d6680b1c695eb8".to_string(),
        date: "Fri, 04 Aug 2023 23:47:11 +0000".to_string(),
        result: BuildResult::Fail,
    });
    repos.push(Repo {
        name: "yaxpeax-x86".to_string(),
        commit: "681262f4472ba4f452446e86012ce629b849d8d9".to_string(),
        date: "Wed, 26 Jun 2024 04:35:15 +0000".to_string(),
        result: BuildResult::InProgress,
    });

    let mut out = String::new();
    let mut writer = HtmlWriter::new(&mut out);

    let mut html = HtmlSink::root(&mut writer);

    {
        let mut style = html.open_tag(style());
        style.rule(".build-table { font-family: monospace; border: 1px solid black; }")?;
        style.rule(".row-item { padding-left: 4px; padding-right: 4px; border-right: 1px solid black; }")?;
        style.rule(".odd-row { background: #eee; }")?;
        style.rule(".even-row { background: #ddd; }")?;
    }

    {
        let mut header = html.open_tag(h1());
        header.text("builds and build accessories")?;
    }

    {
        let mut description = html.open_tag(p());
        match repos.len() {
            0 => description.text("no repos configured, so there are no builds")?,
            1 => description.text("1 repo configured")?,
            o => description.text(&format!("{} repos configured", o))?,
        };
    }

    let mut table = html.open_tag(table().class("build-table"));
    {
        let mut header = table.header();
        let th = th().class("row-item");
        let mut add_cell = |name: &'static str| header.open_tag(th.clone()).text(name);
        add_cell("repo")?;
        add_cell("last build")?;
        add_cell("commit/job")?;
        add_cell("remote")?;
        add_cell("duration")?;
        add_cell("status")?;
        add_cell("result")?;
    }

    let td = td().class("row-item");

    for (i, repo) in repos.iter().enumerate() {
        let mut row = table.row(tr().class(["even-row", "odd-row"][i % 2]));

        {
            let mut elem = row.open_tag(td.clone());
            elem.open_tag(a().href(&format!("/{}", repo.name))).text(&repo.name)?;
        }
        {
            row.open_tag(td.clone()).text(&repo.date)?;
        }
        {
            let mut elem = row.open_tag(td.clone());
            elem.open_tag(a().href(&format!("/{}/{}", repo.name, repo.commit))).text(&repo.commit[..9])?;
        }
        // remote

        // duration

        // status

        // result
        {
            let mut elem = row.open_tag(td.clone());
            elem.open_tag(match repo.result {
                BuildResult::Pass => span().style("color:green;"),
                BuildResult::Fail => span().style("color:red;"),
                BuildResult::InProgress => span().style("color:darkgoldenrod;"),
            }).text(repo.result.as_str())?;
        }
    }

    table.close();

    html.close();

    Ok(out)
}
