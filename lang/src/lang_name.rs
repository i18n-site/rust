pub static LANG_NAME: [&str; 127] = [
  "English",                        // English / 英语
  "简体中文",                       // Chinese / 简体中文
  "español",                        // Spanish / 西班牙语
  "français",                       // French / 法语
  "Deutsch",                        // German / 德语
  "日本語",                         // Japanese / 日语
  "italiano",                       // Italian / 意大利语
  "한국어",                         // Korean / 韩语
  "русский",                        // Russian / 俄语
  "português",                      // Portuguese / 葡萄牙语
  "shqip",                          // Albanian / 阿尔巴尼亚语
  "\u{202b}العربية\u{202c}",        // Arabic / 阿拉伯语
  "አማርኛ",                           // Amharic / 阿姆哈拉语
  "অসমীয়া",                         // Assamese / 阿萨姆语
  "azərbaycan",                     // Azerbaijani / 阿塞拜疆语
  "Eʋegbe",                         // Ewe / 埃维语
  "Aymar aru",                      // Aymara / 艾马拉语
  "Gaeilge",                        // Irish / 爱尔兰语
  "eesti",                          // Estonian / 爱沙尼亚语
  "ଓଡ଼ିଆ",                            // Odia / 奥利亚语
  "Oromoo",                         // Oromo / 奥罗莫语
  "euskara",                        // Basque / 巴斯克语
  "беларуская",                     // Belarusian / 白俄罗斯语
  "bamanakan",                      // Bambara / 班巴拉语
  "български",                      // Bulgarian / 保加利亚语
  "íslenska",                       // Icelandic / 冰岛语
  "polski",                         // Polish / 波兰语
  "bosanski",                       // Bosnian / 波斯尼亚语
  "\u{202b}فارسی\u{202c}",          // Persian / 波斯语
  "भोजपुरी",                         // Bhojpuri / 博杰普尔语
  "Afrikaans",                      // Afrikaans / 布尔语(南非荷兰语)
  "татар",                          // Tatar / 鞑靼语
  "dansk",                          // Danish / 丹麦语
  "\u{202b}ދިވެހިބަސް\u{202c}",          // Divehi / 迪维希语
  "ትግርኛ",                           // Tigrinya / 蒂格尼亚语
  "संस्कृत भाषा",                      // Sanskrit / 梵语
  "Filipino",                       // Filipino / 菲律宾语
  "suomi",                          // Finnish / 芬兰语
  "Frysk",                          // Western Frisian / 弗里西语
  "ខ្មែរ",                           // Khmer / 高棉语
  "ქართული",                        // Georgian / 格鲁吉亚语
  "ગુજરાતી",                         // Gujarati / 古吉拉特语
  "avañe’ẽ",                        // Guarani / 瓜拉尼语
  "қазақ тілі",                     // Kazakh / 哈萨克语
  "Kreyòl ayisyen",                 // Haitian Creole / 海地克里奥尔语
  "Hausa",                          // Hausa / 豪萨语
  "Nederlands",                     // Dutch / 荷兰语
  "кыргызча",                       // Kyrgyz / 吉尔吉斯语
  "galego",                         // Galician / 加利西亚语
  "català",                         // Catalan / 加泰罗尼亚语
  "čeština",                        // Czech / 捷克语
  "ಕನ್ನಡ",                           // Kannada / 卡纳达语
  "corsu",                          // Corsican / 科西嘉语
  "hrvatski",                       // Croatian / 克罗地亚语
  "Runasimi",                       // Quechua / 克丘亚语
  "kurdî",                          // Kurdish / 库尔德语(库尔曼吉语)
  "\u{202b}کوردیی ناوەندی\u{202c}", // Central Kurdish / 库尔德语(索拉尼)
  "Latina",                         // Latin / 拉丁语
  "latviešu",                       // Latvian / 拉脱维亚语
  "ລາວ",                            // Lao / 老挝语
  "lietuvių",                       // Lithuanian / 立陶宛语
  "lingála",                        // Lingala / 林格拉语
  "Luganda",                        // Ganda / 卢干达语
  "Lëtzebuergesch",                 // Luxembourgish / 卢森堡语
  "Kinyarwanda",                    // Kinyarwanda / 卢旺达语
  "română",                         // Romanian / 罗马尼亚语
  "Malagasy",                       // Malagasy / 马尔加什语
  "Malti",                          // Maltese / 马耳他语
  "मराठी",                          // Marathi / 马拉地语
  "മലയാളം",                         // Malayalam / 马拉雅拉姆语
  "Melayu",                         // Malay / 马来语
  "македонски",                     // Macedonian / 马其顿语
  "मैथिली",                          // Maithili / 迈蒂利语
  "Māori",                          // Māori / 毛利语
  "монгол",                         // Mongolian / 蒙古语
  "বাংলা",                          // Bangla / 孟加拉语
  "မြန်မာ",                          // Burmese / 缅甸语
  "𞄀𞄄𞄰𞄩𞄍𞄜𞄰",                          // Hmong Njua / 苗语
  "IsiXhosa",                       // Xhosa / 南非科萨语
  "isiZulu",                        // Zulu / 南非祖鲁语
  "नेपाली",                          // Nepali / 尼泊尔语
  "norsk",                          // Norwegian / 挪威语
  "ਪੰਜਾਬੀ",                          // Punjabi / 旁遮普语
  "\u{202b}پښتو\u{202c}",           // Pashto / 普什图语
  "Nyanja",                         // Nyanja / 齐切瓦语
  "Akan",                           // Akan / 契维语
  "svenska",                        // Swedish / 瑞典语
  "Gagana fa'a Sāmoa",              // Samoan / 萨摩亚语
  "српски",                         // Serbian / 塞尔维亚语
  "Sesotho sa Leboa",               // Northern Sotho / 塞佩蒂语
  "Sesotho",                        // Southern Sotho / 塞索托语
  "සිංහල",                           // Sinhala / 僧伽罗语
  "esperanto",                      // Esperanto / 世界语
  "slovenčina",                     // Slovak / 斯洛伐克语
  "slovenščina",                    // Slovenian / 斯洛文尼亚语
  "Kiswahili",                      // Swahili / 斯瓦希里语
  "Cebuano",                        // Cebuano / 宿务语
  "Soomaali",                       // Somali / 索马里语
  "тоҷикӣ",                         // Tajik / 塔吉克语
  "తెలుగు",                          // Telugu / 泰卢固语
  "தமிழ்",                           // Tamil / 泰米尔语
  "ไทย",                            // Thai / 泰语
  "Türkçe",                         // Turkish / 土耳其语
  "türkmen dili",                   // Turkmen / 土库曼语
  "Cymraeg",                        // Welsh / 威尔士语
  "\u{202b}اردو\u{202c}",           // Urdu / 乌尔都语
  "українська",                     // Ukrainian / 乌克兰语
  "o‘zbek",                         // Uzbek / 乌兹别克语
  "\u{202b}עברית\u{202c}",          // Hebrew / 希伯来语
  "Ελληνικά",                       // Greek / 希腊语
  "ʻŌlelo Hawaiʻi",                 // Hawaiian / 夏威夷语
  "\u{202b}سنڌي\u{202c}",           // Sindhi / 信德语
  "magyar",                         // Hungarian / 匈牙利语
  "chiShona",                       // Shona / 修纳语
  "հայերեն",                        // Armenian / 亚美尼亚语
  "Igbo",                           // Igbo / 伊博语
  "Pagsasao Ilokano",               // Ilokano / 伊洛卡诺语
  "\u{202b}ייִדיש\u{202c}",          // Yiddish / 意第绪语
  "हिन्दी",                          // Hindi / 印地语
  "Basa Sunda",                     // Sundanese / 印尼巽他语
  "Indonesia",                      // Indonesian / 印尼语
  "Jawa",                           // Javanese / 爪哇语
  "Èdè Yorùbá",                     // Yoruba / 约鲁巴语
  "Tiếng Việt",                     // Vietnamese / 越南语
  "正體中文",                       // Traditional Chinese / 正體中文
  "Xitsonga",                       // Tsonga / 宗加语
  "\u{202b}ئۇيغۇرچە\u{202c}",       // Uyghur / 维吾尔语
];
