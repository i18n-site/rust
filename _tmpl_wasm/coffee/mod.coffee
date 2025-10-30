#!/usr/bin/env coffee

> ./_.js > Ed25519Ph

export default (sk)=>
  console.log sk.length
  new Ed25519Ph(sk)
