#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! print_exit {
    ($($arg:tt)*) => {
      
        println!($($arg)*)
    };
}

