use crate::{into_attrs::IntoAttrs, into_elements::IntoElements};

use crate::element::Element;

macro_rules! define_tag_function {
    ($tag:ident) => {
        pub fn $tag<Attrs: IntoAttrs, Children: IntoElements>(
            attrs: Attrs,
            with: Children,
        ) -> Element {
            Element::Tag {
                tag: stringify!($tag),
                attrs: attrs.into_attrs(),
                children: with.into_elements(),
            }
        }
    };

    ($tag:ident, leaf) => {
        pub fn $tag<Attrs: IntoAttrs>(value: Attrs) -> Element {
            Element::LeafTag {
                tag: stringify!($tag),
                attrs: value.into_attrs(),
            }
        }
    };

    ($tag:ident, $tag_str:literal) => {
        pub fn $tag<Attrs: IntoAttrs, Children: IntoElements>(
            attrs: Attrs,
            with: Children,
        ) -> Element {
            Element::Tag {
                tag: $tag_str,
                attrs: attrs.into_attrs(),
                children: with.into_elements(),
            }
        }
    };

    ($tag:ident, $tag_str:literal, leaf) => {
        pub fn $tag<Attrs: IntoAttrs>(value: Attrs) -> Element {
            Element::LeafTag {
                tag: $tag_str,
                attrs: value.into_attrs(),
            }
        }
    };
}

define_tag_function!(a);
define_tag_function!(abbr);
define_tag_function!(address);
define_tag_function!(area, leaf);
define_tag_function!(article);
define_tag_function!(aside);
define_tag_function!(audio);
define_tag_function!(b);
define_tag_function!(base, leaf);
define_tag_function!(bdi);
define_tag_function!(bdo);
define_tag_function!(body);
define_tag_function!(blockquote);
define_tag_function!(br, leaf);
define_tag_function!(button);
define_tag_function!(canvas);
define_tag_function!(caption);
define_tag_function!(cite);
define_tag_function!(code);
define_tag_function!(col);
define_tag_function!(colgroup);
define_tag_function!(data);
define_tag_function!(datalist);
define_tag_function!(dd);
define_tag_function!(del);
define_tag_function!(details);
define_tag_function!(dfn);
define_tag_function!(dialog);
define_tag_function!(div);
define_tag_function!(dl);
define_tag_function!(dt);
define_tag_function!(em);
define_tag_function!(embed);
define_tag_function!(fieldset);
define_tag_function!(figcaption);
define_tag_function!(figure);
define_tag_function!(footer);
define_tag_function!(form);
define_tag_function!(h1);
define_tag_function!(h2);
define_tag_function!(h3);
define_tag_function!(h4);
define_tag_function!(h5);
define_tag_function!(h6);
define_tag_function!(header);
define_tag_function!(head);
define_tag_function!(hr, leaf);
define_tag_function!(html);
define_tag_function!(i);
define_tag_function!(iframe);
define_tag_function!(img, leaf);
define_tag_function!(input, leaf);
define_tag_function!(ins);
define_tag_function!(kbd);
define_tag_function!(label);
define_tag_function!(legend);
define_tag_function!(li);
define_tag_function!(link, leaf);
define_tag_function!(main_, "main");
define_tag_function!(map);
define_tag_function!(mark);
define_tag_function!(math);
define_tag_function!(menu);
define_tag_function!(menuitem);
define_tag_function!(meta, leaf);
define_tag_function!(meter);
define_tag_function!(nav);
define_tag_function!(noscript);
define_tag_function!(object);
define_tag_function!(ol);
define_tag_function!(optgroup);
define_tag_function!(option);
define_tag_function!(output);
define_tag_function!(p);
define_tag_function!(param);
define_tag_function!(picture);
define_tag_function!(pre);
define_tag_function!(progress);
define_tag_function!(q);
define_tag_function!(rp);
define_tag_function!(rt);
define_tag_function!(ruby);
define_tag_function!(s);
define_tag_function!(samp);
define_tag_function!(script);
define_tag_function!(section);
define_tag_function!(select);
define_tag_function!(small);
define_tag_function!(source, leaf);
define_tag_function!(span);
define_tag_function!(strong);
define_tag_function!(sub);
define_tag_function!(summary);
define_tag_function!(sup);
define_tag_function!(svg);
define_tag_function!(table);
define_tag_function!(tbody);
define_tag_function!(td);
define_tag_function!(textarea);
define_tag_function!(tfoot);
define_tag_function!(th);
define_tag_function!(thead);
define_tag_function!(time);
define_tag_function!(title_tag, "title");
define_tag_function!(tr);
define_tag_function!(track, leaf);
define_tag_function!(u);
define_tag_function!(ul);
define_tag_function!(var);
define_tag_function!(video);
define_tag_function!(wbr);
