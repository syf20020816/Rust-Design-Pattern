//! # 过滤器模式
//! 过滤器模式（Filter Pattern）或标准模式（Criteria Pattern）是一种设计模式，
//! 这种模式允许开发人员使用不同的标准来过滤一组对象，通过逻辑运算以解耦的方式把它们连接起来。
//!
//! 这种类型的设计模式属于结构型模式，它结合多个标准来获得单一标准。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```

/// 被过滤的实体
#[derive(Debug, Clone)]
pub struct HttpUrl {
    protocol: String,
    base_url: String,
    is_static: bool,
}

impl HttpUrl {
    pub fn new(protocol: &str, base_url: &str, is_static: bool) -> Self {
        HttpUrl {
            protocol: String::from(protocol),
            base_url: String::from(base_url),
            is_static,
        }
    }
    pub fn get_protocol(&self) -> &str {
        &self.protocol
    }
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
    pub fn get_is_static(&self) -> &bool {
        &self.is_static
    }
}

/// 过滤器trait
/// 用于规范过滤器方法
pub trait Filter {
    fn filter(&self, urls: Vec<HttpUrl>) -> Vec<HttpUrl>;
}

///API过滤器
pub struct ApiFilter;

impl Filter for ApiFilter {
    fn filter(&self, urls: Vec<HttpUrl>) -> Vec<HttpUrl> {
        let mut res = vec![];
        for url in urls {
            if false.eq(url.get_is_static()) {
                res.push(url);
            }
        }
        res
    }
}

/// 静态资源过滤器
pub struct StaticFilter;

impl Filter for StaticFilter {
    fn filter(&self, urls: Vec<HttpUrl>) -> Vec<HttpUrl> {
        let mut res = vec![];
        for url in urls {
            if true.eq(url.get_is_static()) {
                res.push(url);
            }
        }
        res
    }
}

///Https+静态资源过滤器
pub struct HttpsStaticFilter;

impl HttpsStaticFilter {
    const HTTP: &'static str = "http";
    const HTTPS: &'static str = "https";
}

impl Filter for HttpsStaticFilter {
    fn filter(&self, urls: Vec<HttpUrl>) -> Vec<HttpUrl> {
        let mut res = vec![];
        for url in urls {
            match (url.get_protocol(), url.get_is_static()) {
                (HttpsStaticFilter::HTTPS, &true) => {
                    res.push(url);
                }
                _ => ()
            }
        }
        res
    }
}