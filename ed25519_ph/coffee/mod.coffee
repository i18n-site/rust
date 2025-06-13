#!/usr/bin/env coffee

> ./_.js > Ed25519Ph

export default (sk)=>
  new Ed25519Ph(sk)
