mod lib;

use crate::lib::{HttpUrl, HttpsStaticFilter, StaticFilter, ApiFilter, Filter};

fn main() {
    let url1 = HttpUrl::new("https", "127.0.0.1:8808", true);
    let url2 = HttpUrl::new("http", "127.0.0.1:8809", true);
    let url3 = HttpUrl::new("https", "127.0.0.1:8810/api", false);
    let url4 = HttpUrl::new("http", "127.0.0.1:8808/api", false);
    let url5 = HttpUrl::new("http", "127.0.0.1:8808", true);
    let urls = vec![url1, url2, url3, url4, url5];
    let api_filter = ApiFilter;
    let static_filter = StaticFilter;
    let https_static_filter = HttpsStaticFilter;
    let res1 = api_filter.filter(urls.clone());
    let res2 = static_filter.filter(urls.clone());
    let res3 = https_static_filter.filter(urls);
    dbg!(res1);
    dbg!(res2);
    dbg!(res3);

}
