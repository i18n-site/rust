#!/usr/bin/env -S node --trace-uncaught --expose-gc --unhandled-rejections=strict
var begin, cost, gen, ico_li, img;

import {
  Gen
} from './dist/_.js';

import ICO_LI from './ICO_LI.js';

gen = new Gen(ICO_LI);

begin = new Date();

[img, ...ico_li] = gen.gen(500, 600);

cost = new Date() - begin;

console.log('cost', cost / 1000);

await Deno.writeFile('x.avif', img);
