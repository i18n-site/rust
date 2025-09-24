#!/usr/bin/env python

from os.path import abspath, dirname
from tzutil.dirreplace import dirreplace

FROM_STRING = """
txt_li.push(
"""

TO_STRING = """
txt_li.push_no_tran(
"""

dirreplace(
    dirname(abspath(__file__)),
    FROM_STRING,
    TO_STRING,
)
