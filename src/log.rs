pub fn __pretty_type_name(type_name: &str) -> String {
    let tn = &type_name[..type_name.len() - 3]; // 去掉最后的 '::f'
    // println!("========================> {}", tn);
    if let Some(r) = tn.rfind('>') {
        if let Some(l) = tn[..r].rfind('<') {
            let impl_str = &tn[l + 1..r];
            if let Some(as_pos) = impl_str.find(" as ") {
                let method = if let Some(v) = tn[r..].find("::") {
                    &tn[r + v + 2..]
                } else {
                    "?"
                };

                // 保留最后一两级
                let struct_ = impl_str[..as_pos].trim();
                let struct_ = if let Some(r) = struct_.rfind("::") {
                    &struct_[r + 2..]
                } else {
                    struct_
                };

                // 保留最后两级
                let trait_ = impl_str[as_pos + 4..].trim();
                let trait_ = if let Some(r) = trait_.rfind("::") {
                    if let Some(rr) = trait_[..r].rfind("::") {
                        &trait_[rr + 2..]
                    } else {
                        &trait_[r + 2..]
                    }
                } else {
                    trait_
                };

                return format!("{}.{} => {}", struct_, method, trait_);
            }
        }
    }

    // 最后保留一个冒号表示在 closure 中
    let tn = if tn.ends_with("::{{closure}}") { &tn[..tn.len() - 12] } else { &tn };

    // 保留最后两级
    if let Some(r) = tn.rfind("::") {
        if let Some(rr) = tn[..r].rfind("::") {
            // 选取的字段中有 {{closure}} 时保留三级
            if tn[rr + 2..].rfind("}}").is_some() {
                if let Some(rrr) = tn[..rr].rfind("::") {
                    return tn[rrr + 2..].to_string();
                }
            }
            tn[rr + 2..].to_string()
        } else {
            tn[r + 2..].to_string()
        }
    } else {
        tn.to_string()
    }
}

#[macro_export]
macro_rules! type_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let type_name = type_name_of(f);
        $crate::__pretty_type_name(type_name)
    }};
}

#[macro_export]
macro_rules! file_path {
    () => {{
        let f = file!();
        // 倒数第二级不是 'src' 时保留最后两级，否则保留一级
        if let Some(r) = f.rfind('/') {
            if let Some(rr) = f[..r].rfind('/') {
                if f[rr + 1..].starts_with("src/") {
                    &f[r + 1..]
                } else {
                    &f[rr + 1..]
                }
            } else {
                &f[r + 1..]
            }
        } else {
            f
        }
    }};
}

#[macro_export]
#[cfg(feature = "datetime")]
macro_rules! datetime {
    () => {{
        $crate::__private_chrono_local::now().format("%Y-%m-%d %H:%M:%S.%3f")
    }};
}

#[macro_export]
#[cfg(not(feature = "datetime"))]
macro_rules! datetime {
    () => {{
        ""
    }};
}

#[macro_export]
macro_rules! error {
    // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Error, $($arg)+));

    // error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::Level::Error, "{} E[{},L{}|{}] {}", $crate::datetime!(), $crate::file_path!(), line!(), $crate::type_name!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! warn {
    // warn!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // warn!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Warn, $($arg)+));

    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::Level::Warn, "{} W[{},L{}|{}] {}", $crate::datetime!(), $crate::file_path!(), line!(), $crate::type_name!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! info {
    // info!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // info!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Info, $($arg)+));

    // info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::Level::Info, "{} I[{},L{}|{}] {}", $crate::datetime!(), $crate::file_path!(), line!(), $crate::type_name!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! debug {
    // debug!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // debug!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Debug, $($arg)+));

    // debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::Level::Debug, "{} D[{},L{}|{}] {}", $crate::datetime!(), $crate::file_path!(), line!(), $crate::type_name!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! trace {
    // trace!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // trace!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Trace, $($arg)+));

    // trace!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!($crate::Level::Trace, "{} T[{},L{}|{}] {}", $crate::datetime!(), $crate::file_path!(), line!(), $crate::type_name!(), format_args!($($arg)+)))
}

// =============================================================================
// 以下 *_raw 宏不打印 file_path、line、type_name，用于转发 ffi 等非 rust 模块的日志

#[macro_export]
macro_rules! error_raw {
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Error, $($arg)+));
    ($($arg:tt)+) => ($crate::log!($crate::Level::Error, "{} E {}", $crate::datetime!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! warn_raw {
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Warn, $($arg)+));
    ($($arg:tt)+) => ($crate::log!($crate::Level::Warn, "{} W {}", $crate::datetime!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! info_raw {
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Info, $($arg)+));
    ($($arg:tt)+) => ($crate::log!($crate::Level::Info, "{} I {}", $crate::datetime!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! debug_raw {
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Debug, $($arg)+));
    ($($arg:tt)+) => ($crate::log!($crate::Level::Debug, "{} D {}", $crate::datetime!(), format_args!($($arg)+)))
}

#[macro_export]
macro_rules! trace_raw {
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, $crate::Level::Trace, $($arg)+));
    ($($arg:tt)+) => ($crate::log!($crate::Level::Trace, "{} T {}", $crate::datetime!(), format_args!($($arg)+)))
}
