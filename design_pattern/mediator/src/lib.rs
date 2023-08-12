//!# 中介者模式
//!
//! 一般来说，同事类之间的关系是比较复杂的，多个同事类之间互相关联时，他们之间的关系会呈现为复杂的网状结构，这是一种过度耦合的架构，即不利于类的复用，也不稳定。
//! 例如有六个同事类对象，假如对象1发生变化，那么将会有4个对象受到影响。如果对象2发生变化，那么将会有5个对象受到影响。也就是说，同事类之间直接关联的设计是不好的。
//! 如果引入中介者模式，那么同事类之间的关系将变为星型结构，任何一个类的变动只会影响的类本身，以及中介者，这样就减小了系统的耦合。
//! 一个好的设计，必定不会把所有的对象关系处理逻辑封装在本类中，而是使用一个专门的类来管理那些不属于自己的行为。
//!
//! 又叫调停模式，定义一个中介角色来封装一系列对象之间的交互，使原有对象之间的耦合松散，且可以独立地改变它们之间的交互。
//! ## 结构
//!
//! 1. 抽象中介者(Mediator)角色:它是中介者的接口，提供了同事对象注册与转发同事对象信息的抽象方法。
//! 2. 具体中介者(ConcreteMediator)角色:实现中介者接口，定义一个List 来管理同事对象，协调各个同事角色之间的交互关系，因此它依赖于同事角色。
//! 3. 抽象同事类（colleague)角色:定义同事类的接口，保存中介者对象，提供同事对象交互的抽象方法，实现所有相互影响的同事类的公共功能。
//! 4. 具体同事类(Concrete Colleague)角色:是抽象同事类的实现者，当需要与其他同事对象交互时，由中介者对象负责后续的交互。
//!
//! ## 优点
//!
//! 1. 松散耦合：中介者模式通过把多个同事对象之间的交互封装到中介者对象里面，从而使得同事对象之间松散耦合，基本上可以做到互补依赖。这样一来，同事对象就可以独立地变化和复用，而不再像以前那样"牵一处而动全身"了。
//! 2. 集中控制交互：多个同事对象的交互，被封装在中介者对象里面集中管理，使得这些交互行为发生变化的时候，只需要修改中介者对象就可以了，当然如果是已经做好的系统，那么就扩展中介者对象，而各个同事类不需要做修改。
//! 3. 一对多关联转变为一对一的关联：没有使用中介者模式的时候，同事对象之间的关系通常是一对多的，引入中介者对象以后，中介者对象和同事对象的关系通常变成双向的一对一，这会让对象的关系更容易理解和实现。
//!
//! ## 缺点
//!
//! 当同事类太多时，中介者的职责将很大，它会变得复杂而庞大，以至于系统难以维护。
//!
//! ## 使用场景
//!
//! 1. 系统中对象之间存在复杂的引用关系，系统结构混乱且难以理解。
//! 2. 当想创建一个运行于多个类之间的对象，又不想生成新的子类时
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/12
//! @version:0.0.1
//! @description:
//! ```

/// 抽象同事
pub trait Person {
    fn new(name: &str) -> Self where Self: Sized;
    fn chat(&self, mediator: &dyn Mediator, msg: &str) -> ();
}

/// 定义具体同事角色
pub struct HouseOwner {
    name: String,
}

impl Person for HouseOwner {
    fn new(name: &str) -> Self where Self: Sized {
        HouseOwner {
            name: String::from(name),
        }
    }

    fn chat(&self, mediator: &dyn Mediator, msg: &str) -> () {
        mediator.chat(msg, true)
    }
}

pub struct HouseBuyer {
    name: String,

}

impl Person for HouseBuyer {
    fn new(name: &str) -> Self where Self: Sized {
        HouseBuyer {
            name: String::from(name),
        }
    }

    fn chat(&self, mediator: &dyn Mediator, msg: &str) -> () {
        mediator.chat(msg, false)
    }
}

/// 抽象中介者
pub trait Mediator {
    fn chat(&self, msg: &str, is_owner: bool) -> ();
}

/// 具体中介者
/// - 使用泛型参数代替动态分发：如果可能的话，使用泛型参数而不是 trait 对象来表示中介者对象的参与者。这样可以避免运行时的动态分发，提高性能和类型安全性。
/// - 使用引用代替 Box<T>：如果你不需要对参与者进行所有权转移或拥有动态大小类型，请考虑使用引用而不是 Box<T>。引用具有更高的性能和更简洁的语法。
/// - 考虑使用生命周期参数：如果你的中介者对象在创建时就持有参与者的引用，并且这些引用的生命周期应该与中介者对象的生命周期相关联，那么可以使用生命周期参数来明确指定这种关系。
pub struct RealEstateManager<'a, 'b, Own: Person, Buy: Person> {
    house_owner: &'a Own,
    house_buyer: &'b Buy,
}

impl<'a, 'b, Own: Person, Buy: Person> RealEstateManager<'a, 'b, Own, Buy> {
    pub fn new(owner: &'a Own, buyer: &'b Buy) -> Self where Self: Sized, {
        RealEstateManager {
            house_owner: owner,
            house_buyer: buyer,
        }
    }
}

impl<'a, 'b, Own: Person, Buy: Person> Mediator for RealEstateManager<'a, 'b, Own, Buy> {
    fn chat(&self, msg: &str, is_owner: bool) -> () {
        if is_owner {
            println!("To buyer : {}", msg)
        } else {
            println!("To owner : {}", msg)
        }
    }
}