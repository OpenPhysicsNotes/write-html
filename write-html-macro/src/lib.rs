
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Ident, Delimiter, Group, Literal};

/*#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {

    dbg!(item);
    dbg!(32);
    //return item;

    //panic!("ciao");
    //println!("ciao");

    "fn answer() -> u32 { 42 }".parse().unwrap()
}*/

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {

    let tokens: Vec<TokenTree> = item.into_iter().collect();

    if false {
        for token in &tokens {
            match token {
                TokenTree::Ident(ident) => {
                    println!("Ident: {}", ident);
                }
                TokenTree::Literal(literal) => {
                    println!("Literal: {}", literal);
                }
                TokenTree::Punct(punct) => {
                    println!("Punct: {}", punct);
                }
                TokenTree::Group(group) => {
                    println!("Group: {}", group);
                }
            }
        }
    }

    let elements = parse_mutliple_elements(&tokens);

    if elements.is_none() {
        panic!("Invalid HTML syntax");
    }

    let elements = elements.unwrap();

    //dbg!(element);

    fn elements_to_token_stream(elements: &[Element]) -> TokenStream {

        let elements: proc_macro2::TokenStream = elements_to_with_token_stream(&elements).into();

        quote::quote!(
            ::write_html::tags::fragment(::write_html::Empty)
                #elements
        ).into()
    }

    fn elements_to_with_token_stream(elements: &[Element]) -> TokenStream {
        let mut tokens = Vec::new();
        for element in elements {
            let element: proc_macro2::TokenStream = element_to_token_stream(element).into();
            let new_tokens: TokenStream = quote::quote!(
                .child(#element)
            ).into();
            tokens.extend(new_tokens.into_iter());
        }

        tokens.into_iter().collect()
    }

    fn element_to_token_stream(element: &Element) -> TokenStream {
        let mut tokens = Vec::new();

        match element {
            Element::Expression(group) => {
                tokens.extend(group.clone());
            }
            Element::Literal(literal) => {
                let literal: proc_macro2::TokenStream = TokenStream::from(TokenTree::Literal(literal.clone())).into();
                let new_tokens: TokenStream = quote::quote!(
                    ::write_html::HtmlTextStr(#literal)
                ).into();
                tokens.extend(new_tokens);
            }
            Element::Tag(tag) => {
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)));
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Alone)));
                tokens.push(TokenTree::Ident(Ident::new("write_html", tag.identifier_span)));
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)));
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Alone)));
                tokens.push(TokenTree::Ident(Ident::new("tags", tag.identifier_span)));
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)));
                tokens.push(TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Alone)));
                if is_ident(&tag.identifier) {
                    tokens.push(TokenTree::Ident(Ident::new(&tag.identifier, tag.identifier_span)));
                    {
                        let args = "(::write_html::Empty, ::write_html::Empty)";
                        let token_stream: TokenStream = args.parse().unwrap();
                        tokens.extend(token_stream.into_iter());
                    }
                } else {
                    tokens.push(TokenTree::Ident(Ident::new("tag", tag.identifier_span)));
                    {
                        let mut args = Vec::new();
                        args.push(TokenTree::Literal(Literal::string(&tag.identifier)));
                        let token_stream: TokenStream = ", ::write_html::Empty, ::write_html::Empty, ::write_html::Compactability::No".parse().unwrap();
                        args.extend(token_stream.into_iter());
                        let group = Group::new(Delimiter::Parenthesis, args.into_iter().collect());
                        tokens.push(TokenTree::Group(group));
                    }
                }
                for (key, value) in &tag.attributes {
                    tokens.push(TokenTree::Punct(proc_macro::Punct::new('.', proc_macro::Spacing::Alone)));
                    tokens.push(TokenTree::Ident(Ident::new("attr", tag.identifier_span)));
                    {
                        let mut args = Vec::new();
                        args.push(TokenTree::Literal(Literal::string(&key)));
                        args.push(TokenTree::Punct(proc_macro::Punct::new(',', proc_macro::Spacing::Alone)));
                        if let Some(value) = value {
                            match value {
                                AttributeValue::Literal(literal) => {
                                    args.push(TokenTree::Literal(literal.clone()));
                                }
                                AttributeValue::Expression(stream) => {
                                    args.extend(stream.clone().into_iter());
                                }
                            }
                        } else {
                            args.push(TokenTree::Literal(Literal::string("None")));
                        }
                        let group = Group::new(Delimiter::Parenthesis, args.into_iter().collect());
                        tokens.push(TokenTree::Group(group));
                    }
                }
                tokens.extend(elements_to_with_token_stream(&tag.children));
            }
        }

        tokens.into_iter().collect()
    }

    //"42".parse().unwrap()
    //let result = element_to_token_stream(element);
    let result = elements_to_token_stream(&elements);

    result
}

