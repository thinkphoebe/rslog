use env_logger;
// use log_ori::kv::Source;
use rslog as log;
use std::sync::Once;

static INIT: Once = Once::new();

// struct Printer;
//
// impl<'kvs> log_ori::kv::VisitSource<'kvs> for Printer {
//     fn visit_pair(
//         &mut self,
//         key: log_ori::kv::Key<'kvs>,
//         value: log_ori::kv::Value<'kvs>
//     ) -> Result<(), log_ori::kv::Error> {
//         println!("[kv_printer] {key}: {value}");
//         Ok(())
//     }
// }

// 简单的日志初始化函数
fn setup_logger() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter_level(log_ori::LevelFilter::Trace)
            .format(|buf, record| {
                use std::io::Write;
                // _ = record.key_values().visit(&mut Printer {});
                writeln!(buf, "{}", record.args())
            })
            .init();
    });
}

// 顶层函数
fn top_level_function() {
    log::error!("这是一个错误日志");
    log::warn!("这是一个警告日志");
    log::info!("这是一个信息日志");
    log::debug!("这是一个调试日志");
    log::trace!("这是一个跟踪日志");
}

// 嵌套模块结构
mod outer_module {
    use super::*;

    pub fn outer_function() {
        log::error!("外层模块中的错误日志");
        log::warn!("外层模块中的警告日志");
    }

    pub mod inner_module {
        use super::*;

        pub fn inner_function() {
            log::info!("内层模块中的信息日志");
            log::debug!("内层模块中的调试日志");
        }

        pub struct MyStruct;

        impl MyStruct {
            pub fn struct_method(&self) {
                log::trace!("结构体方法中的跟踪日志");
            }
        }

        pub trait MyTrait {
            fn trait_method(&self);
        }

        impl MyTrait for MyStruct {
            fn trait_method(&self) {
                log::error!("特征实现中的错误日志");
            }
        }
    }
}

// 使用闭包和特征的复杂示例，类似于用户提供的示例
fn complex_example() {
    (|| {
        mod complex_module {
            use super::*;

            pub trait ComplexTrait {
                fn complex_function(&self) {
                    println!("模块路径: {}", module_path!());
                    println!("文件: {}:{}:{}", file!(), line!(), column!());

                    // 使用日志宏
                    log::error!("特征方法中的错误日志");
                    log::warn!("特征方法中的警告日志");
                    log::info!("特征方法中的信息日志");
                    log::debug!("特征方法中的调试日志");
                    log::trace!("特征方法中的跟踪日志");
                }
            }

            impl ComplexTrait for () {}
        }

        complex_module::ComplexTrait::complex_function(&());
    })()
}

// 使用更复杂的嵌套结构
fn nested_complex_example() {
    struct Outer;

    impl Outer {
        fn outer_method(&self) {
            struct Inner;

            impl Inner {
                fn inner_method(&self) {
                    log::error!("嵌套结构中的错误日志");
                }
            }

            let inner = Inner;
            inner.inner_method();

            // 在闭包中使用日志
            (|| {
                log::warn!("嵌套结构中闭包的警告日志");
            })();
        }
    }

    let outer = Outer;
    outer.outer_method();
}

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    setup_logger();

    println!("=== 顶层函数日志 ===");
    top_level_function();

    println!("\n=== 外层模块函数日志 ===");
    outer_module::outer_function();

    println!("\n=== 内层模块函数日志 ===");
    outer_module::inner_module::inner_function();

    println!("\n=== 结构体方法日志 ===");
    let my_struct = outer_module::inner_module::MyStruct;
    my_struct.struct_method();

    println!("\n=== 特征实现日志 ===");
    // 需要导入特征才能使用特征方法
    use outer_module::inner_module::MyTrait;
    my_struct.trait_method();

    println!("\n=== 复杂示例日志 ===");
    complex_example();

    println!("\n=== 嵌套复杂示例日志 ===");
    nested_complex_example();

    // log_ori::error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log");
    //
    // let a = 1;
    // log_ori::info!(a; "Something of interest");

    let a = 123;
    log::error!("{}", a);
    log::warn!("{}", a);
    log::info!("{}", a);
    log::debug!("{}", a);
    log::trace!("{}", a);
}
