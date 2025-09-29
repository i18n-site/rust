#!/usr/bin/env python

from os.path import abspath, dirname
from tzutil.dirreplace import dirreplace

FROM_STRING = """
#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]
"""

TO_STRING = """
#![cfg_attr(docsrs, feature(doc_cfg))]
"""

dirreplace(
    dirname(abspath(__file__)),
    FROM_STRING,
    TO_STRING,
)
