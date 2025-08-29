#!/usr/bin/env python

from os.path import abspath, dirname
from tzutil.dirreplace import dirreplace

FROM_STRING = """
tracing = "0.1.41"
tracing
loginit
"""

TO_STRING = """
log = "0.4.27"
log
log_init
"""

dirreplace(
    dirname(abspath(__file__)),
    FROM_STRING,
    TO_STRING,
)
