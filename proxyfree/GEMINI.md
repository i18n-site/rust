安装包依赖都用 cargo add ,不可直接编辑 Cargo.toml
要用最新写法，禁止用过时的模块
use 要写明具体的导入模块
注释都写中文
避免使用 unwrap
格式化字符串尽量把变量名写到字符串中,比如 format!("{varname}")
锁尽量用 parking_lot
如果需要一个并发读写的字典, 用 DashMap
src/ 错误都在 src/error.rs 中定义, 不要用 anyhow
examples/ 和 tests/ 中的错误可以用 anyhow, 比如 main 都定义为 fn main()->Result<(), anyhow::Error>
参数要秉承最小化原则,能传部分属性,就不要传入整个结构体
函数名,变量名都要简洁
每次写完记得 cargo test --all-features -- --nocapture 测试编译,测试通过 commit 代码,并用中文写 commit message
遇到问题,多用 dbg!进行调试