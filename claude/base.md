## 基本概念

Skill ≈ Prompt 模板/常量 ≈ 可以接受参数的函数
MCP = 工具接口，对接外部系统
Agent = 决策思考工作处理流程

## Skill

Skill 只是让你少打字，真正干活的是 Agent SDK + MCP。
简单 Skill 是常量，复杂 Skill 甚至就是 Agent
Skill 的能力上限 ≈ 你能用自然语言描述多复杂的逻辑

## Agent

Agent 是思考和决策的部分，接受到一个任务时，如何完成制定流程

简单的 Agent 可以是一段自然语言描述带有任务条理的 Prompt 模板

```
1. 首先
2. 然后
3. 最后
```

复杂的 Agent 则是一个程序，按照一定的我们定义的逻辑来完成任务

```python
class MyAgent:
    def __init__(self):
        self.tools = [...]
        self.memory = []

    def think(self, task):
        # 规划步骤

    def act(self, action):
        # 执行工具

    def observe(self, result):
        # 分析结果，决定下一步

    def run(self, task):
        while not self.is_done():
            thought = self.think(task)
            action = self.decide(thought)
            result = self.act(action)
            self.observe(result)
```

## MCP

MCP 是工具接口，负责和外部系统交互，对接外部系统或工具
