use unic_langid::LanguageIdentifier;

static REGION_MATCHING_KEYS: &[&str] = &[
    "az", "bg", "cs", "de", "es", "fi", "fr", "hu", "it", "lt", "lv", "nl", "pl", "ro", "ru",
];

pub trait MockLikelySubtags {
    fn add_likely_subtags(&mut self) -> bool;
}

impl MockLikelySubtags for LanguageIdentifier {
    fn add_likely_subtags(&mut self) -> bool {
        let extended = match self.to_string().as_str() {
            "en" => "en-Latn-US",
            "fr" => "fr-Latn-FR",
            "sr" => "sr-Cyrl-SR",
            "sr-RU" => "sr-Latn-SR",
            "az-IR" => "az-Arab-IR",
            "zh-GB" => "zh-Hant-GB",
            "zh-US" => "zh-Hant-US",
            _ => {
                let lang = self.get_language();

                for subtag in REGION_MATCHING_KEYS {
                    if lang == *subtag {
                        self.set_region(Some(subtag)).unwrap();
                        return true;
                    }
                }
                return false;
            }
        };
        let langid: LanguageIdentifier = extended.parse().expect("Failed to parse langid.");
        self.set_language(Some(langid.get_language())).unwrap();
        self.set_script(langid.get_script()).unwrap();
        self.set_region(langid.get_region()).unwrap();
        return true;
    }
}
