use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Er" => "Err",
        "Bone" => "Ok",
        "Ĉeno" | "Cxeno" => "String",
        "Tradukaĵo" | "Tradukajxo" => "HashMap",
        "Defaŭlto" | "Defauxlto" => "Default",
        "Eraro" => "Error",
        "Malnepra" => "Option",
        "Io" => "Some",
        "Nenio" => "None",
        "Rezulto" => "Result",
        "Memo" => "Self",
        "printovico" => "println",
        "eksplodi" => "break",
        "malsink" => "async",
        "atendi" => "await",
        "iteracii" => "loop",
        "movi" => "move",
        "kesto" => "crate",
        "netingebla_programteksto" => "unreachable_code",
        "kiel" => "as",
        "konstanta" => "const",
        "trajto" => "trait",
        "malsekura" => "unsafe",
        "en" => "in",
        "el" => "from",
        "dinamike" => "dyn",
        "senŝeligi" | "sensxeligi" => "unwrap",
        "defaŭlto" | "defauxlto" => "default",
        "kiel_ref" => "as_ref",
        "enel" => "io", // short for enigo-eligo
        "ekster" => "extern",
        "falsa" => "false",
        "funkcio" => "fn",
        "supra" => "super",
        "enmeti" => "insert",
        "akiri" => "get",
        "permesi" => "allow",
        "ekpaniki" | "fek" => "panic",
        "modulo" => "mod",
        "ŝanĝebla" | "sxangxebla" => "mut",
        "ŝanĝebla" | "sxangxebla" => "mutable",
        "freŝa" | "fresxa" => "new",
        "kie" => "where",
        "por" => "for",
        "akiri_aŭ_enigo_kun" | "akiri_aux_enigo_kun" => "get_or_insert_with",
        "ĉefa" => "main",
        "publika" => "pub",
        "redoni" => "return",
        "realigo" => "impl",
        "kompari" => "match",
        "ĉu" | "cxu" => "if",
        "plu" => "else",
        "memo" => "self",
        "ebligi" | "permesi" => "let",
        "senmova" | "statika" => "static",
        "strukturi" | "strukt" => "struct",
        "antaŭvidi" | "antauxvidi" | "ekspekti" => "expect",
        "dum" => "while",
        "uzi" => "use",
        "igi" => "into",
        "vera" => "true",
        "denombrado" | "deno" => "enum",
        "kolektoj" => "collections",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rustteksto(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
