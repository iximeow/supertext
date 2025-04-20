fn main() {
    println!("{}", example().expect("is ok"));
}

fn css_escape(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

fn attribute_escape(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

fn html_escape(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

mod tag {
    use std::fmt::Write;

    pub mod p {
        use crate::tag::{EnterableTag, Sink, SinkableTag};

        #[derive(Clone)]
        pub struct TagP {}

        impl SinkableTag for TagP {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                t.write_str("<p>")
            }
            fn closing_tag(&self) -> &'static str {
                "</p>"
            }
        }

        pub fn p() -> TagP {
            TagP {}
        }
    }

    pub mod style {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::css_escape;

        #[derive(Clone)]
        pub struct TagStyle {}

        impl SinkableTag for TagStyle {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                t.write_str("<style>")
            }
            fn closing_tag(&self) -> &'static str {
                "</style>"
            }
        }

        pub fn style() -> TagStyle {
            TagStyle {}
        }

        impl<W: std::fmt::Write> EnterableTag<W> for TagStyle {
            type Sink<'a> = StyleSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                out.write_str("<style>").expect("can write opening tag");
                StyleSink {
                    sink: out,
                }
            }
        }

        pub struct StyleSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> StyleSink<'a, W> {
            pub fn rule(&mut self, rule: &str) -> std::fmt::Result {
                self.sink.write_str(&css_escape(rule))?;
                // TODO: document pretty-printing makes this optional
                self.sink.write_str("\n")?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for StyleSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }

        impl<'a, W: std::fmt::Write> Drop for StyleSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</style>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }
    }

    pub mod h1 {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagH1 {}

        impl SinkableTag for TagH1 {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                t.write_str("<h1>")
            }
            fn closing_tag(&self) -> &'static str {
                "</h1>"
            }
        }

        pub fn h1() -> TagH1 {
            TagH1 {}
        }

        impl<W: std::fmt::Write> EnterableTag<W> for TagH1 {
            type Sink<'a> = H1Sink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                H1Sink {
                    sink: out,
                }
            }
        }

        pub struct H1Sink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> H1Sink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                // TODO: document pretty-printing makes this optional
                self.sink.write_str("\n")?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: std::fmt::Write> Drop for H1Sink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</h1>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for H1Sink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }

    }

    pub mod table {
        use crate::tag::{EnterableTag, Sink, SinkableTag};

        #[derive(Clone)]
        pub struct TagTable {}

        impl SinkableTag for TagTable {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                t.write_str("<table>")
            }
            fn closing_tag(&self) -> &'static str {
                "</table>"
            }
        }

        pub fn table() -> TagTable {
            TagTable {}
        }
    }

    pub mod th {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::attribute_escape;

        #[derive(Clone)]
        pub struct TagTh {
            class: Option<String>,
        }

        impl SinkableTag for TagTh {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(class) = self.class.as_ref() {
                    write!(t, "<th class=\"{}\">", class)
                } else {
                    t.write_str("<th>")
                }
            }
            fn closing_tag(&self) -> &'static str {
                "</th>"
            }
        }

        impl TagTh {
            // TODO: these
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
    }

    pub mod td {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::attribute_escape;
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagTd {
            class: Option<String>,
        }

        impl SinkableTag for TagTd {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(class) = self.class.as_ref() {
                    write!(t, "<td class=\"{}\">", class)
                } else {
                    t.write_str("<td>")
                }
            }
            fn closing_tag(&self) -> &'static str {
                "</td>"
            }
        }

        impl TagTd {
            // TODO: these
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

        impl<W: std::fmt::Write> EnterableTag<W> for TagTd {
            type Sink<'a> = TdSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TdSink {
                    sink: out,
                }
            }
        }

        pub struct TdSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> TdSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                // TODO: document pretty-printing makes this optional
                self.sink.write_str("\n")?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: std::fmt::Write> Drop for TdSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</td>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for TdSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod a {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagA {
            href: Option<String>,
        }

        impl SinkableTag for TagA {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(href) = self.href.as_ref() {
                    write!(t, "<a href=\"{}\">", href)
                } else {
                    t.write_str("<a>")
                }
            }
            fn closing_tag(&self) -> &'static str {
                "</a>"
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

        impl<W: std::fmt::Write> EnterableTag<W> for TagA {
            type Sink<'a> = ASink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                ASink {
                    sink: out,
                }
            }
        }

        pub struct ASink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> ASink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                // TODO: document pretty-printing makes this optional
                self.sink.write_str("\n")?;
                Ok(())
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: std::fmt::Write> Drop for ASink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</a>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for ASink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    trait SinkableTag {
        fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result;
        fn closing_tag(&self) -> &'static str;
    }

    pub struct HtmlSink<'a, T: Write + ?Sized> {
        sink: &'a mut T,
        closing_tag: &'static str,
    }

    impl<'a, T: Write + ?Sized> HtmlSink<'a, T> {
        pub fn root(sink: &'a mut T) -> Self {
            sink.write_str("<html>\n").expect("str");
            HtmlSink {
                sink,
                closing_tag: "</html>",
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> Drop for HtmlSink<'a, T> {
        fn drop(&mut self) {
            self.sink.write_str(self.closing_tag).expect("can write closing tag");
            // TODO: configurable pretty-printing
            self.sink.write_str("\n").expect("newline");
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> HtmlSink<'a, T> {
        pub fn close(self) {
            std::mem::drop(self);
        }
    }

    pub struct RowSink<'a, T: Write + ?Sized> {
        sink: &'a mut T,
    }

    impl<'a, T: std::fmt::Write + ?Sized> RowSink<'a, T> {
        pub fn close(self) {
            std::mem::drop(self);
        }
    }


    pub struct HeaderSink<'a, T: Write + ?Sized> {
        sink: &'a mut T,
    }

    impl<'a, T: std::fmt::Write + ?Sized> HeaderSink<'a, T> {
        pub fn close(self) {
            std::mem::drop(self);
        }
    }

    impl<'a, T: Write + ?Sized> Drop for HeaderSink<'a, T> {
        fn drop(&mut self) {
            self.sink.write_str("</tr>").expect("can write closing tag");
            // TODO: configurable pretty-printing
            self.sink.write_str("\n").expect("newline");
        }
    }

    impl<'a, T: Write + ?Sized> Drop for RowSink<'a, T> {
        fn drop(&mut self) {
            self.sink.write_str("</tr>").expect("can write closing tag");
            // TODO: configurable pretty-printing
            self.sink.write_str("\n").expect("newline");
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> HeaderSink<'a, T> {
        pub fn open_tag<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> HtmlSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            HtmlSink {
                sink: &mut self.sink,
                closing_tag: tag.closing_tag(),
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> RowSink<'a, T> {
        pub fn untyped_open_tag<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> RowSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            RowSink {
                sink: &mut self.sink,
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> HtmlSink<'a, T> {
        pub fn untyped_open_tag<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> HtmlSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            HtmlSink {
                sink: &mut self.sink,
                closing_tag: tag.closing_tag(),
            }
        }

        pub fn text(&mut self, text: &str) -> std::fmt::Result {
            // TODO: allow a type param to indicate escapedness, escape here..
            self.sink.write_str(&crate::html_escape(text))
        }

        pub fn row<'b>(&'b mut self) -> RowSink<'b, T> {
            RowSink {
                sink: &mut self.sink,
            }
        }

        pub fn header<'b>(&'b mut self) -> HeaderSink<'b, T> {
            self.sink.write_str("<tr>").expect("str");
            self.sink.write_str("\n").expect("str");
            HeaderSink {
                sink: &mut self.sink,
            }
        }
    }




    pub trait Sink<W: std::fmt::Write> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b>;
    }

    pub trait EnterableTag<W: std::fmt::Write> {
        type Sink<'a>: Sink<W> where W: 'a;

        fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a>;
    }

    impl<'a, W: std::fmt::Write> Sink<W> for HtmlSink<'a, W> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
            tag.prepare_sink(&mut self.sink)
        }
    }

    impl<'a, W: std::fmt::Write> Sink<W> for RowSink<'a, W> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
            tag.prepare_sink(&mut self.sink)
        }
    }
}

