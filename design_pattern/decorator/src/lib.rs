//! # 装饰者模式
//!
//! 指在不改变现有对象结构的情况下，动态地给该对象增加一些职责(即增加其额外功能)的模式。
//!
//! ## 装饰者模式的结构
//!
//! 1. 抽象构件(Component) 角色: 定义一个抽象接口以规范准备接收附加责任的对象。
//! 2. 具体构件(Concrete Component) 角色：实现抽象构件，通过装饰角色为其添加一些职责。
//! 3. 抽象装饰(Decorator) 角色：继承或实现抽象构件, 并包含具体构件的实例，可以通过其子类扩展具体构件的功能。
//! 4. 具体装饰(ConcreteDecorator) 角色：实现抽象装饰的相关方法，并给具体构件对象添加附加的责任。
//!
//! ## 优点
//!
//! 1. 装饰者模式可以带来比继承更加灵活性的扩展功能，使用更加方便，可以通过组合不同的装饰者对象来获取具有不同行为状态的多样化的结果。装饰者模式比继承更具良好的扩展性，完美的遵循开闭原则，继承是静态的附加责任，装饰者则是动态的附加责任。
//! 2. 装饰类和被装饰类可以独立发展，不会相互耦合，装饰模式是继承的一个替代模式，装饰模式可以动态扩展-个实现类的功能。
//!
//! ## 使用场景
//!
//! 1. 当不能采用继承的方式对系统进行扩充或者采用继承不利于系统扩展和维护时。
//!    不能采用继承的情况主要有两类:
//!    1. 第一类是系统中存在大量独立的扩展,为支持每一 种组合将产生大量的子类,使得子类数目呈爆炸性增长;
//!    2. 第二类是因为类定义不能继承
//! 2. 在不影响其他对象的情况下，以动态、透明的方式给单个对象添加职责。
//! 3. 当对象的功能要求可以动态地添加，也可以再动态地撤销时。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/9
//! @version:0.0.1
//! @description:
//! ```

pub trait Gun{
    fn new(part:Box<dyn Parts>)->Self where Self:Sized;
    fn cost(&mut self)->();
}

pub struct HandGun{
    price:usize,
    part:Box<dyn Parts>
}

impl Gun for HandGun{
    fn new(part: Box<dyn Parts>) -> Self where Self:Sized,{
        HandGun{
            price: 3600,
            part
        }
    }

    fn cost(&mut self) -> () {
        //进行组合
        self.price += self.part.cost();
        println!("{}",&self.price)
    }
}

pub trait Parts{
    fn new()->Self where Self:Sized;
    fn cost(&self)->usize;
}

pub struct Grip{
    price:usize,
}

impl Parts for Grip{
    fn new() -> Self where Self:Sized,{
        Grip{
            price:260
        }
    }

    fn cost(&self) -> usize {
        self.price
    }
}