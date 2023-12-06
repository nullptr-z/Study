[serde(rename_all = "snake_case")] 蛇形命名所有字段（域）

#[serde(skip_serializing_if = "Option::is_none")] 满足条件不进行序列化

#[serde(rename_all = "snake_case")] Struct、Enum 中的全部字段`snake_case`

#[serde(rename = "text-embedding-ada-002")] 字段序列化名称

#[serde(default)] 表示在反序列化时，如果输入数据中缺少某个字段，就使用该字段的默认值。

#[serde(untagged)] 表示 Serde 库将尝试匹配 JSON 数据和 Rust 结构体或枚举，而不考虑 JSON 对象中的字段名称(key)。这对于处理不同形式的 JSON 数据非常有用
