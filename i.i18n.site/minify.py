#!/usr/bin/env python

li = []
with open("setup.sh", "r") as file:
    for i in file.readlines():
        i = i.strip()
        if i:
            li.append(i)

with open("minify.sh", "w") as out:
    out.write("\n".join(li))
