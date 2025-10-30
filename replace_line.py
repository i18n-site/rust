#!/usr/bin/env python

from os.path import abspath, dirname
from tzutil.dirreplace import dirreplace

FROM_STRING = """
#![feature(doc_cfg)]
"""

TO_STRING = """

"""

dirreplace(
    dirname(abspath(__file__)),
    FROM_STRING,
    TO_STRING,
)
