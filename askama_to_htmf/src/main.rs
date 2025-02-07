use std::path::{Path, PathBuf};

use anyhow::Result;
use tl::{HTMLTag, Node, Parser};

#[cfg(test)]
mod tests;

#[derive(clap::Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();
    let code = convert_file(&args.input)?;
    print!("{code}");

    Ok(())
}

fn convert_file(file: &Path) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(file)?;

    let ast = tl::parse(&contents, tl::ParserOptions::default())?;
    let parser = ast.parser();

    let mut result = "use htmf::prelude::*;\n\npub fn view(data: &Data) -> Element {".to_string();
    if ast.children().len() > 1 {
        result.push_str("fragment().with([");
    }
    for child in ast.children() {
        let Some(child) = child.get(parser) else {
            continue;
        };
        if let Some(cc) = convert_node(child, parser, ast.children().len() == 1)? {
            result.push_str(&cc);
            result.push_str(",\n");
        }
    }

    if ast.children().len() > 1 {
        result.push_str("])\n}");
    }

    Ok(result)
}

fn convert_node(child: &Node, parser: &Parser, only_child: bool) -> Result<Option<String>> {
    let code = match child {
        tl::Node::Tag(tag) => Some(convert_tag(tag, parser)?),
        tl::Node::Raw(bytes) => {
            let s = bytes.as_utf8_str();
            let s = s.trim();
            if s.is_empty() {
                return Ok(None);
            }
            let s = if s.contains("\"") {
                format!("r#\"{s}\"#")
            } else {
                format!("\"{s}\"")
            };

            if only_child {
                Some(s)
            } else {
                Some(format!("text({s})"))
            }
        }
        tl::Node::Comment(bytes) => Some(format!("\n// {}", bytes.as_utf8_str())),
    };

    Ok(code)
}

fn convert_tag(tag: &HTMLTag, parser: &Parser) -> Result<String> {
    let mut attr_code = String::new();
    let many_attrs = tag.attributes().len() > 1;
    if many_attrs {
        attr_code.push('[');
    }
    let mut sorted_attributes: Vec<_> = tag.attributes().iter().collect();
    sorted_attributes.sort();

    for (key, val) in sorted_attributes {
        let attr_fn = convert_attr(&key, val.as_deref());
        attr_code.push_str(&attr_fn);
        if many_attrs {
            attr_code.push(',');
        }
    }
    if many_attrs {
        attr_code.push(']');
    }

    let mut code = format!("{}({attr_code})", tag.name().as_utf8_str());

    let multiple_children = tag.children().top().len() > 1;
    let mut child_code = String::new();
    for child in tag.children().top().iter() {
        if let Some(cc) = convert_node(child.get(parser).unwrap(), parser, !multiple_children)? {
            child_code.push_str(&cc);
            if multiple_children {
                child_code.push_str(",\n");
            }
        }
    }
    if !child_code.is_empty() {
        if multiple_children {
            child_code = format!(".with([{child_code}])");
        } else {
            child_code = format!(".with({child_code})");
        }
    }

    code.push_str(&child_code);

    Ok(code)
}

fn convert_attr(name: &str, value: Option<&str>) -> String {
    let known = match name {
        "accept" => Some("accept"),
        "accept_charset" => Some("accept_charset"),
        "action" => Some("action"),
        "alt" => Some("alt"),
        "aria_checked" => Some("aria_checked"),
        "aria_current" => Some("aria_current"),
        "aria_disabled" => Some("aria_disabled"),
        "aria_hidden" => Some("aria_hidden"),
        "aria_invalid" => Some("aria_invalid"),
        "aria_label" => Some("aria_label"),
        "aria_labelledby" => Some("aria_labelledby"),
        "aria_placeholder" => Some("aria_placeholder"),
        "aria_readonly" => Some("aria_readonly"),
        "aria_required" => Some("aria_required"),
        "async" => Some("async_"),
        "autocapitalize" => Some("autocapitalize"),
        "autocomplete" => Some("autocomplete"),
        "autofocus" => Some("autofocus"),
        "autoplay" => Some("autoplay"),
        "capture" => Some("capture"),
        "charset" => Some("charset"),
        "checked" => Some("checked"),
        "cite_attr" => Some("cite_attr"),
        "class" => Some("class"),
        "content" => Some("content"),
        "contenteditable" => Some("contenteditable"),
        "crossorigin" => Some("crossorigin"),
        "defer" => Some("defer"),
        "disabled" => Some("disabled"),
        "draggable" => Some("draggable"),
        "enctype" => Some("enctype"),
        "for" => Some("for_"),
        "formaction" => Some("formaction"),
        "height" => Some("height"),
        "href" => Some("href"),
        "http_equiv" => Some("http_equiv"),
        "id" => Some("id"),
        "integrity" => Some("integrity"),
        "lang" => Some("lang"),
        "loop" => Some("loop_"),
        "maxlength" => Some("maxlength"),
        "method" => Some("method"),
        "minlength" => Some("minlength"),
        "name" => Some("name"),
        "placeholder" => Some("placeholder"),
        "preload" => Some("preload"),
        "property" => Some("property"),
        "readonly" => Some("readonly"),
        "rel" => Some("rel"),
        "required" => Some("required"),
        "role" => Some("role"),
        "selected" => Some("selected"),
        "src" => Some("src"),
        "style" => Some("style"),
        "tabindex" => Some("tabindex"),
        "target" => Some("target"),
        "title_attr" => Some("title"),
        "type" => Some("type_"),
        "value" => Some("value"),
        "width" => Some("width"),
        _ => None,
    };

    let val = value.unwrap_or("");
    if let Some(name) = known {
        format!("{name}(\"{val}\")")
    } else {
        format!("attr(\"{name}\", \"{val}\")")
    }
}
