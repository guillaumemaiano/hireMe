use fluent_bundle::FluentArgs;
/// Represents the target audience for the app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Audience {
    Business,
    Academic,
    Other,
}

/// Represents a language code supported by the app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Fr, // French
    En, // English
    Ru, // Russian
    Zh, // Chinese
    It, // Italian
}

/// Represents fluency/usage level in a language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fluency {
    Fluent,
    Learning,
    Other,
}

#[derive(Clone, Debug)]
pub struct LangProfile {
    pub lang: Language,
    pub fluency: Fluency,
}

/// The core model holding the app state.
#[derive(Debug, Clone)]
pub struct HireMeModel {
    pub audience: Audience,
    pub current_lang: Language,
    pub fluency: Fluency,
}

impl HireMeModel {
    /// Create a new model with default values.
    pub fn new() -> Self {
        Self {
            audience: Audience::Business,
            current_lang: Language::En,
            fluency: Fluency::Fluent,
        }
    }
    pub fn all_audiences() -> Vec<&'static str> {
        vec!["Business", "Academic", "Other"]
    }

    pub fn current_audience_name(&self) -> &'static str {
        match self.audience {
            Audience::Business => "Business",
            Audience::Academic => "Academic",
            Audience::Other => "Other",
        }
    }

    pub fn set_audience_by_name(&mut self, name: &str) {
        self.audience = match name {
            "Business" => Audience::Business,
            "Academic" => Audience::Academic,
            _ => Audience::Other,
        };
    }

    pub fn set_lang_default(&mut self) {
        self.current_lang = Language::En;
    }

    /// Utility: return a short string code for a language (for FTL args).
    pub fn lang_code(lang: &Language) -> &'static str {
        match lang {
            Language::Fr => "fr",
            Language::En => "en",
            Language::Ru => "ru",
            Language::Zh => "zh",
            Language::It => "it",
        }
    }

    /// Utility: return a user-facing display name for a language (for dropdowns).
    pub fn lang_name(lang: &Language) -> &'static str {
        match lang {
            Language::Fr => "Français",
            Language::En => "English",
            Language::Ru => "Русский",
            Language::Zh => "中文",
            Language::It => "Italiano",
        }
    }

    pub fn set_audience(&mut self, audience: Audience) {
        self.audience = audience;
    }

    pub fn set_language(&mut self, lang: Language) {
        self.current_lang = lang;
    }

    pub fn set_fluency(&mut self, fluency: Fluency) {
        self.fluency = fluency;
    }

    // some helpers because I'd rather have readable UI code
    pub fn current_lang_code(&self) -> &'static str {
        Self::lang_code(&self.current_lang)
    }

    pub fn current_lang_name(&self) -> &'static str {
        Self::lang_name(&self.current_lang)
    }

    pub fn to_args(&self) -> FluentArgs {
        let mut args = FluentArgs::new();

        // audience: "business"/"academic"/"other"
        let audience_code = match self.audience {
            Audience::Business => "business",
            Audience::Academic => "academic",
            Audience::Other => "other",
        };
        args.set("audience", audience_code);

        // language is short code
        args.set("lang", Self::lang_code(&self.current_lang));
        args.set("langName", Self::lang_name(&self.current_lang));
        // fluency : "fluent"/"learning"/"other"
        let fluency_code = match self.fluency {
            Fluency::Fluent => "fluent",
            Fluency::Learning => "learning",
            Fluency::Other => "other",
        };
        args.set("level", fluency_code);

        args
    }

    pub fn all_languages(&self) -> Vec<LangProfile> {
        vec![
            LangProfile {
                lang: Language::Fr,
                fluency: Fluency::Fluent,
            },
            LangProfile {
                lang: Language::En,
                fluency: Fluency::Fluent,
            },
            LangProfile {
                lang: Language::Ru,
                fluency: Fluency::Learning,
            },
            LangProfile {
                lang: Language::Zh,
                fluency: Fluency::Learning,
            },
            LangProfile {
                lang: Language::It,
                fluency: Fluency::Learning,
            },
        ]
    }

    /// Return the list of available languages based on the audience.
    pub fn available_languages(&self) -> Vec<Language> {
        match self.audience {
            Audience::Business => vec![Language::Fr, Language::En],
            Audience::Academic => vec![
                Language::Fr,
                Language::En,
                Language::Ru,
                Language::Zh,
                Language::It,
            ],
            Audience::Other => vec![Language::Fr, Language::En],
        }
    }

    pub fn languages_with_fluency(&self) -> Vec<LangProfile> {
        let avail = self.available_languages();
        self.all_languages()
            .into_iter()
            .filter(|lp| avail.contains(&lp.lang))
            .collect()
    }
}
