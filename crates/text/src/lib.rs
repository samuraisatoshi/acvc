//! Messages for the computer locale. JSON status codes stay fixed.
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lang {
    PtBr = 0,
    EnUs = 1,
    Es = 2,
    Fr = 3,
    Nl = 4,
    El = 5,
    Pl = 6,
    Ru = 7,
}

impl Lang {
    pub fn tag(self) -> &'static str {
        match self {
            Self::PtBr => "pt-BR",
            Self::EnUs => "en-US",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::Nl => "nl",
            Self::El => "el",
            Self::Pl => "pl",
            Self::Ru => "ru",
        }
    }

    fn raw(self) -> &'static str {
        match self {
            Self::PtBr => include_str!("../../../locales/pt-BR.json"),
            Self::EnUs => include_str!("../../../locales/en-US.json"),
            Self::Es => include_str!("../../../locales/es.json"),
            Self::Fr => include_str!("../../../locales/fr.json"),
            Self::Nl => include_str!("../../../locales/nl.json"),
            Self::El => include_str!("../../../locales/el.json"),
            Self::Pl => include_str!("../../../locales/pl.json"),
            Self::Ru => include_str!("../../../locales/ru.json"),
        }
    }
}

thread_local! {
    static ACTIVE: Cell<Lang> = const { Cell::new(Lang::EnUs) };
}

pub fn install(lang: Lang) {
    ACTIVE.with(|slot| slot.set(lang));
}

pub fn active() -> Lang {
    ACTIVE.with(|slot| slot.get())
}

pub fn from_env() -> Lang {
    resolve(
        &std::env::var("LC_ALL").unwrap_or_default(),
        &std::env::var("LC_MESSAGES").unwrap_or_default(),
        &std::env::var("LANG").unwrap_or_default(),
    )
}

pub fn resolve(lc_all: &str, lc_messages: &str, lang: &str) -> Lang {
    for value in [lc_all, lc_messages, lang] {
        if identifiable(value) {
            return parse_tag(value);
        }
    }
    Lang::EnUs
}

pub fn parse_tag(raw: &str) -> Lang {
    let bare = raw.trim().split(['.', '@']).next().unwrap_or("").replace('_', "-");
    let lower = bare.to_ascii_lowercase();
    if !identifiable(&lower) {
        return Lang::EnUs;
    }
    match lower.split('-').next().unwrap_or("") {
        "pt" => Lang::PtBr,
        "en" => Lang::EnUs,
        "es" => Lang::Es,
        "fr" => Lang::Fr,
        "nl" => Lang::Nl,
        "el" => Lang::El,
        "pl" => Lang::Pl,
        "ru" => Lang::Ru,
        _ => Lang::EnUs,
    }
}

fn identifiable(value: &str) -> bool {
    let bare = value.trim().split(['.', '@']).next().unwrap_or("");
    let bare = bare.trim();
    !bare.is_empty() && !bare.eq_ignore_ascii_case("C") && !bare.eq_ignore_ascii_case("POSIX")
}

pub fn tr(key: &str) -> &'static str {
    phrase(active(), key)
}

pub fn fill(key: &str, pairs: &[(&str, &str)]) -> String {
    let mut text = tr(key).to_string();
    for (name, value) in pairs {
        text = text.replace(&format!("{{{name}}}"), value);
    }
    text
}

pub fn status_label(code: &str) -> String {
    match code {
        "on_frontier" => tr("status_frontier").to_string(),
        "superseded" => tr("status_beaten").to_string(),
        "rejected" => tr("status_rejected").to_string(),
        "unmeasured" => tr("no_measurement").to_string(),
        other => other.to_string(),
    }
}

pub fn catalog_json() -> String {
    Lang::raw(active()).to_string()
}

fn phrase(lang: Lang, key: &str) -> &'static str {
    catalog(lang)
        .get(key)
        .copied()
        .or_else(|| catalog(Lang::EnUs).get(key).copied())
        .unwrap_or("missing message")
}

fn catalog(lang: Lang) -> &'static HashMap<String, &'static str> {
    &catalogs()[lang as usize]
}

fn catalogs() -> &'static [HashMap<String, &'static str>; 8] {
    static CACHE: OnceLock<[HashMap<String, &'static str>; 8]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            load(Lang::PtBr),
            load(Lang::EnUs),
            load(Lang::Es),
            load(Lang::Fr),
            load(Lang::Nl),
            load(Lang::El),
            load(Lang::Pl),
            load(Lang::Ru),
        ]
    })
}

fn load(lang: Lang) -> HashMap<String, &'static str> {
    let value: serde_json::Value = serde_json::from_str(lang.raw()).expect("locale catalog");
    let object = value.as_object().expect("locale catalog object");
    object
        .iter()
        .filter_map(|(key, item)| {
            item.as_str()
                .map(|text| (key.clone(), Box::leak(text.to_string().into_boxed_str()) as &'static str))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_or_missing_locale_is_english() {
        assert_eq!(resolve("", "", ""), Lang::EnUs);
        assert_eq!(resolve("C", "POSIX", ""), Lang::EnUs);
        assert_eq!(resolve("de_DE.UTF-8", "", "pt_BR.UTF-8"), Lang::EnUs);
        assert_eq!(parse_tag("C"), Lang::EnUs);
    }

    #[test]
    fn known_locales_select_the_initial_languages() {
        assert_eq!(resolve("pt_BR.UTF-8", "", ""), Lang::PtBr);
        assert_eq!(resolve("C", "", "en_US.UTF-8"), Lang::EnUs);
        assert_eq!(parse_tag("es-MX"), Lang::Es);
        assert_eq!(parse_tag("fr_FR"), Lang::Fr);
        assert_eq!(parse_tag("nl_NL"), Lang::Nl);
        assert_eq!(parse_tag("el_GR"), Lang::El);
        assert_eq!(parse_tag("pl_PL"), Lang::Pl);
        assert_eq!(parse_tag("ru_RU"), Lang::Ru);
        assert_eq!(resolve("", "pt_PT.UTF-8", "en_US"), Lang::PtBr);
    }

    #[test]
    fn english_is_the_fallback_catalog() {
        install(Lang::EnUs);
        assert_eq!(tr("motive_unrecorded"), "Motive was not recorded.");
        install(Lang::PtBr);
        assert_eq!(tr("motive_unrecorded"), "Motivo não gravado.");
        install(Lang::Fr);
        assert_eq!(tr("on_frontier"), "Sur la frontière.");
        assert_eq!(status_label("on_frontier"), tr("status_frontier"));
    }
}
