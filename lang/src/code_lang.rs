use phf::{phf_map, Map};

use crate::Lang;

pub static CODE_LANG: Map<&'static str, Lang> = phf_map! {
"en" => Lang::En, // English / English / 英语
"zh" => Lang::Zh, // 简体中文 / Chinese / 简体中文
"zh-cn" => Lang::Zh, // 简体中文 /  /
"es" => Lang::Es, // español / Spanish / 西班牙语
"fr" => Lang::Fr, // français / French / 法语
"de" => Lang::De, // Deutsch / German / 德语
"ja" => Lang::Ja, // 日本語 / Japanese / 日语
"it" => Lang::It, // italiano / Italian / 意大利语
"ko" => Lang::Ko, // 한국어 / Korean / 韩语
"ru" => Lang::Ru, // русский / Russian / 俄语
"pt" => Lang::Pt, // português / Portuguese / 葡萄牙语
"sq" => Lang::Sq, // shqip / Albanian / 阿尔巴尼亚语
"ar" => Lang::Ar, // العربية / Arabic / 阿拉伯语
"am" => Lang::Am, // አማርኛ / Amharic / 阿姆哈拉语
"as" => Lang::As, // অসমীয়া / Assamese / 阿萨姆语
"az" => Lang::Az, // azərbaycan / Azerbaijani / 阿塞拜疆语
"ee" => Lang::Ee, // Eʋegbe / Ewe / 埃维语
"ay" => Lang::Ay, // Aymar aru / Aymara / 艾马拉语
"ga" => Lang::Ga, // Gaeilge / Irish / 爱尔兰语
"et" => Lang::Et, // eesti / Estonian / 爱沙尼亚语
"or" => Lang::Or, // ଓଡ଼ିଆ / Odia / 奥利亚语
"om" => Lang::Om, // Oromoo / Oromo / 奥罗莫语
"eu" => Lang::Eu, // euskara / Basque / 巴斯克语
"be" => Lang::Be, // беларуская / Belarusian / 白俄罗斯语
"bm" => Lang::Bm, // bamanakan / Bambara / 班巴拉语
"bg" => Lang::Bg, // български / Bulgarian / 保加利亚语
"is" => Lang::Is, // íslenska / Icelandic / 冰岛语
"pl" => Lang::Pl, // polski / Polish / 波兰语
"bs" => Lang::Bs, // bosanski / Bosnian / 波斯尼亚语
"fa" => Lang::Fa, // فارسی / Persian / 波斯语
"bho" => Lang::Bho, // भोजपुरी / Bhojpuri / 博杰普尔语
"af" => Lang::Af, // Afrikaans / Afrikaans / 布尔语(南非荷兰语)
"tt" => Lang::Tt, // татар / Tatar / 鞑靼语
"da" => Lang::Da, // dansk / Danish / 丹麦语
"dv" => Lang::Dv, // ދިވެހިބަސް / Divehi / 迪维希语
"ti" => Lang::Ti, // ትግርኛ / Tigrinya / 蒂格尼亚语
"sa" => Lang::Sa, // संस्कृत भाषा / Sanskrit / 梵语
"fil" => Lang::Fil, // Filipino / Filipino / 菲律宾语
"fi" => Lang::Fi, // suomi / Finnish / 芬兰语
"fy" => Lang::Fy, // Frysk / Western Frisian / 弗里西语
"km" => Lang::Km, // ខ្មែរ / Khmer / 高棉语
"ka" => Lang::Ka, // ქართული / Georgian / 格鲁吉亚语
"gu" => Lang::Gu, // ગુજરાતી / Gujarati / 古吉拉特语
"gn" => Lang::Gn, // avañe’ẽ / Guarani / 瓜拉尼语
"kk" => Lang::Kk, // қазақ тілі / Kazakh / 哈萨克语
"ht" => Lang::Ht, // Kreyòl ayisyen / Haitian Creole / 海地克里奥尔语
"ha" => Lang::Ha, // Hausa / Hausa / 豪萨语
"nl" => Lang::Nl, // Nederlands / Dutch / 荷兰语
"ky" => Lang::Ky, // кыргызча / Kyrgyz / 吉尔吉斯语
"gl" => Lang::Gl, // galego / Galician / 加利西亚语
"ca" => Lang::Ca, // català / Catalan / 加泰罗尼亚语
"cs" => Lang::Cs, // čeština / Czech / 捷克语
"kn" => Lang::Kn, // ಕನ್ನಡ / Kannada / 卡纳达语
"co" => Lang::Co, // corsu / Corsican / 科西嘉语
"hr" => Lang::Hr, // hrvatski / Croatian / 克罗地亚语
"qu" => Lang::Qu, // Runasimi / Quechua / 克丘亚语
"ku" => Lang::Ku, // kurdî / Kurdish / 库尔德语(库尔曼吉语)
"ckb" => Lang::Ckb, // کوردیی ناوەندی / Central Kurdish / 库尔德语(索拉尼)
"la" => Lang::La, // Latina / Latin / 拉丁语
"lv" => Lang::Lv, // latviešu / Latvian / 拉脱维亚语
"lo" => Lang::Lo, // ລາວ / Lao / 老挝语
"lt" => Lang::Lt, // lietuvių / Lithuanian / 立陶宛语
"ln" => Lang::Ln, // lingála / Lingala / 林格拉语
"lg" => Lang::Lg, // Luganda / Ganda / 卢干达语
"lb" => Lang::Lb, // Lëtzebuergesch / Luxembourgish / 卢森堡语
"rw" => Lang::Rw, // Kinyarwanda / Kinyarwanda / 卢旺达语
"ro" => Lang::Ro, // română / Romanian / 罗马尼亚语
"mg" => Lang::Mg, // Malagasy / Malagasy / 马尔加什语
"mt" => Lang::Mt, // Malti / Maltese / 马耳他语
"mr" => Lang::Mr, // मराठी / Marathi / 马拉地语
"ml" => Lang::Ml, // മലയാളം / Malayalam / 马拉雅拉姆语
"ms" => Lang::Ms, // Melayu / Malay / 马来语
"mk" => Lang::Mk, // македонски / Macedonian / 马其顿语
"mai" => Lang::Mai, // मैथिली / Maithili / 迈蒂利语
"mi" => Lang::Mi, // Māori / Māori / 毛利语
"mn" => Lang::Mn, // монгол / Mongolian / 蒙古语
"bn" => Lang::Bn, // বাংলা / Bangla / 孟加拉语
"my" => Lang::My, // မြန်မာ / Burmese / 缅甸语
"hmn" => Lang::Hmn, // 𞄀𞄄𞄰𞄩𞄍𞄜𞄰 / Hmong Njua / 苗语
"xh" => Lang::Xh, // IsiXhosa / Xhosa / 南非科萨语
"zu" => Lang::Zu, // isiZulu / Zulu / 南非祖鲁语
"ne" => Lang::Ne, // नेपाली / Nepali / 尼泊尔语
"no" => Lang::No, // norsk / Norwegian / 挪威语
"pa" => Lang::Pa, // ਪੰਜਾਬੀ / Punjabi / 旁遮普语
"ps" => Lang::Ps, // پښتو / Pashto / 普什图语
"ny" => Lang::Ny, // Nyanja / Nyanja / 齐切瓦语
"ak" => Lang::Ak, // Akan / Akan / 契维语
"sv" => Lang::Sv, // svenska / Swedish / 瑞典语
"sm" => Lang::Sm, // Gagana fa'a Sāmoa / Samoan / 萨摩亚语
"sr" => Lang::Sr, // српски / Serbian / 塞尔维亚语
"nso" => Lang::Nso, // Sesotho sa Leboa / Northern Sotho / 塞佩蒂语
"st" => Lang::St, // Sesotho / Southern Sotho / 塞索托语
"si" => Lang::Si, // සිංහල / Sinhala / 僧伽罗语
"eo" => Lang::Eo, // esperanto / Esperanto / 世界语
"sk" => Lang::Sk, // slovenčina / Slovak / 斯洛伐克语
"sl" => Lang::Sl, // slovenščina / Slovenian / 斯洛文尼亚语
"sw" => Lang::Sw, // Kiswahili / Swahili / 斯瓦希里语
"ceb" => Lang::Ceb, // Cebuano / Cebuano / 宿务语
"so" => Lang::So, // Soomaali / Somali / 索马里语
"tg" => Lang::Tg, // тоҷикӣ / Tajik / 塔吉克语
"te" => Lang::Te, // తెలుగు / Telugu / 泰卢固语
"ta" => Lang::Ta, // தமிழ் / Tamil / 泰米尔语
"th" => Lang::Th, // ไทย / Thai / 泰语
"tr" => Lang::Tr, // Türkçe / Turkish / 土耳其语
"tk" => Lang::Tk, // türkmen dili / Turkmen / 土库曼语
"cy" => Lang::Cy, // Cymraeg / Welsh / 威尔士语
"ur" => Lang::Ur, // اردو / Urdu / 乌尔都语
"uk" => Lang::Uk, // українська / Ukrainian / 乌克兰语
"uz" => Lang::Uz, // o‘zbek / Uzbek / 乌兹别克语
"he" => Lang::He, // עברית / Hebrew / 希伯来语
"iw" => Lang::He, // עברית /  /
"el" => Lang::El, // Ελληνικά / Greek / 希腊语
"haw" => Lang::Haw, // ʻŌlelo Hawaiʻi / Hawaiian / 夏威夷语
"sd" => Lang::Sd, // سنڌي / Sindhi / 信德语
"hu" => Lang::Hu, // magyar / Hungarian / 匈牙利语
"sn" => Lang::Sn, // chiShona / Shona / 修纳语
"hy" => Lang::Hy, // հայերեն / Armenian / 亚美尼亚语
"ig" => Lang::Ig, // Igbo / Igbo / 伊博语
"ilo" => Lang::Ilo, // Pagsasao Ilokano / Ilokano / 伊洛卡诺语
"yi" => Lang::Yi, // ייִדיש / Yiddish / 意第绪语
"hi" => Lang::Hi, // हिन्दी / Hindi / 印地语
"su" => Lang::Su, // Basa Sunda / Sundanese / 印尼巽他语
"id" => Lang::Id, // Indonesia / Indonesian / 印尼语
"jv" => Lang::Jv, // Jawa / Javanese / 爪哇语
"jw" => Lang::Jv, // Jawa /  /
"yo" => Lang::Yo, // Èdè Yorùbá / Yoruba / 约鲁巴语
"vi" => Lang::Vi, // Tiếng Việt / Vietnamese / 越南语
"zh-tw" => Lang::ZhTw, // 正體中文 / Traditional Chinese / 正體中文
"ts" => Lang::Ts, // Xitsonga / Tsonga / 宗加语
"ug" => Lang::Ug, // ئۇيغۇرچە / Uyghur / 维吾尔语
};
