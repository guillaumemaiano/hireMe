use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use std::collections::HashSet;
use std::fs;
use unic_langid::LanguageIdentifier;

pub struct I18n {
    lang: LanguageIdentifier,
    bundle: Option<FluentBundle<FluentResource>>, // Own resources
    failed_langs: HashSet<LanguageIdentifier>,    // Negative cache
}

impl I18n {
    /// Create with a default language code, e.g. "en" or "fr"
    pub fn new(lang: &str) -> Self {
        let lang: LanguageIdentifier = lang.parse().unwrap();
        let mut i18n = Self {
            lang,
            bundle: None,
            failed_langs: HashSet::new(),
        };
        i18n.reload_bundle();
        i18n
    }

    fn reload_bundle(&mut self) {
        if self.failed_langs.contains(&self.lang) {
            eprintln!("Skipping reload, {} already failed before", self.lang);
            self.bundle = None;
            return;
        }

        let filename = format!("./assets/locales/{}/app.ftl", self.lang);
        match fs::read_to_string(&filename) {
            Ok(source) => match FluentResource::try_new(source) {
                Ok(resource) => {
                    let mut bundle = FluentBundle::new(vec![self.lang.clone()]);
                    if let Err(errs) = bundle.add_resource(resource) {
                        eprintln!("Failed to add resource: {:?}", errs);
                        self.bundle = None;
                    } else {
                        self.bundle = Some(bundle);
                    }
                }
                Err(_) => {
                    eprintln!("Failed to parse resource for {}", self.lang);
                    self.failed_langs.insert(self.lang.clone());
                    self.bundle = None;
                }
            },
            Err(err) => {
                eprintln!("FTL file error: {:?}", err);
                self.failed_langs.insert(self.lang.clone());
                self.bundle = None;
            }
        }
    }

    /// Change current language
    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.parse().unwrap();
        self.reload_bundle();
    }

    /// Translate a key without arguments
    pub fn t(&self, key: &str) -> String {
        self.t_with_args(key, &FluentArgs::new())
    }

    /// Translate a key with arguments
    pub fn t_with_args(&self, key: &str, args: &FluentArgs) -> String {
        if let Some(bundle) = &self.bundle {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(value) = msg.value() {
                    let mut errors = vec![];
                    return bundle.format_pattern(value, Some(args), &mut errors).to_string();
                }
            }
        }
        format!("{{{}}}", key) // fallback shows {key}
    }
}
