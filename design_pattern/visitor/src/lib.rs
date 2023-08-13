//! # 访问者模式
//!
//! 封装一些作用于某种数据结构中的各元素的操作，它可以在不改变这个数据结构的前提下定义作用于这些元素的新的操作。
//!
//! ## 结构
//!
//! 1. 抽象访问者(Visitor)角色:定义了对每一个元素(Element)访问的行为，它的参数就是可以访问的元素，它的方法个数理论上来讲与元素类个数(Element的实现类个数)是一样的，从这点不难看出，访问者模式要求元素类的个数不能改变。
//! 2. 具体访问者（ConcreteVisitor)角色:给出对每一个元素类访问时所产生的具体行为。
//! 3. 抽象元素(Element)角色:定义了一个接受访问者的方法（accept)，其意义是指，每一个元素都要可以被访问者访问。
//! 4. 具体元素(ConcreteElement)角色: 提供接受访问方法的具体实现，而这个具体的实现，通常情况下是使用访问者提供的访问该元素类的方法。
//! 5. 对象结构(object structure)角色:定义当中所提到的对象结构，对象结构是一个抽象表述，具体点可以理解为一个具有容器性质或者复合对象特性的类，它会含有一组元素(Element)，并且可以迭代这些元素，供访问者访问。
//!
//! ## 优点
//!
//! 1. 扩展性好:在不修改对象结构中的元素的情况下，为对象结构中的元素添加新的功能。
//! 2. 复用性好:通过访问者来定义整个对象结构通用的功能，从而提高复用程度。
//! 3. 分离无关行为:通过访问者来分离无关的行为，把相关的行为封装在一起，构成一个访问者，这样每一个访问者的功能都比较单一。
//!
//! ## 缺点
//!
//! 1. 对象结构变化很困难:在访问者模式中，每增加一个新的元素类，都要在每一个具体访问者类中增加相应的具体操作，这违背了"开闭原则"。
//! 2. 违反了依赖倒置原则:访问者模式依赖了具体类，而没有依赖抽象类。
//!
//! ## 使用场合
//!
//! 1. 对象结构相对稳定，但其操作算法经常变化的程序。
//! 2. 对象结构中的对象需要提供多种不同且不相关的操作，而且要避免让这些操作的变化影响对象的结构。
//!
//!
//!- 编译器和解释器：访问者模式可以用于遍历抽象语法树（AST）并对不同类型的节点执行特定的操作，例如类型检查、代码生成等。
//!- 数据结构和算法：当需要对数据结构中的元素执行不同的操作，例如统计、搜索、过滤等时，访问者模式非常有用。
//!- UI 组件库：访问者模式可以用于在组件层次结构中的各个组件上执行不同的操作，例如绘制、更新、事件处理等。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

/// 抽象元素
pub trait Element {
    fn accept(&self, visitor: &dyn Visitor) -> ();
}

/// 具体元素
pub struct ElementA;

pub struct ElementB;

impl Element for ElementA {
    fn accept(&self, visitor: &dyn Visitor) -> () {
        visitor.visit_a(&self);
    }
}

impl Element for ElementB {
    fn accept(&self, visitor: &dyn Visitor) -> () {
        visitor.visit_b(&self);
    }
}

pub trait Visitor {
    fn visit_a(&self, el: &ElementA) -> ();
    fn visit_b(&self, el: &ElementB) -> ();
}

pub struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_a(&self, el: &ElementA) -> () {
        println!("{}", "El_A")
    }

    fn visit_b(&self, el: &ElementB) -> () {
        println!("{}", "El_B")
    }
}
