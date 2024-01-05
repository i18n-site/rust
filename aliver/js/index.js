import {default as $decode} from '@3-/proto/decode.js'
import {u64 as $u64,string as $string,u32 as $u32} from '@3-/proto/decode/types.js'

export const IdName /*
  0 id  	u64
  1 name	string
*/ = $decode(
  [$u64,$string],
  [0,""]
)

export const State /*
  0 kindId 	u64
  1 hostId 	u64
  2 dnsType	u32
  3 err    	u32
  4 ts     	u64
*/ = $decode(
  [$u64,$u64,$u32,$u32,$u64],
  [0,0,0,0,0]
)

export const Check /*
  0 last 	u64
  1 count	u64
  2 cost 	u64
*/ = $decode(
  [$u64,$u64,$u64],
  [0,0,0]
)

export const StateLi /*
  0 kind 	[IdName]
  1 host 	[IdName]
  2 li   	[State]
  3 check	Check
*/ = $decode(
  [IdName,IdName,State,Check],
  [1,1,1,0]
)