# Rust TODO List Manager

一个功能完整的命令行 TODO 管理工具，使用 Rust 开发。

## 功能特性

- ✅ 添加 TODO 项目
- ✅ 修改 TODO 状态（HOLD、DOING、DONE）
- ✅ 查看 TODO 列表（支持状态过滤）
- ✅ 搜索 TODO 项目（支持正则表达式）
- ✅ 排序 TODO 列表（支持多字段排序）
- ✅ 删除单个 TODO 项目
- ✅ 清空 TODO 列表（支持状态过滤）
- ✅ 导出 TODO 数据到 JSON 文件
- ✅ 彩色状态显示
- ✅ 数据持久化存储
- ✅ 美观的表格输出

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
# 添加新的TODO项目
rustodo add "完成项目文档"

# 查看所有TODO项目
rustodo show

# 查看特定状态的TODO项目
rustodo show HOLD
rustodo show DOING
rustodo show DONE

# 修改TODO状态
rustodo done <todo_id>      # 标记为完成
rustodo undone <todo_id>    # 标记为待办
rustodo doing <todo_id>     # 标记为进行中

# 搜索TODO项目
rustodo search "关键词"           # 简单字符串搜索
rustodo search -R "正则表达式"    # 正则表达式搜索

# 排序TODO列表
rustodo sort <field>        # 按字段升序排序
rustodo sort -A <field>     # 按字段升序排序
rustodo sort -D <field>     # 按字段降序排序

# 删除TODO项目
rustodo delete <todo_id>

# 清空所有TODO项目
rustodo clear

# 清空特定状态的TODO项目
rustodo clear HOLD
rustodo clear DOING
rustodo clear DONE

# 导出TODO数据
rustodo export -P <path/filename.json>
```

### 状态说明

- **HOLD** (白色): 待办状态
- **DOING** (绿色): 进行中状态
- **DONE** (红色): 已完成状态

### 搜索功能

支持两种搜索模式：

- **简单搜索**: 不区分大小写的字符串匹配
- **正则搜索**: 使用正则表达式进行精确匹配

```bash
# 搜索包含"编程"的TODO项
rustodo search "编程"

# 使用正则表达式搜索以"编写"开头的TODO项
rustodo search -R "^编写"

# 搜索包含"代码"或"测试"的TODO项
rustodo search -R ".*[代码|测试].*"
```

### 排序功能

支持按以下字段排序：

- `id`: 按 ID 排序
- `content`: 按内容排序
- `status`: 按状态排序
- `created`: 按创建时间排序
- `updated`: 按更新时间排序

```bash
# 按创建时间降序排序
rustodo sort -D created

# 按内容升序排序
rustodo sort -A content

# 按状态排序（默认升序）
rustodo sort status
```

### 导出功能

可以将所有 TODO 数据导出为 JSON 格式文件：

```bash
# 导出到当前目录
rustodo export -P ./todos_backup.json

# 导出到指定路径
rustodo export -P /Users/username/Desktop/todos.json
```

### 输出格式

TODO 列表以表格形式展示，包含以下信息：

- **ID**: 唯一标识符（6 位随机字符串）
- **Created**: 创建时间
- **Updated**: 更新时间
- **Status**: 状态（带颜色显示）
- **Content**: 内容描述

### 数据存储

所有 TODO 数据自动保存到 `todos.json` 文件中，程序会自动加载和保存数据。

## 示例

```bash
# 添加几个TODO项目
rustodo add "学习Rust编程"
rustodo add "完成项目文档"
rustodo add "编写单元测试"
rustodo add "代码重构和优化"

# 查看所有项目
rustodo show

# 将第一个项目标记为进行中
rustodo doing <第一个项目的ID>

# 将第二个项目标记为完成
rustodo done <第二个项目的ID>

# 搜索包含"编程"的项目
rustodo search "编程"

# 按创建时间降序排序
rustodo sort -D created

# 查看进行中的项目
rustodo show DOING

# 查看已完成的项目
rustodo show DONE

# 导出所有数据
rustodo export -P ./backup.json
```

## 开发

### 依赖项

- `clap`: 命令行参数解析
- `serde`: 数据序列化
- `serde_json`: JSON 格式支持
- `chrono`: 时间处理
- `colored`: 终端颜色输出
- `uuid`: 唯一 ID 生成
- `anyhow`: 错误处理
- `regex`: 正则表达式支持
- `rand`: 随机数生成

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
