#!/usr/bin/env coffee

> @3-/protopkg
  @3-/uridir
  path > join dirname

ROOT = uridir(import.meta)
await protopkg(
  ROOT
  ROOT
)