fn is_ident(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    if id.chars().next().unwrap().is_numeric() {
        return false;
    }
    for c in id.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
enum Element {
    Expression(TokenStream),
    Literal(Literal),
    Tag(Tag),
}

#[derive(Debug, Clone)]
struct Parsed<T> {
    parsed: T,
    next: usize,
}

fn parse_mutliple_elements(tokens: &[TokenTree]) -> Option<Vec<Element>> {
    let mut elements = Vec::new();

    let mut idx = 0;
    loop {
        if idx >= tokens.len() {
            break;
        }

        let element = parse_element(&tokens[idx..])?;
        elements.push(element.parsed);
        idx += element.next;
    }

    Some(elements)
}

fn parse_element(tokens: &[TokenTree]) -> Option<Parsed<Element>> {

    let first = tokens.first()?;

    match first {
        TokenTree::Ident(_ident) => {
            let tag = parse_tag_element(tokens)?;
            Some(Parsed {
                parsed: Element::Tag(tag.tag),
                next: tag.next,
            })
        }
        TokenTree::Literal(literal) => {
            Some(Parsed {
                parsed: Element::Literal(literal.clone()),
                next: 1,
            })
        }
        TokenTree::Punct(_punct) => {
            None
        }
        TokenTree::Group(group) => {
            match group.delimiter() {
                Delimiter::Parenthesis => {
                    Some(Parsed {
                        parsed: Element::Expression(group.stream()),
                        next: 1,
                    })
                }
                Delimiter::Brace => {
                    None
                }
                Delimiter::Bracket => {
                    None
                }
                Delimiter::None => {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Tag {
    identifier: String,
    identifier_span: proc_macro::Span,
    attributes: Vec<(String, Option<AttributeValue>)>,
    children: Vec<Element>,
}

#[derive(Debug, Clone)]
struct ParsedTag {
    tag: Tag,
    next: usize,
}

fn parse_tag_element(tokens: &[TokenTree]) -> Option<ParsedTag> {

    let (identifier, next) = parse_html_identifier(tokens)?;

    let mut attributes = Vec::new();

    let mut idx = next;
    loop {
        if idx >= tokens.len() {
            break;
        }

        if let Some(inner) = parse_tag_inner(tokens, idx) {
            return Some(ParsedTag {
                tag: Tag {
                    identifier,
                    identifier_span: tokens[0].span(), // TODO this is not correct
                    attributes,
                    children: inner.parsed,
                },
                next: inner.next,
            })
        }

        if let Some((attribute, next2)) = parse_attribute(&tokens[idx..]) {
            attributes.push(attribute);
            idx += next2;
        } else {
            return None;
        }
    }

    None
}

fn parse_tag_inner(tokens: &[TokenTree], start: usize) -> Option<Parsed<Vec<Element>>> {
    if tokens.len() <= start {
        return None;
    }
    let token = &tokens[start];

    match token {
        TokenTree::Group(group) => {
            if group.delimiter() == Delimiter::Brace {
                let delimited = group.stream().into_iter().collect::<Vec<TokenTree>>();
                let children = parse_mutliple_elements(&delimited)?;
                return Some(Parsed {
                    parsed: children,
                    next: start + 1,
                });
            } else {
                None
            }
        }
        TokenTree::Punct(punct) => {
            if punct.as_char() == ';' {
                return Some(Parsed {
                    parsed: Vec::new(),
                    next: start + 1,
                });
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
enum AttributeValue {
    Literal(Literal),
    Expression(TokenStream),
}

fn parse_attribute(tokens: &[TokenTree]) -> Option<((String, Option<AttributeValue>), usize)> {
    // TODO handle # and .

    if tokens.is_empty() {
        return None;
    }

    // class
    if let TokenTree::Punct(punct) = &tokens[0] {
        if punct.as_char() == '.' {
            // TODO use @attribute_value_parsing made in a function
            let (identifier, next) = parse_html_identifier(&tokens[1..])?;
            return Some((("class".to_string(), Some(AttributeValue::Literal(Literal::string(&identifier)))), 1 + next));
        }
    }

    // id
    if let TokenTree::Punct(punct) = &tokens[0] {
        if punct.as_char() == '#' {
            // TODO use @attribute_value_parsing made in a function
            let (identifier, next) = parse_html_identifier(&tokens[1..])?;
            return Some((("id".to_string(), Some(AttributeValue::Literal(Literal::string(&identifier)))), 1 + next));
        }
    }

    let (identifier, next) = parse_html_identifier(tokens)?;

    if next < tokens.len() {
        let token = &tokens[next];
        if let TokenTree::Punct(punct) = token {
            if punct.as_char() == '=' {
                // @attribute_value_parsing
                if let Some((value_string, next2)) = parse_html_identifier(&tokens[next + 1..]) {
                    return Some(((identifier, Some(AttributeValue::Literal(Literal::string(&value_string)))), next + 1 + next2));
                } else {
                    let next_token_index = next + 1;
                    if next_token_index >= tokens.len() {
                        return Some(((identifier, None), next_token_index));
                    }
                    let next_token = &tokens[next_token_index];
                    match next_token {
                        TokenTree::Literal(literal) => {
                            return Some(((identifier, Some(AttributeValue::Literal(literal.clone()))), next_token_index + 1));
                        }
                        TokenTree::Group(group) => {
                            if group.delimiter() == Delimiter::Parenthesis {
                                return Some(((identifier, Some(AttributeValue::Expression(group.stream()))), next_token_index + 1));
                            }
                        }
                        _ => {
                            return Some(((identifier, None), next_token_index));
                        }
                    }
                }
            }

            return Some(((identifier, None), next));
        }
    }

    Some(((identifier, None), next))
}

fn parse_html_identifier(tokens: &[TokenTree]) -> Option<(String, usize)> {
    let mut identifier = String::new();

    let mut next = 0;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Type {
        Ident,
        Punct,
    }

    let mut prev_type = None;

    //let mut span: Option<proc_macro::Span> = None;
    //let push_span = |s: proc_macro::Span| {
    //    if let Some(span) = span {
    //        span = span.join(s).unwrap();
    //    } else {
    //        span = Some(s);
    //    }
    //};

    // iterate over the tokens until we find a non-ident or non "-" token
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                let typ = Type::Ident;
                if prev_type == Some(typ) {
                    break;
                }
                identifier.push_str(&ident.to_string());
                prev_type = Some(typ);
            }
            TokenTree::Literal(_literal) => {
                // TODO handle numbers
                break;
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() == '-' {
                    let typ = Type::Punct;
                    if prev_type == Some(typ) {
                        break;
                    }
                    identifier.push_str("-");
                    prev_type = Some(typ);
                } else {
                    break;
                }
            }
            TokenTree::Group(_group) => {
                break;
            }
        }

        next += 1;
    }

    if next == 0 || identifier.is_empty() {
        None
    } else {
        Some((identifier, next))
    }
}