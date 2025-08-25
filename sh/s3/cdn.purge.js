#!/usr/bin/env node

import { CF_ID, CF_HOST, FASTLY_HOST, FASTLY_TOKEN } from "./conf/conf.js";
import purgeCf from "@3-/cf/purgeCache.js";
import { purge as purgeFastly } from "@3-/fastly";

const PROJECT = process.argv[2];
const URL = `v/${PROJECT}`;

console.log("purge", URL);

await Promise.all([
	purgeFastly(FASTLY_TOKEN, FASTLY_HOST, URL),
	purgeCf(CF_ID, CF_HOST, [`https://${CF_HOST}/${URL}`]),
]);

process.exit();
