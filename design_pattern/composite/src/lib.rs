//!# 组合模式
//!
//! 又名部分整体模式，是用于把一组相似的对象当作一个单一的对象。 组合模式依据树形结构来组合对象，用来表示部分以及整体层次。这种类型的设计模式属于结构型模式，它创建了对象组的树形结构
//!
//! ## 组合模式结构
//!
//! 1.  抽象根节点(Component) ：定义系统各层次对象的共有方法和属性,可以预先定义- -些默认行为和属性。
//! 2.  树枝节点(Composite) ：定义树枝节点的行为，存储子节点，组合树枝节点和叶子节点形成一个树形结构。
//! 3.  叶子节点(Leaf) ：叶子节点对象，其下再无分支，是系统层次遍历的最小单位。
//!
//! ## 组合模式的分类
//!
//! 1. 透明组合模式
//! 2. 安全组合模式
//!
//! ### 透明组合模式
//!
//! 透明组合模式中，抽象根节点角色免声明了所有用于管理成员对象的方法，这样做的好处是确保所有的构件类都有相同的接口。透明组合模式也是组合模式的标准形式
//!
//! 透明组合模式的缺点是不够安全，因为叶子对象和容器对象在本质上是有区别的,叶子对象不可能有下一个层次的对象，即不可能包含成员对象，因此为其提供非必要重写方法是没有意义的， 这在编译阶段不会出错，但在运行阶段如果调用这些方法可能会出错(如果没有提供相应的错误处理代码)
//!
//! ### 安全组合模式
//!
//! 在安全组合模式中，在抽象构件角色中没有声明任何用于管理成员对象的方法，而是在树枝节点Menu 类中声明并实现这些方法。安全组合模式的缺点是不够透明，因为叶子构件和容器构件具有不同的方法，且容器构件中那些用于管理成员对象的方法没有在抽象构件类中定义，因此客户端不能完全针对抽象编程，必须有区别地对待叶子构件和容器构件
//!
//! ## 优点
//!
//! 1. 组合模式可以清楚地定义分层次的复杂对象，表示对象的全部或部分层次，它让客户端忽略了层次的差异，方便对整个层次结构进行控制。
//! 2. 客户端可以一致地使用一一个组合结构或其中单个对象,不必关心处理的是单个对象还是整个组合结构,简化了客户端代码。
//! 3. 在组合模式中增加新的树枝节点和叶子节点都很方便，无须对现有类库进行任何修改，符合“开闭原则”。
//! 4. 组合模式为树形结构的面向对象实现提供了-种灵活的解决方案，通过叶子节点和树枝节点的递归组合，可以形成复杂的树形结构,但对树形结构的控制却非常简单。
//!
//! ## 适用场景
//!
//! 组合模式正是应树形结构而生,所以组合模式的使用场景就是出现树形结构的地方。比如:文件目录显示，多级目录呈现等树形结构数据的操作
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```

/// # 抽象根节点
/// 无论是树枝还是叶子都要实现
pub trait Component {
    fn new(name: &str, level: usize) -> Self where Self: Sized;
    fn add(&mut self, component: Box<dyn Component>) -> ();
    fn remove(&mut self, index: usize) -> ();
    fn get_child(&self, index: usize) -> &Box<dyn Component>;
}

/// # 树枝节点
/// 树枝节点下包含n个叶子节点或树枝节点
/// ``` code
///  树枝
///   ↓
///   * ---
///      |-→ *  ← 叶子
///      |-→ * ---    ← 树枝
///             |-→ *  ← 叶子
/// ```
pub struct Branch {
    name: String,
    level: usize,
    components: Vec<Box<dyn Component>>,
}


impl Component for Branch {
    fn new(name: &str, level: usize) -> Self where Self: Sized {
        Branch {
            name: String::from(name),
            level,
            components: vec![],
        }
    }

    fn add(&mut self, component: Box<dyn Component>) -> () {
        self.components.push(component);
    }

    fn remove(&mut self, index: usize) -> () {
        self.components.remove(index);
    }

    fn get_child(&self, index: usize) -> &Box<dyn Component> {
        &self.components[index]
    }
}

/// # 叶子节点
/// 末节点
#[derive(Debug)]
pub struct Leaf {
    name: String,
    level: usize,
}

impl Component for Leaf {
    fn new(name: &str, level: usize) -> Self where Self: Sized {
        Leaf { name: String::from(name), level }
    }

    fn add(&mut self, component: Box<dyn Component>) -> () {
        panic!("add component failed")
    }

    fn remove(&mut self, index: usize) -> () {
        panic!("remove component failed")
    }

    fn get_child(&self, index: usize) -> &Box<dyn Component> {
        panic!("get component failed")
    }
}
