[‼️]: ✏️README.mdt

# bls

```rust
use aok::{Result, OK};
use bls::PublicKey;
use rand::Rng;
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let mut rng = rand::thread_rng();

  for _ in 0..10000 {
    let mut array = [0u8; 48];
    for i in &mut array {
      *i = rng.gen();
    }
    let pk = PublicKey(array);
    if let Ok::<bls12_381::G1Projective, _>(pk) = (&pk).try_into() {
      dbg!(pk);
    }
  }
  // let sk = SecretKey::default();
  // let pk: PublicKey = sk.pk();
  //
  // println!("SK: {}", sk);
  // println!("PK: {}", pk);
  //
  // let data_to_sign = b"Hello, world!"; // Example data to sign
  // let signature = sign(&sk, data_to_sign);
  // dbg!(signature.len());
  // let is_valid = verify(&pk, data_to_sign, &signature);
  // println!("Signature is valid: {}", is_valid);
  OK
}
```
