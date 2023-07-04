/// 两次 &Lazy::get_instance() 的地址不同是因为 Rust 中的引用类型在编译时会被优化为指针，
/// 而这个优化过程是在编译器级别完成的，因此每次调用 &Lazy::get_instance() 都会得到不同的地址。
/// 这并不意味着它们指向的是两个不同的对象，而只是表示它们在内存中的位置不同。
use crate::lib::lazy::static_impl::Lazy;

fn main() {
    //Lazy { data: 110 }
    println!("{:?}", Lazy::get_instance());
    //true
    println!("{}", &Lazy::get_instance() == &Lazy::get_instance());
    //0xda1d3ffad0
    println!("{:p}", &Lazy::get_instance());
    //0xda1d3ffb20
    println!("{:p}", &Lazy::get_instance());
}
