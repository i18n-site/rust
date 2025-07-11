use std::io::Write;

use aok::{OK, Result};
use static_init::constructor;
use svg2webp::svg2webp;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let img = svg2webp(
    r##"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 90 111">
  <g stroke-linejoin="bevel" stroke-linecap="square" fill="none" stroke="#000">
    <path d="M6,15l30-10l49,11v82v-82l-24,8v83v-83l-55-9v83l56,8l23-8" stroke-width="4"/>
    <path d="M6,98l27-9v-13l-26-4l27-8v-20l-27-5l28-8v-11v11l49,10l-24,8l-26-5v20l50,8l-24,9l-27-5v13l51,8" stroke-width="2"/>
  </g>
</svg>"##,
  )?;
  ifs::w("svg.webp")?.write_all(&img)?;
  OK
}
