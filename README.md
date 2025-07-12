# Rust TODO List 命令行工具

一个功能完整的命令行 TODO 管理工具，使用 Rust 开发。

## 功能特性

-   ✅ 添加新的 TODO 项
-   🔄 管理 TODO 状态（HOLD、DOING、DONE）
-   📋 显示 TODO 列表（支持状态过滤）
-   🗑️ 删除指定的 TODO 项
-   🧹 清空 TODO 列表（支持状态过滤）
-   💾 数据持久化存储（JSON 格式）
-   🎨 彩色状态显示
-   🆔 短 ID 设计，便于记忆和操作
-   ⏰ 显示创建时间和更新时间

## 安装

确保你的系统已安装 Rust，然后在项目目录下运行：

```bash
cargo build --release
```

编译完成后，你可以将可执行文件添加到系统 PATH 中，或者直接使用：

```bash
./target/release/rust_todo_list
```

## 使用方法

### 基本命令

```bash
# 添加新的TODO项
rustodo add "完成项目文档"

# 显示所有TODO项
rustodo show

# 显示特定状态的TODO项
rustodo show HOLD
rustodo show DOING
rustodo show DONE

# 更新TODO状态
rustodo done <todo_id>      # 标记为完成
rustodo doing <todo_id>     # 标记为进行中
rustodo undone <todo_id>    # 标记为待处理

# 删除指定的TODO项
rustodo delete <todo_id>

# 清空所有TODO项
rustodo clear

# 清空特定状态的TODO项
rustodo clear HOLD
rustodo clear DOING
rustodo clear DONE
```

### 状态说明

-   **HOLD** (白色): 待处理状态
-   **DOING** (绿色): 进行中状态
-   **DONE** (红色): 已完成状态

### 示例

```bash
# 添加几个TODO项
rustodo add "学习Rust编程"
rustodo add "完成项目文档"
rustodo add "代码审查"

# 查看所有TODO项
rustodo show

# 将第一个TODO标记为进行中（使用短ID）
rustodo doing YJU0KT

# 查看进行中的TODO项
rustodo show DOING

# 将TODO标记为完成
rustodo done ACGITS

# 查看已完成的TODO项
rustodo show DONE
```

## 数据存储

TODO 数据会自动保存到当前目录下的 `todos.json` 文件中，格式如下：

```json
[
    {
        "id": "YJU0KT",
        "content": "TODO内容",
        "status": "HOLD",
        "created_at": "2024-01-01T12:00:00Z",
        "updated_at": "2024-01-01T12:00:00Z"
    }
]
```

## 显示格式

TODO 列表以表格形式显示，包含以下信息：

-   **ID**: 6 位短 ID（字母数字组合），便于记忆和操作
-   **创建时间**: 格式为 MM-DD HH:MM
-   **更新时间**: 格式为 MM-DD HH:MM，显示最后修改时间
-   **状态**: 彩色显示（HOLD 白色、DOING 绿色、DONE 红色）
-   **内容**: TODO 项的主要内容（超过 38 字符会截断显示）

### 显示示例

```
📋 TODO列表:
ID       创建时间                 更新时间                 状态         内容
----------------------------------------------------------------------------------------------------
YJU0KT   07-12 09:52          07-12 09:52          DOING 学习Rust编程
ACGITS   07-12 09:52          07-12 09:52          HOLD 完成项目文档
----------------------------------------------------------------------------------------------------
总计: 2 项
```

## 错误处理

工具包含完善的错误处理机制：

-   无效的 TODO ID 会显示错误信息
-   无效的状态过滤会提示有效选项
-   文件读写错误会给出相应提示

## 开发

### 依赖项

-   `clap`: 命令行参数解析
-   `serde`: 序列化/反序列化
-   `serde_json`: JSON 处理
-   `chrono`: 时间处理
-   `colored`: 终端颜色输出
-   `rand`: 随机数生成（用于短 ID）
-   `anyhow`: 错误处理

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test
```

## 许可证

MIT License
