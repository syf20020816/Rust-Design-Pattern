# README

## 总体来说设计模式分为三大类：

- 创建模式：用来构建对象以便能从实现系统解耦。
- 结构模式：用不同的对象组成大规模的对象结构。
- 行为模式：用来在对象中管理算法，关系，和责任。

### 创建型模式

- 单例模式（Singleton Pattern）
- 工厂模式（Factory Pattern）
- 抽象工厂模式（Abstract Factory Pattern）
- 建造者模式（Builder Pattern）
- 原型模式（Prototype Pattern）

### 结构型模式

- 适配器模式（Adapter Pattern）
- 桥接模式（Bridge Pattern）
- 过滤器模式（Filter、Criteria Pattern）
- 组合模式（Composite Pattern）
- 装饰器模式（Decorator Pattern）
- 外观模式（Facade Pattern）
- 享元模式（Flyweight Pattern）
- 代理模式（Proxy Pattern）

### 行为型模式

- 责任链模式（Chain of Responsibility Pattern）
- 命令模式（Command Pattern）
- 解释器模式（Interpreter Pattern）
- 迭代器模式（Iterator Pattern）
- 中介者模式（Mediator Pattern）
- 备忘录模式（Memento Pattern）
- 观察者模式（Observer Pattern）
- 状态模式（State Pattern）
- 空对象模式（Null Object Pattern）
- 策略模式（Strategy Pattern）
- 模板模式（Template Pattern）
- 访问者模式（Visitor Pattern）

### 另外的两种模式：

- 并发模式
    - future
- 线程池模式

## 设计模式六大原则

总原则：开闭原则（Open Close Principle）

开闭原则就是说对扩展开放，对修改关闭。在程序需要进行拓展的时候，不能去修改原有的代码，而是要扩展原有代码，实现一个热插拔的效果。想要达到这样的效果，我们需要使用接口和抽象类等。

1. 单一职责原则 （SRP：Single responsibility principle）

不要存在多于一个导致类变更的原因，也就是说每个类应该实现单一的职责，如若不然，就应该把类拆分。

2. 里氏替换原则（Liskov Substitution Principle）

里氏代换原则中说，任何基类可以出现的地方，子类一定可以出现。 LSP是继承复用的基石，只有当衍生类可以替换掉基类，软件单位的功能不受到影响时，基类才能真正被复用，而衍生类也能够在基类的基础上增加新的行为。里氏代换原则是对“开-闭”原则的补充。实现“开-闭”原则的关键步骤就是抽象化。而基类与子类的继承关系就是抽象化的具体实现，所以里氏代换原则是对实现抽象化的具体步骤的规范。

里氏替换原则中，子类对父类的方法尽量不要重写和重载。因为父类代表了定义好的结构，通过这个规范的接口与外界交互，子类不应该随便破坏它。

3. 依赖倒转原则（Dependence Inversion Principle）

这个是开闭原则的基础，具体内容：面向接口编程，依赖于抽象而不依赖于具体。写代码时用到具体类时，不与具体类交互，而与具体类的上层接口交互。

4. 接口隔离原则（Interface Segregation Principle）

这个原则的意思是：每个接口中不存在子类用不到却必须实现的方法，如果不然，就要将接口拆分。使用多个隔离的接口，比使用单个接口（多个接口方法集合到一个的接口）要好。

5. 迪米特法则（最少知道原则）（Demeter Principle）
   就是说：一个类对自己依赖的类知道的越少越好。也就是说无论被依赖的类多么复杂，都应该将逻辑封装在方法的内部，通过public方法提供给外部。这样当被依赖的类变化时，才能最小的影响该类。

最少知道原则的另一个表达方式是：只与直接的朋友通信。类之间只要有耦合关系，就叫朋友关系。耦合分为依赖、关联、聚合、组合等。我们称出现为成员变量、方法参数、方法返回值中的类为直接朋友。局部变量、临时变量则不是直接的朋友。我们要求陌生的类不要作为局部变量出现在类中。

6. 合成复用原则（Composite Reuse Principle）
   原则是尽量首先使用合成/聚合的方式，而不是使用继承。

## 总结

1. 开闭原则：扩展易维护，接口为规范，实现为使用，实现热插拔
2. 里氏代换：不要质疑前人的智慧，我们要站在巨人的肩膀上（子类不重写，只扩展功能）
3. 依赖倒转：对抽象进行编程，使用抽象，隐藏具体实现，降低耦合
4. 接口隔离：用多少，拿多少，对接口方法实现必要的，不出现任何冗余
5. 迪米特：中介助力降低风险提升效率
6. 合成复用：高内聚低耦合（多聚合，少继承），易扩展易复用