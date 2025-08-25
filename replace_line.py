#!/usr/bin/env python3

from os.path import abspath, dirname, basename, join, exists
from tzutil.dirreplace import dirreplace

FROM_STRING = """
0.23.30
"""

TO_STRING = """
0.23.29
"""

dirreplace(
    dirname(abspath(__file__)),
    FROM_STRING,
    TO_STRING,
)
