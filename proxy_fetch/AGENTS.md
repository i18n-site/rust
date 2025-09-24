安装包依赖都用 cargo add 和 cargo add -F,不可直接编辑 Cargo.toml
Cargo.toml 的 edition 用最新的版本, rust 要用最新写法，禁止用过时的模块
use 要写明具体的导入模块
注释都写英文和中文双语
避免使用 unwrap
格式化字符串尽量把变量名写到字符串中,比如 format!("{varname}")
锁尽量用 parking_lot
如果需要一个并发读写的字典, 用 DashMap
src/ 错误都用 thiserror 在 src/error.rs 中定义
参数要秉承最小化原则,能传部分属性,就不要传入整个结构体
函数名,变量名都要简洁
遇到问题,多用 dbg!进行调试