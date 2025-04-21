// TODO: obv remove
#![allow(dead_code)]

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
    use std::fmt::Write;

    pub mod p {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagP {}

        impl SinkableTag for TagP {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                t.write_str("<p>")
            }
        }

        pub fn p() -> TagP {
            TagP {}
        }

        impl<W: std::fmt::Write> EnterableTag<W> for TagP {
            type Sink<'a> = PSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                PSink {
                    sink: out,
                }
            }
        }

        pub struct PSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> PSink<'a, W> {
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

        impl<'a, W: std::fmt::Write> Drop for PSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</p>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for PSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
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
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagTable {
            class: Option<String>,
        }

        impl TagTable {
            // TODO: these
            pub fn class(mut self, text: &str) -> Self {
                // TODO: would be nice to just take a `&str` and have the lifetimes plumbed through..
                self.class = Some(attribute_escape(text));
                self
            }
        }

        impl SinkableTag for TagTable {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<table class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<table>")
                }
            }
        }

        pub fn table() -> TagTable {
            TagTable {
                class: None,
            }
        }

        impl<W: std::fmt::Write> EnterableTag<W> for TagTable {
            type Sink<'a> = TableSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TableSink {
                    sink: out,
                }
            }
        }

        pub struct TableSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> TableSink<'a, W> {
            pub fn text(&mut self, text: &str) -> std::fmt::Result {
                self.sink.write_str(&html_escape(text))?;
                // TODO: document pretty-printing makes this optional
                self.sink.write_str("\n")?;
                Ok(())
            }

            pub fn row<'b>(&'b mut self, tr: crate::tag::tr::TagTr) -> crate::tag::tr::TrSink<'b, W> {
                tr.prepare_sink(self.sink)
            }

            pub fn header<'b>(&'b mut self) -> HeaderSink<'b, W> {
                self.sink.write_str("<tr>").expect("str");
                self.sink.write_str("\n").expect("str");
                HeaderSink {
                    sink: &mut self.sink,
                }
            }

            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, W: std::fmt::Write> Drop for TableSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</table>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        pub struct HeaderSink<'a, T: std::fmt::Write + ?Sized> {
            sink: &'a mut T,
        }

        impl<'a, T: std::fmt::Write + ?Sized> HeaderSink<'a, T> {
            pub fn close(self) {
                std::mem::drop(self);
            }
        }

        impl<'a, T: std::fmt::Write + ?Sized> Drop for HeaderSink<'a, T> {
            fn drop(&mut self) {
                self.sink.write_str("</tr>").expect("can write closing tag");
                // TODO: configurable pretty-printing
                self.sink.write_str("\n").expect("newline");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for HeaderSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod th {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::{attribute_escape, html_escape};

        #[derive(Clone)]
        pub struct TagTh {
            class: Option<String>,
        }

        impl SinkableTag for TagTh {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<th class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<th>")
                }
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

        impl<W: std::fmt::Write> EnterableTag<W> for TagTh {
            type Sink<'a> = ThSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                ThSink {
                    sink: out,
                }
            }
        }

        pub struct ThSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> ThSink<'a, W> {
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

        impl<'a, W: std::fmt::Write> Drop for ThSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</th>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for ThSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub mod tr {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::attribute_escape;
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagTr {
            class: Option<String>,
        }

        impl SinkableTag for TagTr {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(class) = self.class.as_ref() {
                    t.write_str("<tr class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<tr>")
                }
            }
        }

        impl TagTr {
            // TODO: these
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

        impl<W: std::fmt::Write> EnterableTag<W> for TagTr {
            type Sink<'a> = TrSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                TrSink {
                    sink: out,
                }
            }
        }

        pub struct TrSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> TrSink<'a, W> {
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

        impl<'a, W: std::fmt::Write> Drop for TrSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</tr>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for TrSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
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
                    t.write_str("<td class=\"").expect("ok");
                    t.write_str(class).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<td>")
                }
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
                    t.write_str("<a href=\"").expect("ok");
                    t.write_str(href).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<a>")
                }
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

    pub mod span {
        use crate::tag::{EnterableTag, Sink, SinkableTag};
        use crate::html_escape;

        #[derive(Clone)]
        pub struct TagSpan {
            style: Option<String>,
        }

        impl SinkableTag for TagSpan {
            fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result {
                if let Some(style) = self.style.as_ref() {
                    t.write_str("<span style=\"").expect("ok");
                    t.write_str(style).expect("ok");
                    t.write_str("\">")
                } else {
                    t.write_str("<span>")
                }
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

        impl<W: std::fmt::Write> EnterableTag<W> for TagSpan {
            type Sink<'a> = SpanSink<'a, W> where W: 'a;

            fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a> {
                self.write_opening_tag(out).expect("is ok");
                SpanSink {
                    sink: out,
                }
            }
        }

        pub struct SpanSink<'a, W: std::fmt::Write> {
            sink: &'a mut W,
        }

        impl<'a, W: std::fmt::Write> SpanSink<'a, W> {
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

        impl<'a, W: std::fmt::Write> Drop for SpanSink<'a, W> {
            fn drop(&mut self) {
                self.sink.write_str("</span>").expect("can write closing tag");
                self.sink.write_str("\n").expect("can write closing tag");
            }
        }

        impl<'a, W: std::fmt::Write> Sink<W> for SpanSink<'a, W> {
            fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
                tag.prepare_sink(&mut self.sink)
            }
        }
    }

    pub trait SinkableTag {
        fn write_opening_tag(&self, t: &mut impl std::fmt::Write) -> std::fmt::Result;
    }

    pub struct HtmlSink<'a, T: Write + ?Sized> {
        sink: &'a mut T,
    }

    impl<'a, T: Write + ?Sized> HtmlSink<'a, T> {
        pub fn root(sink: &'a mut T) -> Self {
            sink.write_str("<html>\n").expect("str");
            HtmlSink {
                sink,
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> Drop for HtmlSink<'a, T> {
        fn drop(&mut self) {
            // TODO: configurable pretty-printing
            self.sink.write_str("</html>").expect("newline");
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> HtmlSink<'a, T> {
        pub fn close(self) {
            std::mem::drop(self);
        }
    }




    pub trait Sink<W: std::fmt::Write> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b>;
    }

    pub trait EnterableTag<W: std::fmt::Write> {
        type Sink<'a> /*: Sink<W>*/ where W: 'a;

        fn prepare_sink<'a>(self, out: &'a mut W) -> Self::Sink<'a>;
    }

    impl<'a, W: std::fmt::Write> Sink<W> for HtmlSink<'a, W> {
        fn open_tag<'b, Tag: EnterableTag<W>>(&'b mut self, tag: Tag) -> Tag::Sink<'b> {
            tag.prepare_sink(&mut self.sink)
        }
    }

}

use tag::{
    style::style, a::a, p::p, h1::h1, td::td,
    th::th, tr::tr, span::span,
    table::table, HtmlSink
};
use tag::Sink;

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
