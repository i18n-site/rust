use mysql_async::Value;

pub struct MysqlVal(pub Vec<Value>);

impl<T, const N: usize> From<[T; N]> for MysqlVal
where
  T: Into<Value>,
{
  fn from(values: [T; N]) -> Self {
    MysqlVal(values.into_iter().map(Into::into).collect())
  }
}

impl From<Vec<Value>> for MysqlVal {
  fn from(values: Vec<Value>) -> Self {
    MysqlVal(values)
  }
}

macro_rules! tuple {
  ($($t:ident),*) => {
    impl<$($t: Into<Value>),*> From<($($t,)*)> for MysqlVal {
      #[allow(unused_variables)]
      fn from(li: ($($t,)*)) -> Self {
        MysqlVal(vec![
          $(${ignore($t)} li.${index()}.into()),*
        ])
      }
    }
  };
}

tuple!();
tuple!(V0);
tuple!(V0, V1);
tuple!(V0, V1, V2);
tuple!(V0, V1, V2, V3);
tuple!(V0, V1, V2, V3, V4);
tuple!(V0, V1, V2, V3, V4, V5);
tuple!(V0, V1, V2, V3, V4, V5, V6);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15);
tuple!(V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16);
