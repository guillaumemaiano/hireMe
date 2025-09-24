use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use fluent_resmgr::ResourceManager;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

pub struct I18n {
    rm: ResourceManager,
    lang: LanguageIdentifier,
}

impl I18n {
    /// Create with a default language code, e.g. "en" or "fr"
    pub fn new(lang: &str) -> Self {
        let rm = ResourceManager::new("./assets/locales/{locale}/");
        let lang: LanguageIdentifier = lang.parse().unwrap();
        Self { rm, lang }
    }

    /// Change current language
    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.parse().unwrap();
    }

    /// Translate a key without arguments
    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, &FluentArgs::new())
    }

    /// Translate a key with arguments
    pub fn t_with_args(&self, key: &str, args: &FluentArgs) -> String {
        // Load bundle for this lang
        let res = self
            .rm
            .get_bundle(&self.lang, &["app.ftl".into()])
            .ok()
            .flatten();

        if let Some(bundle) = res {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(value) = msg.value() {
                    let mut errors = vec![];
                    let s = bundle.format_pattern(value, Some(args), &mut errors);
                    return s.to_string();
                }
            }
        }
        format!("{{{}}}", key) // fallback shows {key}
    }
}
