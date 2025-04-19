fn main() {
    println!("{}", example().expect("is ok"));
}

mod tag {
    use std::fmt::Write;

    #[derive(Clone)]
    pub struct TagP {}
    #[derive(Clone)]
    pub struct TagStyle {}
    #[derive(Clone)]
    pub struct TagH1 {}
    #[derive(Clone)]
    pub struct TagTable {}
    #[derive(Clone)]
    pub struct TagTh {}

    trait SinkableTag {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result;
        fn closing_tag(&self) -> &'static str;
    }

    impl SinkableTag for TagP {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result {
            t.write_str("<p>")
        }
        fn closing_tag(&self) -> &'static str {
            "</p>"
        }
    }

    impl SinkableTag for TagStyle {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result {
            t.write_str("<style>")
        }
        fn closing_tag(&self) -> &'static str {
            "</style>"
        }
    }

    impl SinkableTag for TagH1 {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result {
            t.write_str("<h1>")
        }
        fn closing_tag(&self) -> &'static str {
            "</h1>"
        }
    }

    impl SinkableTag for TagTable {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result {
            t.write_str("<table>")
        }
        fn closing_tag(&self) -> &'static str {
            "</table>"
        }
    }

    impl SinkableTag for TagTh {
        fn write_opening_tag(&self, t: &mut impl Write) -> std::fmt::Result {
            t.write_str("<th>")
        }
        fn closing_tag(&self) -> &'static str {
            "</th>"
        }
    }

    impl TagTh {
        pub fn class(mut self, text: &str) -> Self {
            self
        }

        pub fn text(mut self, text: &str) -> Self {
            self
        }
    }

    pub fn th() -> TagTh {
        TagTh {}
    }

    pub fn table() -> TagTable {
        TagTable {}
    }

    pub fn h1() -> TagH1 {
        TagH1 {}
    }

    pub fn style() -> TagStyle {
        TagStyle {}
    }

    pub fn p() -> TagP {
        TagP {}
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

    pub struct StyleSink<'a, T: Write + ?Sized> {
        sink: &'a mut T,
    }

    impl<'a, T: std::fmt::Write + ?Sized> StyleSink<'a, T> {
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

    fn css_escape(s: &str) -> String {
        s
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
    }

    fn html_escape(s: &str) -> String {
        s
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
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
        pub fn append<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> HtmlSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            HtmlSink {
                sink: &mut self.sink,
                closing_tag: tag.closing_tag(),
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> RowSink<'a, T> {
        pub fn append<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> RowSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            RowSink {
                sink: &mut self.sink,
            }
        }
    }

    impl<'a, T: std::fmt::Write + ?Sized> HtmlSink<'a, T> {
        pub fn append<'b, Tag: SinkableTag>(&'b mut self, tag: Tag) -> HtmlSink<'b, T> {
            tag.write_opening_tag(&mut self.sink).expect("opening tag");
            HtmlSink {
                sink: &mut self.sink,
                closing_tag: tag.closing_tag(),
            }
        }

        pub fn text(&mut self, text: &str) -> std::fmt::Result {
            // TODO: allow a type param to indicate escapedness, escape here..
            self.sink.write_str(&html_escape(text))
        }

        pub fn rule(&mut self, rule: &str) -> std::fmt::Result {
            self.sink.write_str(&css_escape(rule))?;
            // TODO: document pretty-printing makes this optional
            self.sink.write_str("\n")?;
            Ok(())
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
}

use tag::{style, p, h1, th, table, HtmlSink};

pub fn example() -> Result<String, std::fmt::Error> {
    let repos: Vec<String> = Vec::new();

    let mut out = String::new();

    let mut html = HtmlSink::root(&mut out);

    {
        let mut style = html.append(style());
        style.rule(".build-table { font-family: monospace; border: 1px solid black; }")?;
        style.rule(".odd-row { background: #eee; }")?;
        style.rule(".even-row { background: #ddd; }")?;
    }

    {
        let mut header = html.append(h1());
        header.text("builds and build accessories")?;
    }

    {
        let mut description = html.append(p());
        match repos.len() {
            0 => description.text("no repos configured & so there are no builds"),
            1 => description.text("1 repo configured"),
            o => description.text(&format!("{} repos configured", o)),
        };
    }

    let mut table = html.append(table());
    {
        let mut header = table.header();
        let th = th().class("row-item");
        let mut add_cell = |name: &'static str| header.append(th.clone()).text(name);
        add_cell("repo")?;
        add_cell("last build")?;
        add_cell("commit/job")?;
        add_cell("remote")?;
        add_cell("duration")?;
        add_cell("status")?;
        add_cell("result")?;
    }

    for repo in repos.into_iter() {
//        for remote in ctx.remotes_by_repo(repo.id);
        let row = table.row();
    }

    table.close();

    html.close();

    Ok(out)
}
