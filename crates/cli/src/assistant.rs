use acvc_domain::{Error, Result};
use acvc_text::Lang;
use serde_json::{Value, json};

const NAMES: &[&str] = &[
    "indice",
    "objetivo",
    "recursos",
    "como-usar",
    "boas-praticas",
    "exemplos",
    "mau-uso",
    "trade-offs",
];

pub fn document(part: Option<&str>) -> Result<(Value, String)> {
    let lang = acvc_text::active();
    let names: Vec<&str> = match part {
        None => NAMES.to_vec(),
        Some(name) if NAMES.contains(&name) => vec![name],
        Some(name) => {
            return Err(Error::new("unknown assistant document").ctx("part", name));
        }
    };
    let tag = lang.tag();
    let documents: Vec<Value> = names
        .iter()
        .map(|name| {
            let text = text_of(lang, name);
            json!({
                "id": format!("acvc-assistant/{tag}/{name}"),
                "source": format!("assistant/{tag}/{name}.md"),
                "text": text,
                "metadata": {"lang": tag, "pack": "acvc-assistant"}
            })
        })
        .collect();
    let instruction = acvc_text::tr("assistant_index");
    let value = json!({
        "lang": tag,
        "index": {
            "mcp": "asb",
            "tool": "rag.index",
            "search": "rag.search",
            "instruction": instruction
        },
        "documents": documents
    });
    let mut text = format!("{instruction}\n");
    for name in &names {
        text.push('\n');
        text.push_str(text_of(lang, name));
        if !text.ends_with('\n') {
            text.push('\n');
        }
    }
    Ok((value, text))
}

