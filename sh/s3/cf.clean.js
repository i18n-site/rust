#!/usr/bin/env node

import { CF_ID, CF_HOST } from "./conf/CF.js"
import purgeCache from "@3-/cf/purgeCache.js"
console.log(await purgeCache(CF_ID, CF_HOST, [`v/${process.argv[2]}`]))
process.exit()
