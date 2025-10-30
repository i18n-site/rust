#[macro_export]
macro_rules! conn {
  ($($name:ident, $name_str:literal);+)=>{
    pub mod conn {
      $(
        #[cfg(feature = $name_str)]
        pub mod $name {
          use $crate::$name::StreamType;

          pub struct ConnFuture {
            pub(crate) fut: std::pin::Pin<Box<dyn std::future::Future<Output = Result<StreamType, std::io::Error>> + Send>>,
          }

          impl std::future::Future for ConnFuture {
            type Output = Result<StreamType, std::io::Error>;
            fn poll(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                self.fut.as_mut().poll(cx)
            }
          }
        }
      )+
    }
  }
}