fn text_of(lang: Lang, name: &str) -> &'static str {
    match (lang, name) {
        (Lang::PtBr, "indice") => include_str!("../../../assistant/pt-BR/indice.md"),
        (Lang::PtBr, "objetivo") => include_str!("../../../assistant/pt-BR/objetivo.md"),
        (Lang::PtBr, "recursos") => include_str!("../../../assistant/pt-BR/recursos.md"),
        (Lang::PtBr, "como-usar") => include_str!("../../../assistant/pt-BR/como-usar.md"),
        (Lang::PtBr, "boas-praticas") => include_str!("../../../assistant/pt-BR/boas-praticas.md"),
        (Lang::PtBr, "exemplos") => include_str!("../../../assistant/pt-BR/exemplos.md"),
        (Lang::PtBr, "mau-uso") => include_str!("../../../assistant/pt-BR/mau-uso.md"),
        (Lang::PtBr, "trade-offs") => include_str!("../../../assistant/pt-BR/trade-offs.md"),
        (Lang::EnUs, "indice") => include_str!("../../../assistant/en-US/indice.md"),
        (Lang::EnUs, "objetivo") => include_str!("../../../assistant/en-US/objetivo.md"),
        (Lang::EnUs, "recursos") => include_str!("../../../assistant/en-US/recursos.md"),
        (Lang::EnUs, "como-usar") => include_str!("../../../assistant/en-US/como-usar.md"),
        (Lang::EnUs, "boas-praticas") => include_str!("../../../assistant/en-US/boas-praticas.md"),
        (Lang::EnUs, "exemplos") => include_str!("../../../assistant/en-US/exemplos.md"),
        (Lang::EnUs, "mau-uso") => include_str!("../../../assistant/en-US/mau-uso.md"),
        (Lang::EnUs, "trade-offs") => include_str!("../../../assistant/en-US/trade-offs.md"),
        (Lang::Es, "indice") => include_str!("../../../assistant/es/indice.md"),
        (Lang::Es, "objetivo") => include_str!("../../../assistant/es/objetivo.md"),
        (Lang::Es, "recursos") => include_str!("../../../assistant/es/recursos.md"),
        (Lang::Es, "como-usar") => include_str!("../../../assistant/es/como-usar.md"),
        (Lang::Es, "boas-praticas") => include_str!("../../../assistant/es/boas-praticas.md"),
        (Lang::Es, "exemplos") => include_str!("../../../assistant/es/exemplos.md"),
        (Lang::Es, "mau-uso") => include_str!("../../../assistant/es/mau-uso.md"),
        (Lang::Es, "trade-offs") => include_str!("../../../assistant/es/trade-offs.md"),
        (Lang::Fr, "indice") => include_str!("../../../assistant/fr/indice.md"),
        (Lang::Fr, "objetivo") => include_str!("../../../assistant/fr/objetivo.md"),
        (Lang::Fr, "recursos") => include_str!("../../../assistant/fr/recursos.md"),
        (Lang::Fr, "como-usar") => include_str!("../../../assistant/fr/como-usar.md"),
        (Lang::Fr, "boas-praticas") => include_str!("../../../assistant/fr/boas-praticas.md"),
        (Lang::Fr, "exemplos") => include_str!("../../../assistant/fr/exemplos.md"),
        (Lang::Fr, "mau-uso") => include_str!("../../../assistant/fr/mau-uso.md"),
        (Lang::Fr, "trade-offs") => include_str!("../../../assistant/fr/trade-offs.md"),
        (Lang::Nl, "indice") => include_str!("../../../assistant/nl/indice.md"),
        (Lang::Nl, "objetivo") => include_str!("../../../assistant/nl/objetivo.md"),
        (Lang::Nl, "recursos") => include_str!("../../../assistant/nl/recursos.md"),
        (Lang::Nl, "como-usar") => include_str!("../../../assistant/nl/como-usar.md"),
        (Lang::Nl, "boas-praticas") => include_str!("../../../assistant/nl/boas-praticas.md"),
        (Lang::Nl, "exemplos") => include_str!("../../../assistant/nl/exemplos.md"),
        (Lang::Nl, "mau-uso") => include_str!("../../../assistant/nl/mau-uso.md"),
        (Lang::Nl, "trade-offs") => include_str!("../../../assistant/nl/trade-offs.md"),
        (Lang::El, "indice") => include_str!("../../../assistant/el/indice.md"),
        (Lang::El, "objetivo") => include_str!("../../../assistant/el/objetivo.md"),
        (Lang::El, "recursos") => include_str!("../../../assistant/el/recursos.md"),
        (Lang::El, "como-usar") => include_str!("../../../assistant/el/como-usar.md"),
        (Lang::El, "boas-praticas") => include_str!("../../../assistant/el/boas-praticas.md"),
        (Lang::El, "exemplos") => include_str!("../../../assistant/el/exemplos.md"),
        (Lang::El, "mau-uso") => include_str!("../../../assistant/el/mau-uso.md"),
        (Lang::El, "trade-offs") => include_str!("../../../assistant/el/trade-offs.md"),
        (Lang::Pl, "indice") => include_str!("../../../assistant/pl/indice.md"),
        (Lang::Pl, "objetivo") => include_str!("../../../assistant/pl/objetivo.md"),
        (Lang::Pl, "recursos") => include_str!("../../../assistant/pl/recursos.md"),
        (Lang::Pl, "como-usar") => include_str!("../../../assistant/pl/como-usar.md"),
        (Lang::Pl, "boas-praticas") => include_str!("../../../assistant/pl/boas-praticas.md"),
        (Lang::Pl, "exemplos") => include_str!("../../../assistant/pl/exemplos.md"),
        (Lang::Pl, "mau-uso") => include_str!("../../../assistant/pl/mau-uso.md"),
        (Lang::Pl, "trade-offs") => include_str!("../../../assistant/pl/trade-offs.md"),
        (Lang::Ru, "indice") => include_str!("../../../assistant/ru/indice.md"),
        (Lang::Ru, "objetivo") => include_str!("../../../assistant/ru/objetivo.md"),
        (Lang::Ru, "recursos") => include_str!("../../../assistant/ru/recursos.md"),
        (Lang::Ru, "como-usar") => include_str!("../../../assistant/ru/como-usar.md"),
        (Lang::Ru, "boas-praticas") => include_str!("../../../assistant/ru/boas-praticas.md"),
        (Lang::Ru, "exemplos") => include_str!("../../../assistant/ru/exemplos.md"),
        (Lang::Ru, "mau-uso") => include_str!("../../../assistant/ru/mau-uso.md"),
        (Lang::Ru, "trade-offs") => include_str!("../../../assistant/ru/trade-offs.md"),
        _ => "missing assistant document",
    }
}

#[cfg(test)]
mod tests {
    use super::document;

    #[test]
    fn portuguese_pack_is_ready_to_index() {
        acvc_text::install(acvc_text::Lang::PtBr);
        let (value, text) = document(None).unwrap();
        assert_eq!(value["lang"], "pt-BR");
        assert_eq!(value["index"]["tool"], "rag.index");
        assert_eq!(value["index"]["search"], "rag.search");
        let docs = value["documents"].as_array().unwrap();
        assert_eq!(docs.len(), 8);
        assert_eq!(docs[0]["id"], "acvc-assistant/pt-BR/indice");
        assert!(docs[0]["text"].as_str().unwrap().contains("rag.index"));
        assert!(text.contains("rag.search"));
        assert!(text.contains("Assistente ACVC"));
    }

    #[test]
    fn english_is_the_fallback_pack() {
        acvc_text::install(acvc_text::Lang::EnUs);
        let (value, text) = document(Some("objetivo")).unwrap();
        let docs = value["documents"].as_array().unwrap();
        assert_eq!(docs.len(), 1);
        assert!(text.contains("Git stores what a human chose to record."));
        assert!(document(Some("nope")).is_err());
    }
}
