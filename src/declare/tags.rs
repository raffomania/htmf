use crate::Element;

macro_rules! define_tag_function {
    ($tag:ident $(, leaf)*) => {
        pub fn $tag<'a>() -> Element<'a> {
            Element::Tag {
                tag: stringify!($tag),
                attrs: Vec::new(),
                children: Vec::new(),
            }
        }
    };

    ($tag:ident, $tag_str:literal) => {
        pub fn $tag<'a>() -> Element<'a> {
            Element::Tag {
                tag: $tag_str,
                attrs: Vec::new(),
                children: Vec::new(),
            }
        }
    };
}

/// Prepend `<!doctype html>` to the given children.
pub fn document<'a>() -> Element<'a> {
    Element::Document {
        children: Vec::new(),
    }
}

macro_rules! define_tag_method {
    ($tag:ident $(, leaf)*) => {
        pub fn $tag<'b>(&'b mut self) -> &'b mut Element<'a> {
            self.children_mut().push($tag());
            self.children_mut().last_mut().unwrap()
        }
    };
}

impl<'a> Element<'a> {
    pub(crate) fn children_mut<'b>(&'b mut self) -> &'b mut Vec<Element<'a>> {
        match self {
            Element::Tag {
                tag: _,
                attrs: _,
                ref mut children,
            } => children,
            Element::Fragment { ref mut children } => children,
            Element::Text(_) => todo!(),
            Element::Document { ref mut children } => children,
        }
    }

    define_tag_method!(a);
    define_tag_method!(abbr);
    define_tag_method!(address);
    define_tag_method!(area, leaf);
    define_tag_method!(article);
    define_tag_method!(aside);
    define_tag_method!(audio);
    define_tag_method!(b);
    define_tag_method!(base, leaf);
    define_tag_method!(bdi);
    define_tag_method!(bdo);
    define_tag_method!(body);
    define_tag_method!(blockquote);
    define_tag_method!(br, leaf);
    define_tag_method!(button);
    define_tag_method!(canvas);
    define_tag_method!(caption);
    define_tag_method!(cite);
    define_tag_method!(code);
    define_tag_method!(col);
    define_tag_method!(colgroup);
    define_tag_method!(data);
    define_tag_method!(datalist);
    define_tag_method!(dd);
    define_tag_method!(del);
    define_tag_method!(details);
    define_tag_method!(dfn);
    define_tag_method!(dialog);
    define_tag_method!(div);
    define_tag_method!(dl);
    define_tag_method!(dt);
    define_tag_method!(em);
    define_tag_method!(embed);
    define_tag_method!(fieldset);
    define_tag_method!(figcaption);
    define_tag_method!(figure);
    define_tag_method!(footer);
    define_tag_method!(form);
    define_tag_method!(h1);
    define_tag_method!(h2);
    define_tag_method!(h3);
    define_tag_method!(h4);
    define_tag_method!(h5);
    define_tag_method!(h6);
    define_tag_method!(header);
    define_tag_method!(head);
    define_tag_method!(hr, leaf);
    define_tag_method!(html);
    define_tag_method!(i);
    define_tag_method!(iframe);
    define_tag_method!(img, leaf);
    define_tag_method!(input, leaf);
    define_tag_method!(ins);
    define_tag_method!(kbd);
    define_tag_method!(label);
    define_tag_method!(legend);
    define_tag_method!(li);
    define_tag_method!(link, leaf);
    define_tag_method!(main_);
    define_tag_method!(map);
    define_tag_method!(mark);
    define_tag_method!(math);
    define_tag_method!(menu);
    define_tag_method!(menuitem);
    define_tag_method!(meta, leaf);
    define_tag_method!(meter);
    define_tag_method!(nav);
    define_tag_method!(noscript);
    define_tag_method!(object);
    define_tag_method!(ol);
    define_tag_method!(optgroup);
    define_tag_method!(option);
    define_tag_method!(output);
    define_tag_method!(p);
    define_tag_method!(param);
    define_tag_method!(picture);
    define_tag_method!(pre);
    define_tag_method!(progress);
    define_tag_method!(q);
    define_tag_method!(rp);
    define_tag_method!(rt);
    define_tag_method!(ruby);
    define_tag_method!(s);
    define_tag_method!(samp);
    define_tag_method!(script);
    define_tag_method!(section);
    define_tag_method!(select);
    define_tag_method!(small);
    define_tag_method!(source, leaf);
    define_tag_method!(span);
    define_tag_method!(strong);
    define_tag_method!(sub);
    define_tag_method!(summary);
    define_tag_method!(sup);
    define_tag_method!(svg);
    define_tag_method!(table);
    define_tag_method!(tbody);
    define_tag_method!(td);
    define_tag_method!(textarea);
    define_tag_method!(tfoot);
    define_tag_method!(th);
    define_tag_method!(thead);
    define_tag_method!(time);
    define_tag_method!(tr);
    define_tag_method!(track, leaf);
    define_tag_method!(u);
    define_tag_method!(ul);
    define_tag_method!(var);
    define_tag_method!(video);
    define_tag_method!(wbr);
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
