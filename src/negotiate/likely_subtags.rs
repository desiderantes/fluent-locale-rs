use unic_locale::Locale;

static REGION_MATCHING_KEYS: &[&str] = &[
    "az", "bg", "cs", "de", "es", "fi", "fr", "hu", "it", "lt", "lv", "nl", "pl", "ro", "ru",
];

pub fn add(loc: &str) -> Option<String> {
    let extended = match loc {
        "en" => "en-Latn-US",
        "fr" => "fr-Latn-FR",
        "sr" => "sr-Cyrl-SR",
        "sr-RU" => "sr-Latn-RU",
        "az-IR" => "az-Arab-IR",
        "zh-GB" => "zh-Hant-GB",
        "zh-US" => "zh-Hant-US",
        _ => {
            let mut locale = Locale::from_str(loc).unwrap();
            let lang = locale.get_language().to_owned();

            for subtag in REGION_MATCHING_KEYS {
                if lang.as_str() == *subtag {
                    locale.set_region(Some(subtag)).unwrap();
                    let loc = locale.to_string();
                    return Some(loc);
                }
            }
            return None;
        }
    };

    Some(extended.to_owned())
}
