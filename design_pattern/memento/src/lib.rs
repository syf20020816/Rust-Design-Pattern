//! # 备忘录模式
//!
//! 备忘录模式提供了一种状态恢复的实现机制，使得用户可以方便地回到一个特定的历史步骤，当新的状态无效或者存在问题时，可以使用暂时存储起来的备忘录将状态复原，很多软件都提供了撤销（Undo)操作，如 word、记事本、Photoshop,IDEA等软件在编辑时按ctrl+Z组合键时能撤销当前操作，使文档恢复到之前的状态，还有在浏览器中的后退键、数据库事务管理中的回滚操作、玩游戏时的中间结果存档功能、数据库与操作系统的备份操作、棋类游戏中的悔棋功能等都属于这类。
//!
//! 又叫快照模式，在不破坏封装性的前提下，捕获一个对象的内部状态，并在该对象之外保存这个状态，以便以后当需要时能将该对象恢复到原先保存的状态。
//!
//! ## 结构
//!
//! 1. 发起人(originator)角色:记录当前时刻的内部状态信息，提供创建备忘录和恢复备忘录数据的功能，实现其他业务功能，它可以访问备忘录里的所有信息。
//! 2. 备忘录(Memento)急色:负责存储发起人的内部状态，在需要的时候提供这些内部状态给发起人。
//! 3. 管理者(Caretaker)角色:对备忘录进行管理，提供保存与获取备忘录的功能，但其不能对备忘录的内容进行访问与修改。
//!
//! > 备忘录有两个等效的接口:
//! >
//! > 1. 窄接口:管理者(Caretaker)对象（和其他发起人对象之外的任何对象）看到的是备忘录的窄接口(narror Interface)，这个窄接口只允许他把备忘录对象传给其他的对象。
//! > 2. 宽接口:与管理者看到的窄接口相反，发起人对象可以看到一个宽接口(wide Interface)，这个宽接口允许它读取所有的数据，以便根据这些数据恢复这个发起人对象的内部状态。
//!
//! ## 优点
//!
//! 1. 提供了一种可以恢复状态的机制。当用户需要时能够比较方便地将数据恢复到某个历史的状态。
//! 2. 实现了内部状态的封装。除了创建它的发起人之外，其他对象都不能够访问这些状态信息。
//! 3. 简化了发起人类。发起人不需要管理和保存其内部状态的各个备份，所有状态信息都保存在备忘录中，并由管理者进行管理，这符合单一职责原则。
//!
//! ## 缺点
//!
//! 资源消耗大。如果要保存的内部状态信息过多或者特别频繁，将会占用比较大的内存资源。
//!
//! ## 使用场景
//!
//! 1. 需要保存与恢复数据的场景，如玩游戏时的中间结果的存档功能。
//! 2. 需要提供一个可回滚操作的场景，如mord、记事本、Ehotoshop，idea等软件在编辑时按ctrl+z组合键，还有数据库中事务操作。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

/// # 发起者
/// 获取当前状态，调用管理者进行存储和恢复
#[derive(Clone, Debug)]
pub struct Originator {
    version: usize,
    data: String,
}

impl Originator {
    pub fn new(version: usize, data: &str) -> Self {
        Originator {
            version,
            data: String::from(data),
        }
    }
    pub fn from(&mut self, obj: &Originator) -> Self {
        Self {
            version: obj.version,
            data: obj.data.clone(),
        }
    }
}

/// # 备忘录
/// 存储状态,由管理者进行调用
#[derive(Clone, Debug)]
pub struct Memento {
    data: Vec<Originator>,
}

impl Memento {
    pub fn new() -> Self {
        Memento {
            data: vec![]
        }
    }
    pub fn store(&mut self, data: Originator) -> () {
        self.data.push(data);
    }
    pub fn get(&self, version: usize) -> &Originator {
        &self.data[version]
    }
}

/// # 管理者
/// 负责对备忘录进行管理，提供保存和获取备忘录功能
#[derive(Clone, Debug)]
pub struct CareTaker {
    memento: Memento,
}

impl CareTaker {
    pub fn new(memento: Memento) -> Self {
        CareTaker {
            memento
        }
    }
    pub fn store(&mut self, data: Originator) -> () {
        self.memento.store(data);
    }
    pub fn recover(&self, version: usize) -> &Originator {
        //get from Memento
        self.memento.get(version)
    }
}