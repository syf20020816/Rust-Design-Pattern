//!# 解释器模式
//!
//! 给定一个语言，定义它的文法表示，并定义一个解释器，这个解释器使用该标识来解释语言中的句子。
//!
//! 在解释器模式中，我们需要将待解决的问题，提取出规则，抽象为一种""语言”。比如加减法运算，规则为:由数值和+-符号组成的合法序列，"1+3-2”就是这种语言的句子。
//!
//! 解释器就是要解析出来语句的含义。但是如何描述规则呢?
//!
//! ## 文法(语法)规则:
//!
//! 文法是用于描述语言的语法结构的形式规则。
//!
//! ``` code
//! expression ::= value | plus | minus
//! plus : := expression '+' expression
//! minus : := expression '-' expression
//! value ::= integer
//! ```
//! 表达式可以是一个值，也可以是plus或者minus运算，而plus和minus又是由表达式结合运算符构成，值的类型为整型数。
//!
//! ## 结构
//!
//! 1. 抽象表达式(Abstract Expression)角色:定义解释器的接口，约定解释器的解释操作，主要包含解释方法interpret()。
//! 2. 终结符表达式(Terminal Expression)角色:是抽象表达式的子类，用来实现文法中与终结符相关的操作，文法中的每一个终结符都有一个具体终结表达式与之相对应。
//! 3. 非终结符表达式(Wonterminal Expression)角色:也是抽象表达式的子类，用来实现文法中与非终结符相关的操作，文法中的每条规则都对应于一个非终结符表达式。
//! 4. 环境(Context)角色:通常包含各个解释器需要的数据或是公共的功能，一般用来传递被所有解释器共享的数据，后面的解释器可以从这里获取这些值。
//! 5. 客户端(client):主要任务是将需要分析的句子或表达式转换成使用解释器对象描述的抽象语法树，然后调用解释器的解释方法，当然也可以通过环境角色间接访问解释器的解释方法。工
//!
//! ## 优点
//!
//! 1. 易于改变和扩展文法：由于在解释器模式中使用类来表示语言的文法规则，因此可以通过继承等机制来改变或扩展文法。每一条文法规则都可以表示为一个类，因此可以方便地实现一个简单的语言。
//! 2. 实现文法较为容易：在抽象语法树中每一个表达式节点类的实现方式都是相似的，这些类的代码编写都不会特别复杂。
//! 3. 增加新的解释表达式较为方便：如果用户需要增加新的解释表达式只需要对应增加一个新的终结符表达式或非终结符表达式类，原有表达式类代码无须修改，符合“开闭原则"。
//!
//! ## 缺点
//!
//! 1. 对于复杂文法难以维护：在解释器模式中，每一条规则至少需要定义一个类，因此如果一个语言包含太多文法规则，类的个数将会急剧增加，导致系统难以管理和维护。
//! 2. 执行效率较低：由于在解释器模式中使用了大量的循环和递归调用，因此在解释较为复杂的句子时其速度很慢，而且代码的调试过程也比较麻烦。
//!
//! ## 使用场景
//!
//! 1. 当语言的文法较为简单，且执行效率不是关键问题时。
//! 2. 当问题重复出现，且可以用一种简单的语言来进行表达时。
//! 3. 当一个语言需要解释执行，并且语言中的句子可以表示为一个抽象语法树的时候。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/12
//! @version:0.0.1
//! @description:
//! ```

use std::collections::HashMap;

///抽象表达式
pub trait Expression {
    fn interpret(&self, context: &Context) -> isize;
    fn show(&self) -> String;
}

/// 非终结符表达式
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Self { Var { name: String::from(name) } }
}

impl Expression for Var {
    fn interpret(&self, context: &Context) -> isize {
        context.get(&self.name)
    }

    fn show(&self) -> String {
        String::from(&self.name)
    }
}

/// 环境角色
pub struct Context {
    vars: HashMap<Var, isize>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            vars: HashMap::new()
        }
    }
    pub fn add(&mut self, name: Var, value: isize) -> () {
        self.vars.insert(name, value);
    }
    pub fn get(&self, name: &str) -> isize {
        *self.vars.get(&Var::new(name)).unwrap()
    }
}

pub struct Minus {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Minus {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self where Self: Sized, {
        Minus {
            left,
            right,
        }
    }
}

impl Expression for Minus {
    /// 运算规则:
    /// - 深入查找Var::interpret() -> 获取环境Context中存储的值
    fn interpret(&self, context: &Context) -> isize {
        self.left.interpret(context) - self.right.interpret(context)
    }

    fn show(&self) -> String {
        format!("{} - {}", &self.left.show(), &self.right.show())
    }
}
