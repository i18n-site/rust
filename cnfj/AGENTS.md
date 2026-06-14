合理划分模块，暴露接口要低耦合，高内聚
依赖用 cargo add 和 cargo add -F 安装,不可直接编辑 Cargo.toml
性能最大化,能在编译期确定的，就要避免运行时开销
用到的模块、函数都尽量在文件开头导入
对于模块内部用的结构体，直接曝光字段，而不是写包装函数
use 要写明具体的导入模块, 禁止用 `use *`
用 as 等数字转换要小心静默失败，但是要是 100%确定没问题，就不用 try_into，大胆用 as 提高性能
禁止使用 panic
用 as 向下转换数字要小心静默失败
要避免过渡设计
约束要最小化（避免冗余的约束）
参数要秉承最小化原则,能传部分属性,就不要传入整个结构体
避免不必要的复制 clone 开销，时间复杂度、空间复杂度都要追求完美
不写重复的代码，不写雷同的类，可通过函数抽象+泛型参数、泛型 trait 等编程技巧减少冗余代码
函数名,变量名都要简洁，不要起冗长的名
避免雷同的字符串，请定义为常量
注释都写中文,注释要简洁，不必注释显而易见的东西,不要写语言名做前缀
格式化字符串尽量把变量名写到字符串中,比如 format!("{varname}")
遇到问题,多用 dbg!进行调试
追求性能的极致，包括但不限于 memchr、fastrand、hipstr、parking_lot、coarsetime 、bitcode、 sonic_rs、rapidhash::RapidHashMap 等高性能库替换标准库
如果需要一个并发读写的字典, 用 papaya
./src/ 中公开函数和结构体都在 ./src/lib.rs 导出，禁止 pub 模块（pub mod consts; 除外），而是 pub use 模块::{函数,结构体}; 内部用的用 pub(crate) ; 禁止 pub use 第三方库的函数
日志用 log；测试中用下面代码初始化日志显示

```
#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}
```

Cargo.toml 的 edition 用 2024, rust 用最新的函数、库（不用旧写法），比如：

- 支持 let-chain 语法
- 标准库的 fs 支持 try_lock
- trait 支持返回 impl Future 的方式定义异步函数
- 时间库用 jiff

每次写完记得 ./test.sh 测试编译