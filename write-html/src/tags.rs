/*!
Provides functions for creating common tags.
*/

use crate::{Attributes, Html, Compactability, Sum, AttributeName, AttributeValue, HtmlEnv, Empty};


/// Represents a tag.
///
/// TODO better docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag<'t, A: Attributes, I: Html, const SILENT: bool = false> { // TODO maybe with another type or an option
    tag: &'t str,
    attributes: A,
    inner_html: I,
    compactability: Compactability,
}

impl<'n, A: Attributes, I: Html, const SILENT: bool> Tag<'n, A, I, SILENT> {
    /// Adds a child to the tag.
    ///
    /// # Arguments
    /// * `child` - The child to add.
    ///
    /// # Example
    /// ```
    /// use write_html::*;
    /// use std::fmt::Write;
    ///
    /// let mut s = String::new();
    /// s.with(tags::div(Empty, Empty)
    ///     .child(tags::p(Empty, Empty)
    ///         .child("Hello, world!".as_html())
    ///     )
    /// ).unwrap();
    /// assert_eq!(s, "<div><p>Hello, world!</p></div>");
    /// ```
    pub fn child<C: Html>(
        self,
        child: C,
    ) -> Tag<'n, A, Sum<I, C>> {
        Tag {
            tag: self.tag,
            attributes: self.attributes,
            inner_html: Sum(self.inner_html, child),
            compactability: self.compactability,
        }
    }

    /// Adds multiple attributes to the tag.
    ///
    /// # Arguments
    /// * `attributes` - The attributes to add.
    ///
    /// # Example
    /// ```
    /// use write_html::*;
    /// use std::fmt::Write;
    ///
    /// let mut s = String::new();
    /// s.with(tags::div(Empty, Empty)
    ///     .attributes([
    ///         ("class", "container"),
    ///         ("id", "main"),
    ///     ])
    /// ).unwrap();
    /// assert_eq!(s, "<div class=\"container\" id=\"main\"></div>");
    /// ```
    pub fn attributes<B: Attributes>(
        self,
        attributes: B,
    ) -> Tag<'n, Sum<A, B>, I> {
        Tag {
            tag: self.tag,
            attributes: Sum(self.attributes, attributes),
            inner_html: self.inner_html,
            compactability: self.compactability,
        }
    }

    /// Adds an attribute to the tag.
    pub fn attr<Name: AttributeName, Value: AttributeValue>(
        self,
        name: Name,
        value: Value,
    ) -> Tag<'n, Sum<A, [(Name, Value); 1]>, I> {
        Tag {
            tag: self.tag,
            attributes: Sum(self.attributes, [(name, value)]),
            inner_html: self.inner_html,
            compactability: self.compactability,
        }
    }
}

impl<'n, A: Attributes, I: Html, const SILENT: bool> Html for Tag<'n, A, I, SILENT> {
    fn write_html(self, env: &mut impl crate::HtmlEnv) -> std::fmt::Result {
        if SILENT {
            if !self.inner_html.is_unit() {
                env.with(self.inner_html).map(|_| ())
            } else {
                Ok(())
            }
        } else {
            if self.inner_html.is_unit() {
                env
                    .tag(self.tag, self.compactability)?
                    .with_attributes(self.attributes)
                    .map(|_| ())
            } else {
                env
                    .tag(self.tag, self.compactability)?
                    .with_attributes(self.attributes)?
                    .inner_html()?
                    .with(self.inner_html)
                    .map(|_| ())
            }
        }
    }
}

// TODO comment tag

// TODO see https://www.w3schools.com/tags/

/// Creates a new custom tag.
///
/// See [`Tag`] for more information.
///
/// # Arguments
/// * `tag` - The name of the tag.
/// * `attributes` - The attributes of the tag.
/// * `inner_html` - The inner HTML of the tag.
/// * `compactability` - Whether the tag can be compacted.
pub fn tag<'t, A: Attributes, I: Html>(
    tag: &'t str,
    attributes: A,
    inner_html: I,
    compactability: Compactability,
) -> Tag<'t, A, I> {
    Tag {
        tag,
        attributes,
        inner_html,
        compactability,
    }
}

