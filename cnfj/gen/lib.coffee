#!/usr/bin/env coffee

> @3-/aiapi/gemini.js
  @3-/read
  os > homedir
  path > join

ROOT = import.meta.dirname

TOKEN_LI = (await import(join(
  homedir()
  '.config/gemini.js'
))).default

export chat = gemini TOKEN_LI

export TXT = new Set read(join ROOT,"xiandaihaiyuchangyongcibiao.txt").trim().split('\n').filter(
  (i)=>i.length>1
)
