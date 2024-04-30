pub static LANG_NAME: [&str; 121] = [
  "English",                 // English / 英语
  "简体中文",                // Chinese / 简体中文
  "Español",                 // Spanish / 西班牙语
  "Français",                // French / 法语
  "Deutsch",                 // German / 德语
  "日本語",                  // Japanese / 日语
  "Italiano",                // Italian / 意大利语
  "한국어",                  // Korean / 韩语
  "Русский",                 // Russian / 俄语
  "Português",               // Portuguese / 葡萄牙语
  "Shqip",                   // Albanian / 阿尔巴尼亚语
  "\u{202b}العربية\u{202c}", // Arabic / 阿拉伯语
  "አማርኛ",                    // Amharic / 阿姆哈拉语
  "অসমীয়া",                  // Assamese / 阿萨姆语
  "Azərbaycan",              // Azerbaijani / 阿塞拜疆语
  "Eʋegbe",                  // Ewe / 埃维语
  "Aymar Aru",               // Aymara / 艾马拉语
  "Gaeilge",                 // Irish / 爱尔兰语
  "Eesti",                   // Estonian / 爱沙尼亚语
  "ଓଡ଼ିଆ",                     // Odia / 奥利亚语
  "Oromoo",                  // Oromo / 奥罗莫语
  "Euskara",                 // Basque / 巴斯克语
  "Беларуская",              // Belarusian / 白俄罗斯语
  "Bamanakan",               // Bambara / 班巴拉语
  "Български",               // Bulgarian / 保加利亚语
  "Íslenska",                // Icelandic / 冰岛语
  "Polski",                  // Polish / 波兰语
  "Bosanski",                // Bosnian / 波斯尼亚语
  "\u{202b}فارسی\u{202c}",   // Persian / 波斯语
  "भोजपुरी",                  // Bhojpuri / 博杰普尔语
  "Afrikaans",               // Afrikaans / 布尔语(南非荷兰语)
  "Татар",                   // Tatar / 鞑靼语
  "Dansk",                   // Danish / 丹麦语
  "\u{202b}ދިވެހިބަސް\u{202c}",   // Divehi / 迪维希语
  "ትግርኛ",                    // Tigrinya / 蒂格尼亚语
  "संस्कृत भाषा",               // Sanskrit / 梵语
  "Filipino",                // Filipino / 菲律宾语
  "Suomi",                   // Finnish / 芬兰语
  "Frysk",                   // Western Frisian / 弗里西语
  "ខ្មែរ",                    // Khmer / 高棉语
  "Ქართული",                 // Georgian / 格鲁吉亚语
  "ગુજરાતી",                  // Gujarati / 古吉拉特语
  "Avañe’ẽ",                 // Guarani / 瓜拉尼语
  "Қазақ Тілі",              // Kazakh / 哈萨克语
  "Kreyòl Ayisyen",          // Haitian Creole / 海地克里奥尔语
  "Hausa",                   // Hausa / 豪萨语
  "Nederlands",              // Dutch / 荷兰语
  "Кыргызча",                // Kyrgyz / 吉尔吉斯语
  "Galego",                  // Galician / 加利西亚语
  "Català",                  // Catalan / 加泰罗尼亚语
  "Čeština",                 // Czech / 捷克语
  "ಕನ್ನಡ",                    // Kannada / 卡纳达语
  "Corsu",                   // Corsican / 科西嘉语
  "Hrvatski",                // Croatian / 克罗地亚语
  "Runasimi",                // Quechua / 克丘亚语
  "Kurdî",                   // Kurdish / 库尔德语(库尔曼吉语)
  "Latina",                  // Latin / 拉丁语
  "Latviešu",                // Latvian / 拉脱维亚语
  "ລາວ",                     // Lao / 老挝语
  "Lietuvių",                // Lithuanian / 立陶宛语
  "Lingála",                 // Lingala / 林格拉语
  "Luganda",                 // Ganda / 卢干达语
  "Lëtzebuergesch",          // Luxembourgish / 卢森堡语
  "Kinyarwanda",             // Kinyarwanda / 卢旺达语
  "Română",                  // Romanian / 罗马尼亚语
  "Malagasy",                // Malagasy / 马尔加什语
  "Malti",                   // Maltese / 马耳他语
  "मराठी",                   // Marathi / 马拉地语
  "മലയാളം",                  // Malayalam / 马拉雅拉姆语
  "Melayu",                  // Malay / 马来语
  "Македонски",              // Macedonian / 马其顿语
  "मैथिली",                   // Maithili / 迈蒂利语
  "Māori",                   // Māori / 毛利语
  "Монгол",                  // Mongolian / 蒙古语
  "বাংলা",                   // Bangla / 孟加拉语
  "မြန်မာ",                   // Burmese / 缅甸语
  "𞄀𞄄𞄰𞄩𞄍𞄜𞄰",                   // Hmong Njua / 苗语
  "Isixhosa",                // Xhosa / 南非科萨语
  "Isizulu",                 // Zulu / 南非祖鲁语
  "नेपाली",                   // Nepali / 尼泊尔语
  "Norsk",                   // Norwegian / 挪威语
  "ਪੰਜਾਬੀ",                   // Punjabi / 旁遮普语
  "\u{202b}پښتو\u{202c}",    // Pashto / 普什图语
  "Nyanja",                  // Nyanja / 齐切瓦语
  "Akan",                    // Akan / 契维语
  "Svenska",                 // Swedish / 瑞典语
  "Gagana Fa'a Sāmoa",       // Samoan / 萨摩亚语
  "සිංහල",                    // Sinhala / 僧伽罗语
  "Esperanto",               // Esperanto / 世界语
  "Slovenčina",              // Slovak / 斯洛伐克语
  "Slovenščina",             // Slovenian / 斯洛文尼亚语
  "Kiswahili",               // Swahili / 斯瓦希里语
  "Cebuano",                 // Cebuano / 宿务语
  "Soomaali",                // Somali / 索马里语
  "Тоҷикӣ",                  // Tajik / 塔吉克语
  "తెలుగు",                   // Telugu / 泰卢固语
  "தமிழ்",                    // Tamil / 泰米尔语
  "ไทย",                     // Thai / 泰语
  "Türkçe",                  // Turkish / 土耳其语
  "Türkmen Dili",            // Turkmen / 土库曼语
  "Cymraeg",                 // Welsh / 威尔士语
  "\u{202b}اردو\u{202c}",    // Urdu / 乌尔都语
  "O‘zbek",                  // Uzbek / 乌兹别克语
  "\u{202b}עברית\u{202c}",   // Hebrew / 希伯来语
  "Ελληνικά",                // Greek / 希腊语
  "ʻōlelo Hawaiʻi",          // Hawaiian / 夏威夷语
  "\u{202b}سنڌي\u{202c}",    // Sindhi / 信德语
  "Magyar",                  // Hungarian / 匈牙利语
  "Chishona",                // Shona / 修纳语
  "Հայերեն",                 // Armenian / 亚美尼亚语
  "Igbo",                    // Igbo / 伊博语
  "Pagsasao Ilokano",        // Ilokano / 伊洛卡诺语
  "\u{202b}ייִדיש\u{202c}",   // Yiddish / 意第绪语
  "हिन्दी",                   // Hindi / 印地语
  "Basa Sunda",              // Sundanese / 印尼巽他语
  "Indonesia",               // Indonesian / 印尼语
  "Jawa",                    // Javanese / 爪哇语
  "Èdè Yorùbá",              // Yoruba / 约鲁巴语
  "Tiếng Việt",              // Vietnamese / 越南语
  "正體中文",                // Traditional Chinese / 正體中文
  "Xitsonga",                // Tsonga / 宗加语
];
