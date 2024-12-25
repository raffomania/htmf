use crate::attr::IntoAttrs;

#[cfg(feature = "unstable-builder")]
use crate::builder::Builder;

use crate::element::Element;

macro_rules! define_tag_function {
    ($tag:ident $(, leaf)*) => {
        pub fn $tag<Attrs: IntoAttrs>(value: Attrs) -> Element {
            Element::Tag {
                tag: stringify!($tag),
                attrs: value.into_attrs(),
                children: Vec::new(),
            }
        }
    };

    ($tag:ident, $tag_str:literal) => {
        pub fn $tag<Attrs: IntoAttrs>(value: Attrs) -> Element {
            Element::Tag {
                tag: $tag_str,
                attrs: value.into_attrs(),
                children: Vec::new(),
            }
        }
    };
}

#[cfg(feature = "unstable-builder")]
macro_rules! define_tag_builder_method {
    ($tag:ident $(, leaf)*) => {
        pub fn $tag<Attrs: IntoAttrs>(self, value: Attrs) -> Builder {
            self.into_new_child_tag(stringify!($tag), value.into_attrs())
        }
    };
    ($tag:ident, $tag_str:literal) => {
        pub fn $tag<Attrs: IntoAttrs>(self, value: Attrs) -> Builder {
            self.into_new_child_tag($tag_str, value.into_attrs())
        }
    };
}

#[cfg(feature = "unstable-builder")]
macro_rules! define_tag_element_method {
    ($tag:ident $(, leaf)*) => {
        pub fn $tag<Attrs: IntoAttrs>(self, value: Attrs) -> Builder {
            Builder::from(self).into_new_child_tag(stringify!($tag), value.into_attrs())
        }
    };
    ($tag:ident, $tag_str:literal) => {
        pub fn $tag<Attrs: IntoAttrs>(self, value: Attrs) -> Builder {
            Builder::from(self).into_new_child_tag($tag_str, value.into_attrs())
        }
    };
}

#[cfg(feature = "unstable-builder")]
impl Builder {
    define_tag_builder_method!(a);
    define_tag_builder_method!(abbr);
    define_tag_builder_method!(address);
    define_tag_builder_method!(area, leaf);
    define_tag_builder_method!(article);
    define_tag_builder_method!(aside);
    define_tag_builder_method!(audio);
    define_tag_builder_method!(b);
    define_tag_builder_method!(base, leaf);
    define_tag_builder_method!(bdi);
    define_tag_builder_method!(bdo);
    define_tag_builder_method!(body);
    define_tag_builder_method!(blockquote);
    define_tag_builder_method!(br, leaf);
    define_tag_builder_method!(button);
    define_tag_builder_method!(canvas);
    define_tag_builder_method!(caption);
    define_tag_builder_method!(cite);
    define_tag_builder_method!(code);
    define_tag_builder_method!(col);
    define_tag_builder_method!(colgroup);
    define_tag_builder_method!(data);
    define_tag_builder_method!(datalist);
    define_tag_builder_method!(dd);
    define_tag_builder_method!(del);
    define_tag_builder_method!(details);
    define_tag_builder_method!(dfn);
    define_tag_builder_method!(dialog);
    define_tag_builder_method!(div);
    define_tag_builder_method!(dl);
    define_tag_builder_method!(dt);
    define_tag_builder_method!(em);
    define_tag_builder_method!(embed);
    define_tag_builder_method!(fieldset);
    define_tag_builder_method!(figcaption);
    define_tag_builder_method!(figure);
    define_tag_builder_method!(footer);
    define_tag_builder_method!(form);
    define_tag_builder_method!(h1);
    define_tag_builder_method!(h2);
    define_tag_builder_method!(h3);
    define_tag_builder_method!(h4);
    define_tag_builder_method!(h5);
    define_tag_builder_method!(h6);
    define_tag_builder_method!(header);
    define_tag_builder_method!(head);
    define_tag_builder_method!(hr, leaf);
    define_tag_builder_method!(html);
    define_tag_builder_method!(i);
    define_tag_builder_method!(iframe);
    define_tag_builder_method!(img, leaf);
    define_tag_builder_method!(input, leaf);
    define_tag_builder_method!(ins);
    define_tag_builder_method!(kbd);
    define_tag_builder_method!(label);
    define_tag_builder_method!(legend);
    define_tag_builder_method!(li);
    define_tag_builder_method!(link, leaf);
    define_tag_builder_method!(main_, "main");
    define_tag_builder_method!(map);
    define_tag_builder_method!(mark);
    define_tag_builder_method!(math);
    define_tag_builder_method!(menu);
    define_tag_builder_method!(menuitem);
    define_tag_builder_method!(meta, leaf);
    define_tag_builder_method!(meter);
    define_tag_builder_method!(nav);
    define_tag_builder_method!(noscript);
    define_tag_builder_method!(object);
    define_tag_builder_method!(ol);
    define_tag_builder_method!(optgroup);
    define_tag_builder_method!(option);
    define_tag_builder_method!(output);
    define_tag_builder_method!(p);
    define_tag_builder_method!(param);
    define_tag_builder_method!(picture);
    define_tag_builder_method!(pre);
    define_tag_builder_method!(progress);
    define_tag_builder_method!(q);
    define_tag_builder_method!(rp);
    define_tag_builder_method!(rt);
    define_tag_builder_method!(ruby);
    define_tag_builder_method!(s);
    define_tag_builder_method!(samp);
    define_tag_builder_method!(script);
    define_tag_builder_method!(section);
    define_tag_builder_method!(select);
    define_tag_builder_method!(small);
    define_tag_builder_method!(source, leaf);
    define_tag_builder_method!(span);
    define_tag_builder_method!(strong);
    define_tag_builder_method!(sub);
    define_tag_builder_method!(summary);
    define_tag_builder_method!(sup);
    define_tag_builder_method!(svg);
    define_tag_builder_method!(table);
    define_tag_builder_method!(tbody);
    define_tag_builder_method!(td);
    define_tag_builder_method!(textarea);
    define_tag_builder_method!(tfoot);
    define_tag_builder_method!(th);
    define_tag_builder_method!(thead);
    define_tag_builder_method!(time);
    define_tag_builder_method!(tr);
    define_tag_builder_method!(track, leaf);
    define_tag_builder_method!(u);
    define_tag_builder_method!(ul);
    define_tag_builder_method!(var);
    define_tag_builder_method!(video);
    define_tag_builder_method!(wbr);
}

