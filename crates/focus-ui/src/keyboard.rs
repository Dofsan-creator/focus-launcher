//! Clavier à l'écran : une disposition par écriture, selon la langue.

/// Touches par rangée pour une langue.
pub fn layout(lang: &str) -> &'static [&'static str] {
    match lang {
        "fr" => &["1234567890", "azertyuiop", "qsdfghjklm", "wxcvbné'-"],
        "de" => &["1234567890", "qwertzuiopü", "asdfghjklöä", "yxcvbnmß-"],
        "es" => &["1234567890", "qwertyuiop", "asdfghjklñ", "zxcvbnm'-"],
        "ru" => &["1234567890", "йцукенгшщзхъ", "фывапролджэ", "ячсмитьбюё-"],
        "ar" => &["١٢٣٤٥٦٧٨٩٠", "ضصثقفغعهخحج", "شسيبلاتنمكط", "ئءؤرىةوزظد"],
        "ko" => &[
            "1234567890",
            "ㅂㅈㄷㄱㅅㅛㅕㅑㅐㅔ",
            "ㅁㄴㅇㄹㅎㅗㅓㅏㅣ",
            "ㅋㅌㅊㅍㅠㅜㅡ",
        ],
        "ja" => &[
            "1234567890",
            "あいうえおかきくけこ",
            "さしすせそたちつてと",
            "なにぬねのはひふへほ",
            "まみむめもやゆよらりるれろわをん",
        ],
        _ => &["1234567890", "qwertyuiop", "asdfghjkl", "zxcvbnm'-"],
    }
}
