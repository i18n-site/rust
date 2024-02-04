use cnu::is_cn_char;

#[test]
fn test() {
  let s = "B端工具型产品在企业环境中发挥着至关重要的作用，然而，它们的复杂性常常让用户感到困扰。引导式设计成为解决这一挑战的重要策略。通过本文，我们将深入探讨B端产品引导式设计的特殊性，以及如何通过它来提高效率、降低成本，满足用户需求，创造更大价值。";
  let f = cnu::j2f(s);
  let j = cnu::f2j(s);
  dbg!(f);
  dbg!(j);

  dbg!(is_cn_char('a'));
  dbg!(is_cn_char('我'));
}
