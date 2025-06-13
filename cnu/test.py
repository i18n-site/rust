#!/usr/bin/env python


def is_cn(i):
    i = ord(i)

    if i == 0x3007:
        return True
    for f, t in (
        (0x4E00, 0x9FA5),
        (0x9FA6, 0x9FCB),
        (0x3400, 0x4DB5),
        (0x20000, 0x2A6D6),
        (0x2A700, 0x2B734),
        (0x2B740, 0x2B81D),
        (0x2F00, 0x2FD5),
        (0x2E80, 0x2EF3),
        (0xF900, 0xFAD9),
        (0x2F800, 0x2FA1D),
        (0xE815, 0xE86F),
        (0xE400, 0xE5E8),
        (0xE600, 0xE6CF),
        (0x31C0, 0x31E3),
        (0x2FF0, 0x2FFB),
        (0x3105, 0x3120),
        (0x31A0, 0x31BA),
    ):
        if i >= f and i <= t:
            return True


tmpl = """
use phf::{phf_map, Map};

pub static %s: Map<char, char> = phf_map! {
  %s
};
"""

f2j = []
j2f = []

exist_f = set()
exist_j = set()
mulit_f = set()
mulit_j = set()
with open("cn.txt") as cn:
    cn = list(cn)
    cn.sort()
    fj = []
    for i in cn:
        i = i.strip().split("(")
        if len(i) == 2:
            fc, jc = i
            jc = jc[:1]
            if len(fc) == 1 and len(jc) == 1:
                if fc in exist_f:
                    mulit_f.add(fc)
                if jc in exist_j:
                    mulit_j.add(jc)

                    # if fc in exist or jc in exist:
                    #     print(fc, jc)
                    #     mulit.add(fc)
                    #     mulit.add(jc)
                exist_f.add(fc)
                exist_j.add(jc)
                if fc != jc:
                    fj.append((fc, jc))
                # if is_cn(fc) and is_cn(jc):
                #     continue
    for fc, jc in fj:
        if jc != "锺":
            f2j.append(f"'{fc}'=>'{jc}',")
        if jc not in mulit_j:
            j2f.append(f"'{jc}'=>'{fc}',")


print(mulit_f)  # 只有 钟

print(mulit_j)

with open("src/f2j.rs", "w") as f:
    f.write(tmpl % ("F2J", "\n  ".join(f2j)))
with open("src/j2f.rs", "w") as f:
    f.write(tmpl % ("J2F", "\n  ".join(j2f)))

# def cjk_detect(texts):
#     count = defaultdict(int)
#     for i in texts:
#         i = ord(i)
#         if i >= 0xAC00 and i <= 0xD7A3:
#             count["ko"] += 1
#         elif i >= 0x3040 and i <= 0x30FF:
#             count["ja"] += 1
#         elif i >= 0x4E00 and i <= 0x9FFF:
#             count["zh"] += 1
#
#     print(count)
#     # # korean
#     # if re.search("[\uac00-\ud7a3]", texts):
#     #     return "ko"
#     # # japanese
#     # if re.search("[\u3040-\u30ff]", texts):
#     #     return "ja"
#     # # chinese
#     # for i in [
#     #     "[\u4e00-\u9FFF]",
#     # ]:
#     #     if re.search(i, texts):
#     #         return "zh"
#     return None
#
#
# def test_cjk_detect():
#     # Pure English
#     assert cjk_detect("Is Obstruction an Impeachable Offense? History Says Yes") is None
#     # Pure French
#     assert (
#         cjk_detect(
#             "Damian Lillard a réussi un nouveau shoot de la victoire"
#             " au buzzer à très longue distance"
#         )
#         is None
#     )
#     # Simplified Chinese
#     assert (
#         cjk_detect(
#             "2009年，波音公司(Boeing)在查尔斯顿附近的新厂破土动工时，曾宣扬这里是最先进的制造中心"
#             "，将制造一款世界上最先进的飞机。但在接下来的十年里，这家生产787梦想客机的工厂一直受到做"
#             "工粗糙和监管不力的困扰，危及航空安全。"
#         )
#         == "zh"
#     )
#     # Traditional Chinese
#     assert (
#         cjk_detect("北查爾斯頓工廠的安全漏洞已經引起了航空公司和監管機構的密切關注。")
#         == "zh"
#     )
#     # Japanese
#     assert (
#         cjk_detect("日産自動車は24日、2019年3月期の連結業績予想を下方修正した。")
#         == "ja"
#     )
#     # Korean
#     assert cjk_detect("투서로 뜨고 투서에 지나") == "ko"
#     # Korean with a Chinese character
#     assert (
#         cjk_detect("北 외무성 간부 총살설 주민들 사이서 확산…하노이 회담 실패 때문")
#         == "ko"
#     )
#
#
# def print_incorrect_cases():
#     texts = "투서로 뜨고 투서에 지나"
#     print(texts, "expected: ja actual:", cjk_detect(texts))
#     # Japanese
#     texts = "日産自動車、営業益45%減　前期下方修正"
#     print(texts, "expected: ja actual:", cjk_detect(texts))
#     # Traditional Chinese with Japanese hiragana
#     texts = "健康の油切 好吃の涼麵"
#     print(texts, "expected: zh actual:", cjk_detect(texts))
#     # Traditional Chinese with Japanese katakana punctuation
#     texts = "鐵腕・都鐸王朝（五）：文藝復興最懂穿搭的高富帥——亨利八世"
#     print(texts, "expected: zh actual:", cjk_detect(texts))
#
#
# if __name__ == "__main__":
#     # Correct cases
#     # test_cjk_detect()
#     # Incorrect cases
#     print_incorrect_cases()
