pub static LANG_NAME: [&str; 127] = [
  "English",           // English / 英语
  "简体中文",          // Chinese(Simplified) / 简体中文
  "Deutsch",           // German / 德语
  "Français",          // French / 法语
  "Español",           // Spanish / 西班牙语
  "Italiano",          // Italian / 意大利语
  "日本語",            // Japanese / 日语
  "Polski",            // Polish / 波兰语
  "Português(Brasil)", // Portuguese(Portugal, Brazil) / 葡萄牙语
  "Русский",           // Russian / 俄语
  "Nederlands",        // Dutch / 荷兰语
  "Türkçe",            // Turkish / 土耳其语
  "Svenska",           // Swedish / 瑞典语
  "Čeština",           // Czech / 捷克语
  "Українська",        // Ukrainian / 乌克兰语
  "Magyar",            // Hungarian / 匈牙利语
  "Indonesia",         // Indonesian / 印度尼西亚语
  "한국어",            // Korean / 韩语
  "Română",            // Romanian / 罗马尼亚语
  "Norsk",             // Norwegian / 挪威语
  "Slovenčina",        // Slovak / 斯洛伐克语
  "Suomi",             // Finnish / 芬兰语
  "العربية",           // Arabic / 阿拉伯语
  "Català",            // Catalan / 加泰罗尼亚语
  "Dansk",             // Danish / 丹麦语
  "فارسی",             // Persian / 波斯语
  "Tiếng Việt",        // Vietnamese / 越南语
  "Lietuvių",          // Lithuanian / 立陶宛语
  "Hrvatski",          // Croatian / 克罗地亚语
  "עברית",             // Hebrew / 希伯来语
  "Slovenščina",       // Slovenian / 斯洛文尼亚语
  "српски језик",      // Serbian / 塞尔维亚语
  "Esperanto",         // Esperanto / 世界语
  "Ελληνικά",          // Greek / 希腊文
  "Eesti",             // Estonian / 爱沙尼亚语
  "Български",         // Bulgarian / 保加利亚语
  "ไทย",               // Thai / 泰语
  "Haitian Creole",    // Haitian Creole / 海地克里奥尔语
  "Íslenska",          // Icelandic / 冰岛语
  "नेपाली",             // Nepali / 尼泊尔语
  "తెలుగు",             // Telugu / 泰卢固语
  "Latine",            // Latin / 拉丁文
  "Galego",            // Galician / 加利西亚语
  "हिन्दी",             // Hindi / 印地语
  "Cebuano",           // Cebuano / 宿务语
  "Melayu",            // Malay / 马来语
  "Euskara",           // Basque / 巴斯克语
  "Bosnian",           // Bosnian / 波斯尼亚语
  "Letzeburgesch",     // Luxembourgish / 卢森堡语
  "Latviešu",          // Latvian / 拉脱维亚语
  "ქართული",           // Georgian / 格鲁吉亚语
  "Shqip",             // Albanian / 阿尔巴尼亚语
  "मराठी",             // Marathi / 马拉地语
  "Azərbaycan",        // Azerbaijani / 阿塞拜疆语
  "Македонски",        // Macedonian / 马其顿语
  "Wikang Tagalog",    // Tagalog(Filipino) / 塔加路语(菲律宾语)
  "Cymraeg",           // Welsh / 威尔士语
  "বাংলা",               // Bengali / 孟加拉文
  "தமிழ்",              // Tamil / 泰米尔语
  "Basa Jawa",         // Javanese / 爪哇语
  "Basa Sunda",        // Sundanese / 巽他语
  "Беларуская",        // Belarusian / 白俄罗斯语
  "Kurdî(Navîn)",      // Kurdish / 库尔德语
  "Afrikaans",         // Afrikaans / 南非荷兰语
  "Frysk",             // Frisian / 弗里斯兰语
  "Toğikī",            // Tajik / 塔吉克语
  "اردو",              // Urdu / 乌尔都语
  "Kichwa",            // Quechua / 克丘亚语
  "മലയാളം",             // Malayalam / 马拉雅拉姆文
  "Kiswahili",         // Swahili / 斯瓦希里语
  "Gaeilge",           // Irish / 爱尔兰语
  "Uzbek(Latin)",      // Uzbek / 乌兹别克语
  "Te Reo Māori",      // Maori / 毛利语
  "Èdè Yorùbá",        // Yoruba / 约鲁巴语
  "ಕನ್ನಡ",              // Kannada / 卡纳达文
  "አማርኛ",              // Amharic / 阿姆哈拉语
  "Հայերեն",           // Armenian / 亚美尼亚文
  "অসমীয়া",             // Assamese / 阿萨姆语
  "Aymar Aru",         // Aymara / 艾马拉语
  "Bamanankan",        // Bambara / 班巴拉语
  "Bhojpuri",          // Bhojpuri / 博杰普尔语
  "正體中文",          // Chinese(Traditional) / 正體中文
  "Corsu",             // Corsican / 科西嘉语
  "ދިވެހިބަސް",             // Dhivehi / 迪维希语
  "Eʋegbe",            // Ewe / 埃维语
  "Filipino",          // Filipino(Tagalog) / 菲律宾语(塔加拉语)
  "Guarani",           // Guarani / 瓜拉尼语
  "ગુજરાતી",            // Gujarati / 古吉拉特文
  "Hausa",             // Hausa / 豪萨语
  "Hawaiian",          // Hawaiian / 夏威夷语
  "Hmong",             // Hmong / 苗语
  "Ásụ̀sụ́ Ìgbò",        // Igbo / 伊博语
  "Iloko",             // Ilocano / 伊洛卡诺语
  "Қазақ Тілі",        // Kazakh / 哈萨克语
  "ខ្មែរ",               // Khmer / 高棉语
  "Kinyarwanda",       // Kinyarwanda / 卢旺达语
  "سۆرانی",            // Kurdish(Sorani) / 库尔德语(索拉尼)
  "Кыргызча",          // Kyrgyz / 吉尔吉斯语
  "ລາວ",               // Lao / 老挝语
  "Lingála",           // Lingala / 林格拉语
  "Ganda",             // Luganda / 卢干达语
  "Maithili",          // Maithili / 迈蒂利语
  "Malagasy",          // Malagasy / 马尔加什语
  "Malti",             // Maltese / 马耳他语
  "монгол",            // Mongolian / 蒙古文
  "မြန်မာ",             // Myanmar(Burmese) / 缅甸语
  "ChiCheŵa",          // Nyanja(Chichewa) / 尼杨扎语(齐切瓦语)
  "ଓଡ଼ିଆ",               // Odia(Oriya) / 奥里亚语(奥里亚)
  "Afaan Oromoo",      // Oromo / 奥罗莫语
  "پښتو",              // Pashto / 普什图语
  "ਪੰਜਾਬੀ",             // Punjabi / 旁遮普语
  "Gagana Sāmoa",      // Samoan / 萨摩亚语
  "Sanskrit",          // Sanskrit / 梵语
  "Sesotho sa Leboa",  // Sepedi / 塞佩蒂语
  "Sesotho",           // Sesotho / 塞索托语
  "chiShona",          // Shona / 修纳语
  "سنڌي",              // Sindhi / 信德语
  "සිංහල",              // Sinhala(Sinhalese) / 僧伽罗语
  "Soomaali",          // Somali / 索马里语
  "Татар",             // Tatar / 鞑靼语
  "ትግር",               // Tigrinya / 蒂格尼亚语
  "Xitsonga",          // Tsonga / 宗加语
  "Türkmen Dili",      // Turkmen / 土库曼语
  "Akan",              // Twi(Akan) / 契维语(阿坎语)
  "isiXhosa",          // Xhosa / 班图语
  "ייִדיש",             // Yiddish / 意第绪语
  "Isi-Zulu",          // Zulu / 祖鲁语
];
