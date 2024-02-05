import {default as $decode} from '@3-/proto/decode.js'
import {u64 as $u64,string as $string,u32 as $u32} from '@3-/proto/decode/types.js'
import BIN1 from "@3-/proto/decode/BIN1.js"

export const IdName /*
  0 id  	u64
  1 name	string
*/ = $decode(
  [$u64,$string],
  [0,""]
)

export const State /*
  0 dnsType	u32
  1 err    	u32
  2 ts     	u64
*/ = $decode(
  [$u32,$u32,$u64],
  [0,0,0]
)

export const HostStateLi /*
  0 hostId	u64
  1 li    	[State]
*/ = $decode(
  [$u64,State],
  [0,1]
)

export const KindStateLi /*
  0 kindId	u64
  1 li    	[HostStateLi]
*/ = $decode(
  [$u64,HostStateLi],
  [0,1]
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
  2 ip   	[IdName]
  3 ok   	[KindStateLi]
  4 err  	[KindStateLi]
  5 check	Check
*/ = $decode(
  [IdName,IdName,IdName,KindStateLi,KindStateLi,Check],
  [1,1,1,1,1,Check(BIN1)]
)