use tag::{style::style, a::a, p::p, h1::h1, td::td, th::th, table::table, HtmlSink};
use tag::Sink;

struct Repo {
    name: String,
    commit: String,
    date: String,
}

pub fn example() -> Result<String, std::fmt::Error> {
    let mut repos: Vec<Repo> = Vec::new();
    repos.push(Repo {
        name: "yaxpeax-avr".to_string(),
        commit: "50254b46dc2daad0cbe23b999b733e0e989e81ab".to_string(),
        date: "Wed, 21 Dec 2022 08:21:17 +0000".to_string(),
    });
    repos.push(Repo {
        name: "yaxpeax-zvm".to_string(),
        commit: "2b2175d771afb478f30b176118d6680b1c695eb8".to_string(),
        date: "Fri, 04 Aug 2023 23:47:11 +0000".to_string(),
    });

    let mut out = String::new();

    let mut html = HtmlSink::root(&mut out);

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
        let mut description = html.untyped_open_tag(p());
        match repos.len() {
            0 => description.text("no repos configured, so there are no builds")?,
            1 => description.text("1 repo configured")?,
            o => description.text(&format!("{} repos configured", o))?,
        };
    }

    let mut table = html.untyped_open_tag(table());
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

    for repo in repos.iter() {
        let mut row = table.row();

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
    }

    table.close();

    html.close();

    Ok(out)
}
