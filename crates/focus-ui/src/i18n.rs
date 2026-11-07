//! Traductions Fluent. Ajouter une langue : un fichier `.ftl` et une ligne dans `LANGS`.

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use leptos::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use unic_langid::LanguageIdentifier;

/// Langue disponible.
pub struct Lang {
    /// Code BCP 47.
    pub code: &'static str,
    /// Nom de la langue dans cette langue.
    pub name: &'static str,
    /// Écriture de droite à gauche.
    pub rtl: bool,
    /// Relue par une personne compétente.
    pub reviewed: bool,
    source: &'static str,
}

macro_rules! lang {
    ($code:literal, $name:literal, $rtl:literal, $reviewed:literal) => {
        Lang {
            code: $code,
            name: $name,
            rtl: $rtl,
            reviewed: $reviewed,
            source: include_str!(concat!("../../../locales/", $code, ".ftl")),
        }
    };
}

/// Langues livrées. Voir `docs/langues.md`.
pub const LANGS: &[Lang] = &[
    lang!("en", "English", false, true),
    lang!("fr", "Français", false, false),
    lang!("es", "Español", false, false),
    lang!("de", "Deutsch", false, false),
    lang!("it", "Italiano", false, false),
    lang!("pt-BR", "Português (Brasil)", false, false),
    lang!("ru", "Русский", false, false),
    lang!("ja", "日本語", false, false),
    lang!("ko", "한국어", false, false),
    lang!("zh-CN", "简体中文", false, false),
    lang!("ar", "العربية", true, false),
];

/// Choisit la meilleure langue pour un code du système.
pub fn best_match(code: &str) -> &'static str {
    let lower = code.to_lowercase();
    LANGS
        .iter()
        .find(|lang| lang.code.eq_ignore_ascii_case(code))
        .or_else(|| {
            LANGS
                .iter()
                .find(|lang| lower.starts_with(&lang.code[..2].to_lowercase()))
        })
        .map_or("en", |lang| lang.code)
}

type Bundle = FluentBundle<FluentResource>;

fn bundle(lang: &Lang) -> Option<Bundle> {
    let id: LanguageIdentifier = lang.code.parse().ok()?;
    let mut bundle = FluentBundle::new(vec![id]);
    // Sans ces marques, les paramètres se mélangent mal dans une phrase RTL... mais
    // elles s'affichent dans certains cas : la direction est déjà gérée par la page.
    bundle.set_use_isolating(false);
    bundle
        .add_resource(FluentResource::try_new(lang.source.to_owned()).ok()?)
        .ok()?;
    Some(bundle)
}

/// Accès aux traductions, réactif à la langue courante.
#[derive(Clone, Copy)]
pub struct I18n {
    /// Langue courante.
    pub lang: RwSignal<&'static str>,
    bundles: StoredValue<Rc<HashMap<&'static str, Bundle>>, LocalStorage>,
}

impl I18n {
    /// Charge toutes les langues.
    pub fn new(initial: &'static str) -> Self {
        let bundles: HashMap<_, _> = LANGS
            .iter()
            .filter_map(|lang| Some((lang.code, bundle(lang)?)))
            .collect();
        Self {
            lang: RwSignal::new(initial),
            bundles: StoredValue::new_local(Rc::new(bundles)),
        }
    }

    fn format(&self, key: &str, args: Option<&FluentArgs<'_>>) -> String {
        let code = self.lang.get();
        self.bundles.with_value(|bundles| {
            // Clé manquante : anglais, jamais une clé brute (v4.3, section 4).
            [code, "en"]
                .iter()
                .find_map(|lang| {
                    let bundle = bundles.get(lang)?;
                    let pattern = bundle.get_message(key)?.value()?;
                    let mut errors = Vec::new();
                    Some(
                        bundle
                            .format_pattern(pattern, args, &mut errors)
                            .into_owned(),
                    )
                })
                .unwrap_or_default()
        })
    }

    /// Texte sans paramètre.
    pub fn t(&self, key: &str) -> String {
        self.format(key, None)
    }

    /// Texte avec un paramètre texte.
    pub fn ts(&self, key: &str, name: &'static str, value: &str) -> String {
        let mut args = FluentArgs::new();
        args.set(name, value.to_owned());
        self.format(key, Some(&args))
    }

    /// Texte avec un paramètre nombre, pour les pluriels.
    pub fn tn(&self, key: &str, name: &'static str, value: u64) -> String {
        let mut args = FluentArgs::new();
        args.set(name, FluentValue::from(value as f64));
        self.format(key, Some(&args))
    }

    /// Texte à deux paramètres.
    pub fn ts2(
        &self,
        key: &str,
        first: (&'static str, &str),
        second: (&'static str, &str),
    ) -> String {
        let mut args = FluentArgs::new();
        args.set(first.0, first.1.to_owned());
        args.set(second.0, second.1.to_owned());
        self.format(key, Some(&args))
    }

    /// La langue courante.
    pub fn current(&self) -> Option<&'static Lang> {
        let code = self.lang.get();
        LANGS.iter().find(|lang| lang.code == code)
    }
}

/// Raccourci pour récupérer les traductions.
pub fn use_i18n() -> I18n {
    use_context::<I18n>().unwrap_or_else(|| I18n::new("en"))
}