pub fn silent_tag<I: Html>(
    inner_html: I,
) -> Tag<'static, Empty, I> {
    Tag {
        tag: "",
        attributes: Empty,
        inner_html,
        compactability: Compactability::No,
    }
}

// see https://stackoverflow.com/questions/41361897/documenting-a-function-created-with-a-macro-in-rust
macro_rules! define_tag {
    ($(#[$attr:meta])* $tag:ident $compactability:expr) => {
/// Creates a new
$(#[$attr])*
/// tag.
///
/// See [`tag`] for more information.
pub fn $tag(
    attributes: impl Attributes,
    inner_html: impl Html,
) -> Tag<'static, impl Attributes, impl Html> {
    tag(
        stringify!($tag),
        attributes,
        inner_html,
        $compactability,
    )
}
    };
}



define_tag!(
    /// [`<a>`](https://www.w3schools.com/tags/tag_a.asp)
    a
    Compactability::No
);

define_tag!(
    /// [`<abbr>`](https://www.w3schools.com/tags/tag_abbr.asp)
    abbr
    Compactability::No
);

define_tag!(
    /// [`<address>`](https://www.w3schools.com/tags/tag_address.asp)
    address
    Compactability::No
);

define_tag!(
    /// [`<area>`](https://www.w3schools.com/tags/tag_area.asp)
    area
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<article>`](https://www.w3schools.com/tags/tag_article.asp)
    article
    Compactability::No
);

define_tag!(
    /// [`<aside>`](https://www.w3schools.com/tags/tag_aside.asp)
    aside
    Compactability::No
);

define_tag!(
    /// [`<audio>`](https://www.w3schools.com/tags/tag_audio.asp)
    audio
    Compactability::No
);

define_tag!(
    /// [`<b>`](https://www.w3schools.com/tags/tag_b.asp)
    b
    Compactability::No
);

define_tag!(
    /// [`<base>`](https://www.w3schools.com/tags/tag_base.asp)
    base
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<bdi>`](https://www.w3schools.com/tags/tag_bdi.asp)
    bdi
    Compactability::No
);

define_tag!(
    /// [`<bdo>`](https://www.w3schools.com/tags/tag_bdo.asp)
    bdo
    Compactability::No
);

define_tag!(
    /// [`<blockquote>`](https://www.w3schools.com/tags/tag_blockquote.asp)
    blockquote
    Compactability::No
);

define_tag!(
    /// [`<body>`](https://www.w3schools.com/tags/tag_body.asp)
    body
    Compactability::No
);

define_tag!(
    /// [`<br>`](https://www.w3schools.com/tags/tag_br.asp)
    br
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<button>`](https://www.w3schools.com/tags/tag_button.asp)
    button
    Compactability::No
);

define_tag!(
    /// [`<canvas>`](https://www.w3schools.com/tags/tag_canvas.asp)
    canvas
    Compactability::No
);

define_tag!(
    /// [`<caption>`](https://www.w3schools.com/tags/tag_caption.asp)
    caption
    Compactability::No
);

define_tag!(
    /// [`<cite>`](https://www.w3schools.com/tags/tag_cite.asp)
    cite
    Compactability::No
);

define_tag!(
    /// [`<code>`](https://www.w3schools.com/tags/tag_code.asp)
    code
    Compactability::No
);

define_tag!(
    /// [`<col>`](https://www.w3schools.com/tags/tag_col.asp)
    col
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<colgroup>`](https://www.w3schools.com/tags/tag_colgroup.asp)
    colgroup
    Compactability::No
);

define_tag!(
    /// [`<data>`](https://www.w3schools.com/tags/tag_data.asp)
    data
    Compactability::No
);

define_tag!(
    /// [`<datalist>`](https://www.w3schools.com/tags/tag_datalist.asp)
    datalist
    Compactability::No
);

define_tag!(
    /// [`<dd>`](https://www.w3schools.com/tags/tag_dd.asp)
    dd
    Compactability::No
);

define_tag!(
    /// [`<del>`](https://www.w3schools.com/tags/tag_del.asp)
    del
    Compactability::No
);

define_tag!(
    /// [`<details>`](https://www.w3schools.com/tags/tag_details.asp)
    details
    Compactability::No
);

define_tag!(
    /// [`<dfn>`](https://www.w3schools.com/tags/tag_dfn.asp)
    dfn
    Compactability::No
);

define_tag!(
    /// [`<dialog>`](https://www.w3schools.com/tags/tag_dialog.asp)
    dialog
    Compactability::No
);

define_tag!(
    /// [`<div>`](https://www.w3schools.com/tags/tag_div.asp)
    div
    Compactability::No
);

define_tag!(
    /// [`<dl>`](https://www.w3schools.com/tags/tag_dl.asp)
    dl
    Compactability::No
);

define_tag!(
    /// [`<dt>`](https://www.w3schools.com/tags/tag_dt.asp)
    dt
    Compactability::No
);

define_tag!(
    /// [`<em>`](https://www.w3schools.com/tags/tag_em.asp)
    em
    Compactability::No
);

define_tag!(
    /// [`<embed>`](https://www.w3schools.com/tags/tag_embed.asp)
    embed
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<fieldset>`](https://www.w3schools.com/tags/tag_fieldset.asp)
    fieldset
    Compactability::No
);

define_tag!(
    /// [`<figcaption>`](https://www.w3schools.com/tags/tag_figcaption.asp)
    figcaption
    Compactability::No
);

define_tag!(
    /// [`<figure>`](https://www.w3schools.com/tags/tag_figure.asp)
    figure
    Compactability::No
);

define_tag!(
    /// [`<footer>`](https://www.w3schools.com/tags/tag_footer.asp)
    footer
    Compactability::No
);

define_tag!(
    /// [`<form>`](https://www.w3schools.com/tags/tag_form.asp)
    form
    Compactability::No
);

define_tag!(
    /// [`<h1>`](https://www.w3schools.com/tags/tag_hn.asp)
    h1
    Compactability::No
);

define_tag!(
    /// [`<h2>`](https://www.w3schools.com/tags/tag_hn.asp)
    h2
    Compactability::No
);

define_tag!(
    /// [`<h3>`](https://www.w3schools.com/tags/tag_hn.asp)
    h3
    Compactability::No
);

define_tag!(
    /// [`<h4>`](https://www.w3schools.com/tags/tag_hn.asp)
    h4
    Compactability::No
);

define_tag!(
    /// [`<h5>`](https://www.w3schools.com/tags/tag_hn.asp)
    h5
    Compactability::No
);

define_tag!(
    /// [`<h6>`](https://www.w3schools.com/tags/tag_hn.asp)
    h6
    Compactability::No
);

define_tag!(
    /// [`<head>`](https://www.w3schools.com/tags/tag_head.asp)
    head
    Compactability::No
);

define_tag!(
    /// [`<header>`](https://www.w3schools.com/tags/tag_header.asp)
    header
    Compactability::No
);

define_tag!(
    /// [`<hgroup>`](https://www.w3schools.com/tags/tag_hgroup.asp)
    hgroup
    Compactability::No
);

define_tag!(
    /// [`<hr>`](https://www.w3schools.com/tags/tag_hr.asp)
    hr
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<html>`](https://www.w3schools.com/tags/tag_html.asp)
    html
    Compactability::No
);

define_tag!(
    /// [`<i>`](https://www.w3schools.com/tags/tag_i.asp)
    i
    Compactability::No
);

define_tag!(
    /// [`<iframe>`](https://www.w3schools.com/tags/tag_iframe.asp)
    iframe
    Compactability::No
);

define_tag!(
    /// [`<img>`](https://www.w3schools.com/tags/tag_img.asp)
    img
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<input>`](https://www.w3schools.com/tags/tag_input.asp)
    input
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<ins>`](https://www.w3schools.com/tags/tag_ins.asp)
    ins
    Compactability::No
);

define_tag!(
    /// [`<kbd>`](https://www.w3schools.com/tags/tag_kbd.asp)
    kbd
    Compactability::No
);

define_tag!(
    /// [`<label>`](https://www.w3schools.com/tags/tag_label.asp)
    label
    Compactability::No
);

define_tag!(
    /// [`<legend>`](https://www.w3schools.com/tags/tag_legend.asp)
    legend
    Compactability::No
);

define_tag!(
    /// [`<li>`](https://www.w3schools.com/tags/tag_li.asp)
    li
    Compactability::No
);

define_tag!(
    /// [`<link>`](https://www.w3schools.com/tags/tag_link.asp)
    link
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<main>`](https://www.w3schools.com/tags/tag_main.asp)
    main
    Compactability::No
);

define_tag!(
    /// [`<map>`](https://www.w3schools.com/tags/tag_map.asp)
    map
    Compactability::No
);

define_tag!(
    /// [`<mark>`](https://www.w3schools.com/tags/tag_mark.asp)
    mark
    Compactability::No
);

define_tag!(
    /// [`<meta>`](https://www.w3schools.com/tags/tag_meta.asp)
    meta
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<nav>`](https://www.w3schools.com/tags/tag_nav.asp)
    nav
    Compactability::No
);

define_tag!(
    /// [`<noscript>`](https://www.w3schools.com/tags/tag_noscript.asp)
    noscript
    Compactability::No
);

define_tag!(
    /// [`<object>`](https://www.w3schools.com/tags/tag_object.asp)
    object
    Compactability::No
);

define_tag!(
    /// [`<ol>`](https://www.w3schools.com/tags/tag_ol.asp)
    ol
    Compactability::No
);

define_tag!(
    /// [`<optgroup>`](https://www.w3schools.com/tags/tag_optgroup.asp)
    optgroup
    Compactability::No
);

define_tag!(
    /// [`<option>`](https://www.w3schools.com/tags/tag_option.asp)
    option
    Compactability::No
);

define_tag!(
    /// [`<output>`](https://www.w3schools.com/tags/tag_output.asp)
    output
    Compactability::No
);

define_tag!(
    /// [`<p>`](https://www.w3schools.com/tags/tag_p.asp)
    p
    Compactability::No
);

define_tag!(
    /// [`<param>`](https://www.w3schools.com/tags/tag_param.asp)
    param
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<pre>`](https://www.w3schools.com/tags/tag_pre.asp)
    pre
    Compactability::No
);

define_tag!(
    /// [`<progress>`](https://www.w3schools.com/tags/tag_progress.asp)
    progress
    Compactability::No
);

define_tag!(
    /// [`<q>`](https://www.w3schools.com/tags/tag_q.asp)
    q
    Compactability::No
);

define_tag!(
    /// [`<rp>`](https://www.w3schools.com/tags/tag_rp.asp)
    rp
    Compactability::No
);

define_tag!(
    /// [`<rt>`](https://www.w3schools.com/tags/tag_rt.asp)
    rt
    Compactability::No
);

define_tag!(
    /// [`<ruby>`](https://www.w3schools.com/tags/tag_ruby.asp)
    ruby
    Compactability::No
);

define_tag!(
    /// [`<s>`](https://www.w3schools.com/tags/tag_s.asp)
    s
    Compactability::No
);

define_tag!(
    /// [`<samp>`](https://www.w3schools.com/tags/tag_samp.asp)
    samp
    Compactability::No
);

define_tag!(
    /// [`<script>`](https://www.w3schools.com/tags/tag_script.asp)
    script
    Compactability::No
);

define_tag!(
    /// [`<section>`](https://www.w3schools.com/tags/tag_section.asp)
    section
    Compactability::No
);

define_tag!(
    /// [`<select>`](https://www.w3schools.com/tags/tag_select.asp)
    select
    Compactability::No
);

define_tag!(
    /// [`<small>`](https://www.w3schools.com/tags/tag_small.asp)
    small
    Compactability::No
);

define_tag!(
    /// [`<source>`](https://www.w3schools.com/tags/tag_source.asp)
    source
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<span>`](https://www.w3schools.com/tags/tag_span.asp)
    span
    Compactability::No
);

define_tag!(
    /// [`<strong>`](https://www.w3schools.com/tags/tag_strong.asp)
    strong
    Compactability::No
);

define_tag!(
    /// [`<style>`](https://www.w3schools.com/tags/tag_style.asp)
    style
    Compactability::No
);

define_tag!(
    /// [`<sub>`](https://www.w3schools.com/tags/tag_sub.asp)
    sub
    Compactability::No
);

define_tag!(
    /// [`<summary>`](https://www.w3schools.com/tags/tag_summary.asp)
    summary
    Compactability::No
);

define_tag!(
    /// [`<sup>`](https://www.w3schools.com/tags/tag_sup.asp)
    sup
    Compactability::No
);

define_tag!(
    /// [`<table>`](https://www.w3schools.com/tags/tag_table.asp)
    table
    Compactability::No
);

define_tag!(
    /// [`<tbody>`](https://www.w3schools.com/tags/tag_tbody.asp)
    tbody
    Compactability::No
);

define_tag!(
    /// [`<td>`](https://www.w3schools.com/tags/tag_td.asp)
    td
    Compactability::No
);

define_tag!(
    /// [`<template>`](https://www.w3schools.com/tags/tag_template.asp)
    template
    Compactability::No
);

define_tag!(
    /// [`<textarea>`](https://www.w3schools.com/tags/tag_textarea.asp)
    textarea
    Compactability::No
);

define_tag!(
    /// [`<tfoot>`](https://www.w3schools.com/tags/tag_tfoot.asp)
    tfoot
    Compactability::No
);

define_tag!(
    /// [`<th>`](https://www.w3schools.com/tags/tag_th.asp)
    th
    Compactability::No
);

define_tag!(
    /// [`<thead>`](https://www.w3schools.com/tags/tag_thead.asp)
    thead
    Compactability::No
);

define_tag!(
    /// [`<time>`](https://www.w3schools.com/tags/tag_time.asp)
    time
    Compactability::No
);

define_tag!(
    /// [`<title>`](https://www.w3schools.com/tags/tag_title.asp)
    title
    Compactability::No
);

define_tag!(
    /// [`<tr>`](https://www.w3schools.com/tags/tag_tr.asp)
    tr
    Compactability::No
);

define_tag!(
    /// [`<track>`](https://www.w3schools.com/tags/tag_track.asp)
    track
    Compactability::Yes { final_slash: false }
);

define_tag!(
    /// [`<u>`](https://www.w3schools.com/tags/tag_u.asp)
    u
    Compactability::No
);

define_tag!(
    /// [`<ul>`](https://www.w3schools.com/tags/tag_ul.asp)
    ul
    Compactability::No
);

define_tag!(
    /// [`<var>`](https://www.w3schools.com/tags/tag_var.asp)
    var
    Compactability::No
);

define_tag!(
    /// [`<video>`](https://www.w3schools.com/tags/tag_video.asp)
    video
    Compactability::No
);

define_tag!(
    /// [`<wbr>`](https://www.w3schools.com/tags/tag_wbr.asp)
    wbr
    Compactability::Yes { final_slash: false }
);