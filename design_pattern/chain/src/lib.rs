//! # 责任链模式
//!
//! 在现实生活中，常常会出现这样的事例:一个请求有多个对象可以处理，但每个对象的处理条件或权限不同。
//! 例如，公司员工请假，可批假的领导有部门负责人、副总经理、总经理等，但每个领导能批准的天数不同，员工必须根据自己要请假的天数去找不同的领导签名，也就是说员工必须记住每个领导的姓名、电话和地址等信息，这增加了难度。这样的例子还有很多，如找领导出差报销、生活中的"击鼓传花′游戏等。
//!
//! 又名职责链模式，为了避免请求发送者与多个请求处理者耦合在一起，将所有请求的处理者通过前一对象记住其下一个对象的引用而连成一条链;当有请求发生时，可将请求沿着这条链传递，直到有对象处理它为止。
//!
//! ## 结构
//!
//! 1. 抽象处理者(Handler)角色:定义一个处理请求的接口，包含抽象处理方法和一个后继连接。
//! 2. 具体处理者 (Concrete Handler)角色:实现抽象处理者的处理方法，判断能否处理本次请求，如果可以处理请求则处理，否则将该请求转给它的后继者。
//! 3. 客户类(client)角色:创建处理链，并向链头的具体处理者对象提交请求，它不关心处理细节和请求的传递过程。
//!
//! ## 优点
//!
//! 1. 降低了对象之间的耦合度：该模式降低了请求发送者和接收者的耦合度。
//! 2. 增强了系统的可扩展性：可以根据需要增加新的请求处理类，满足开闭原则。
//! 3. 增强了给对象指派职责的灵活性：当工作流程发生变化，可以动态地改变链内的成员或者修改它们的次序，也可动态地新增或者删除责任。
//! 4. 责任链简化了对象之间的连接：一个对象只需保持一个指向其后继者的引用，不需保持其他所有处理者的引用，这避免了使用众多的if 或者 if: …else语句。
//! 5. 责任分担：每个类只需要处理自己该处理的工作，不能处理的传递给下一个对象完成，明确各类的责任范围，符合类的单一职责原则。
//!
//! ## 缺点
//!
//! 1. 不能保证每个请求一定被处理。由于一个请求没有明确的接收者，所以不能保证它一定会被处理，该请求可能一直传到链的末端都得不到处理。
//! 2. 对比较长的职责链，请求的处理可能涉及多个处理对象，系统性能将受到一定影响。
//! 3. 职责链建立的合理性要靠客户端来保证，增加了客户端的复杂性，可能会由于职责链的错误设置而导致系统出错，如可能会造成循环调用。
//!
//! ## 使用场景
//!
//! 链式工作，如FilterChain
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/11
//! @version:0.0.1
//! @description:
//! ```

const DAY_LITTLE: isize = 1_isize;
const DAY_MID: isize = 4_isize;
const DAT_LARGE: isize = 10_isize;

/// # 抽象处理者
/// 分为三个等级
/// ## 优化
/// 可以考虑使用Range
pub trait Handler {
    fn new() -> Self where Self: Sized;
    fn handle(&self, level: isize) -> ();
    fn next(&self) -> Box<dyn Handler>;
}

pub struct GroupLeader {
    start: isize,
    end: isize,
}

impl Handler for GroupLeader {
    fn new() -> Self where Self: Sized {
        GroupLeader {
            start: DAY_LITTLE,
            end: DAY_MID,
        }
    }

    fn handle(&self, level: isize) -> () {
        /// 使用如下代码采取usize防止减法溢出：self.start.checked_sub(level).is_some()
        match ((level - self.start) >= 0) && ((self.end - level) >= 0) {
            true => {
                println!("{}", "group leader : OK")
            }
            false => {
                println!("{}", "to next");
                //next handler
                &self.next().handle(level);
            }
        }
    }

    fn next(&self) -> Box<dyn Handler> {
        Box::new(Manager::new())
    }
}

pub struct Manager {
    start: isize,
    end: isize,
}

impl Handler for Manager {
    fn new() -> Self where Self: Sized {
        Manager {
            start: DAY_MID,
            end: DAT_LARGE,
        }
    }

    fn handle(&self, level: isize) -> () {
        match ((level - self.start) >= 0) && ((self.end - level) >= 0) {
            true => {
                println!("{}", "manager : OK")
            }
            false => {
                //next handler
                &self.next();
            }
        }
    }

    fn next(&self) -> Box<dyn Handler> {
        panic!("no handler match!")
    }
}
