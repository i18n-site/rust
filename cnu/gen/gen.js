#!/usr/bin/env bun

import write from '@3-/write'

import {simplified_chinese,traditional_chinese,toSimplified,words} from './transverter.js'
console.log(simplified_chinese.length,traditional_chinese.length)

write(
  '../src/f.rs',
  `pub static F: &str = "${traditional_chinese}";`
)

write(
  '../src/j.rs',
  `pub static J: &str = "${simplified_chinese}";`
)

let jli = [], fli = [];

Object.entries(toSimplified).forEach(([f, j]) => {
  fli.push(f)
  jli.push(j)
})

write(
  '../src/f2j.rs',
  `pub static F: &str = "${fli.join('')}";
pub static J: &str = "${jli.join('')}";
`
)

jli = [];
fli = [];

Object.entries(words).forEach(([j, f]) => {
  jli.push(j)
  fli.push(f)
  console.log(
    f,
    j,
    [...f].forEach((i)=>{
      console.log('>',i,simplified_chinese[traditional_chinese.indexOf(i)]);
    }),
    [...j].forEach((i)=>{
      console.log('>',i,traditional_chinese[simplified_chinese.indexOf(i)]);
    })
  )

})

write(
  '../src/word.rs',
  `pub static F: &[&str] = &${JSON.stringify(fli)};
pub static J: &[&str] = &${JSON.stringify(jli)};
`
)
