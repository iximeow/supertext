# supertext

hypertext, a bit less hyper

i want to construct HTML documents in some Rust code. i don't want a full
templating solution. i just want to append some tags and not forget to escape
user-provided input.

## example

TODO: !! 

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
