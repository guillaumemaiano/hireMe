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
            current_lang: Language::Fr,
            fluency: Fluency::Fluent,
        }
    }

    /// Return the list of available languages based on the audience.
    pub fn available_languages(&self) -> Vec<Language> {
        match self.audience {
            Audience::Business => vec![Language::Fr, Language::En],
            Audience::Academic => vec![Language::Fr, Language::En, Language::Ru, Language::Zh, Language::It],
            Audience::Other => vec![Language::Fr, Language::En],
        }
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
}
