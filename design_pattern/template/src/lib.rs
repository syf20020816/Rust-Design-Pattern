//! # 模板方法
//!
//! 在面向对象程序设计过程中，程序员常常会遇到这种情况:设计一个系统时知道了算法所需的关键步骤，而且确定了这些步骤的执行顺序，但某些步骤的具体实现还未知，或者说某些步骤的实现与具体的环境相关。
//!
//! 定义一个操作中的算法骨架，而将算法的一些步骤延迟到子类中，使得子类可以不改变该算法结构的情况下重定义该算法的某些特定步骤。
//!
//! ## 结构
//!
//! 模板方法(Template Method)模式包含以下主要角色:
//!
//! 1. 抽象类(Abstract class)∶负责给出一个算法的轮廓和骨架。它由一个模板方法和若干个基本方法构成。
//!    1. 模板方法:定义了算法的骨架，按某种顺序调用其包含的基本方法。
//!    2. 基本方法:是实现算法各个步骤的方法，是模板方法的组成部分。基本方法又可以分为三种:
//!       1. 抽象方法(Abstract Method) :一个抽象方法由抽象类声明、由其具体子类实现。
//!       2. 具体方法(Concrete Method) :一个具体方法由一个抽象类或具体类声明并实现，其子类可以进行覆盖也可以直接继承。
//!       3. 钩子方法(Hook Method)︰在抽象类中已经实现，包括用于判断的逻辑方法和需要子类重写的空方法两种。一般钩子方法是用于判断的逻辑方法，这类方法名一般为isXxx，返回值类型为boolean类型。
//! 2. 具体子类(Concrete Class)︰实现抽象类中所定义的抽象方法和钩子方法，它们是一个顶级逻辑的组成步骤。
//!
//! ## 优点
//!
//! 1. 提高代码复用性：将相同部分的代码放在抽象的父类中，而将不同的代码放入不同的子类中。
//! 2. 实现了反向控制：通过一个父类调用其子类的操作，通过对子类的具体实现扩展不同的行为，实现了反向控制，并符合"开闭原则”。
//!
//! ## 缺点
//!
//! 1. 对每个不同的实现都需要定义一个子类，这会导致类的个数增加，系统更加庞大，设计也更加抽象。
//! 2. 父类中的抽象方法由子类实现，子类执行的结果会影响父类的结果，这导致一种反向的控制结构，它提高了代码阅读的难度。
//!
//! ## 使用场景
//!
//! 1. 算法的整体步骤很固定，但其中个别部分易变时，这时候可以使用模板方法模式，将容易变的部分抽象出来，供子类实现。
//! 2. 需要通过子类来决定父类算法中某个步骤是否执行，实现子类对父类的反向控制。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

/// 模板
pub trait GameTemplate {
    fn init() -> Self;
    fn start(&self) -> () {
        println!("{}", "game start");
    }
    fn end(&self, hook: impl Fn() -> ()) -> () {
        println!("{}", "game over");
    }
}

/// 具体模板
pub struct SportGame;

impl GameTemplate for SportGame {
    fn init() -> Self {
        SportGame
    }
}

pub struct MobaGame {
    left: String,
    right: String,
}

impl MobaGame {
    pub fn win(&self, is_left: bool) -> &str {
        if is_left {
            &self.left
        } else {
            &self.right
        }
    }
}

impl GameTemplate for MobaGame {
    fn init() -> Self {
        MobaGame {
            left: String::from("IG"),
            right: String::from("EDG"),
        }
    }
    fn end(&self, hook: impl Fn() -> ()) -> () {
        hook();
    }
}