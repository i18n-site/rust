#!/usr/bin/env node

import { CF_ID, CF_HOST } from "./conf/CF.js"
import purgeCache from "@3-/cf/purgeCache.js"

const url = `v/${process.argv[2]}`
console.log(url)
console.log(await purgeCache(CF_ID, CF_HOST, [url]))
process.exit()
