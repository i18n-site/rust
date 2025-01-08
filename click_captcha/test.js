#!/usr/bin/env -S node --trace-uncaught --expose-gc --unhandled-rejections=strict
var begin, cost, flag_li, gen, img;

import {
  Gen
} from './dist/_.js';

import FLAG_LI from './FLAG_LI.js';

gen = new Gen(FLAG_LI);

begin = new Date();

[img, ...flag_li] = gen.gen(500, 600);

cost = new Date() - begin;

console.log('cost', cost / 1000);

await Deno.writeFile('x.avif', img);