#[cfg(feature = "unstable-builder")]
impl Element {
    define_tag_element_method!(a);
    define_tag_element_method!(abbr);
    define_tag_element_method!(address);
    define_tag_element_method!(area, leaf);
    define_tag_element_method!(article);
    define_tag_element_method!(aside);
    define_tag_element_method!(audio);
    define_tag_element_method!(b);
    define_tag_element_method!(base, leaf);
    define_tag_element_method!(bdi);
    define_tag_element_method!(bdo);
    define_tag_element_method!(body);
    define_tag_element_method!(blockquote);
    define_tag_element_method!(br, leaf);
    define_tag_element_method!(button);
    define_tag_element_method!(canvas);
    define_tag_element_method!(caption);
    define_tag_element_method!(cite);
    define_tag_element_method!(code);
    define_tag_element_method!(col);
    define_tag_element_method!(colgroup);
    define_tag_element_method!(data);
    define_tag_element_method!(datalist);
    define_tag_element_method!(dd);
    define_tag_element_method!(del);
    define_tag_element_method!(details);
    define_tag_element_method!(dfn);
    define_tag_element_method!(dialog);
    define_tag_element_method!(div);
    define_tag_element_method!(dl);
    define_tag_element_method!(dt);
    define_tag_element_method!(em);
    define_tag_element_method!(embed);
    define_tag_element_method!(fieldset);
    define_tag_element_method!(figcaption);
    define_tag_element_method!(figure);
    define_tag_element_method!(footer);
    define_tag_element_method!(form);
    define_tag_element_method!(h1);
    define_tag_element_method!(h2);
    define_tag_element_method!(h3);
    define_tag_element_method!(h4);
    define_tag_element_method!(h5);
    define_tag_element_method!(h6);
    define_tag_element_method!(header);
    define_tag_element_method!(head);
    define_tag_element_method!(hr, leaf);
    define_tag_element_method!(html);
    define_tag_element_method!(i);
    define_tag_element_method!(iframe);
    define_tag_element_method!(img, leaf);
    define_tag_element_method!(input, leaf);
    define_tag_element_method!(ins);
    define_tag_element_method!(kbd);
    define_tag_element_method!(label);
    define_tag_element_method!(legend);
    define_tag_element_method!(li);
    define_tag_element_method!(link, leaf);
    define_tag_element_method!(main_, "main");
    define_tag_element_method!(map);
    define_tag_element_method!(mark);
    define_tag_element_method!(math);
    define_tag_element_method!(menu);
    define_tag_element_method!(menuitem);
    define_tag_element_method!(meta, leaf);
    define_tag_element_method!(meter);
    define_tag_element_method!(nav);
    define_tag_element_method!(noscript);
    define_tag_element_method!(object);
    define_tag_element_method!(ol);
    define_tag_element_method!(optgroup);
    define_tag_element_method!(option);
    define_tag_element_method!(output);
    define_tag_element_method!(p);
    define_tag_element_method!(param);
    define_tag_element_method!(picture);
    define_tag_element_method!(pre);
    define_tag_element_method!(progress);
    define_tag_element_method!(q);
    define_tag_element_method!(rp);
    define_tag_element_method!(rt);
    define_tag_element_method!(ruby);
    define_tag_element_method!(s);
    define_tag_element_method!(samp);
    define_tag_element_method!(script);
    define_tag_element_method!(section);
    define_tag_element_method!(select);
    define_tag_element_method!(small);
    define_tag_element_method!(source, leaf);
    define_tag_element_method!(span);
    define_tag_element_method!(strong);
    define_tag_element_method!(sub);
    define_tag_element_method!(summary);
    define_tag_element_method!(sup);
    define_tag_element_method!(svg);
    define_tag_element_method!(table);
    define_tag_element_method!(tbody);
    define_tag_element_method!(td);
    define_tag_element_method!(textarea);
    define_tag_element_method!(tfoot);
    define_tag_element_method!(th);
    define_tag_element_method!(thead);
    define_tag_element_method!(time);
    define_tag_element_method!(tr);
    define_tag_element_method!(track, leaf);
    define_tag_element_method!(u);
    define_tag_element_method!(ul);
    define_tag_element_method!(var);
    define_tag_element_method!(video);
    define_tag_element_method!(wbr);
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
define_tag_function!(tr);
define_tag_function!(track, leaf);
define_tag_function!(u);
define_tag_function!(ul);
define_tag_function!(var);
define_tag_function!(video);
define_tag_function!(wbr);
