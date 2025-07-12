use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use rand::Rng;

#[derive(Parser)]
#[command(name = "rustodo")]
#[command(about = "一个简单的命令行TODO工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新的TODO项
    Add {
        /// TODO内容
        content: String,
    },
    /// 将TODO标记为完成
    Done {
        /// TODO的ID
        todo_id: String,
    },
    /// 将TODO标记为进行中
    Doing {
        /// TODO的ID
        todo_id: String,
    },
    /// 将TODO标记为待处理
    Undone {
        /// TODO的ID
        todo_id: String,
    },
    /// 显示TODO列表
    Show {
        /// 可选的TODO状态过滤
        status: Option<String>,
    },
    /// 清空TODO列表
    Clear {
        /// 可选的TODO状态过滤
        status: Option<String>,
    },
    /// 删除指定的TODO项
    Delete {
        /// TODO的ID
        todo_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum TodoStatus {
    HOLD,
    DOING,
    DONE,
}

impl TodoStatus {
    fn to_colored_string(&self) -> String {
        match self {
            TodoStatus::HOLD => "HOLD".white().to_string(),
            TodoStatus::DOING => "DOING".green().to_string(),
            TodoStatus::DONE => "DONE".red().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: String,
    content: String,
    status: TodoStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TodoItem {
    fn new(content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Self::generate_short_id(),
            content,
            status: TodoStatus::HOLD,
            created_at: now,
            updated_at: now,
        }
    }

    fn generate_short_id() -> String {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
        (0..6).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
    }

    fn update_status(&mut self, status: TodoStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

struct TodoManager {
    todos: HashMap<String, TodoItem>,
    file_path: String,
}

impl TodoManager {
    fn new() -> Self {
        let file_path = "todos.json".to_string();
        Self {
            todos: HashMap::new(),
            file_path,
        }
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let todos: Vec<TodoItem> = serde_json::from_str(&content)?;
            self.todos = todos.into_iter().map(|todo| (todo.id.clone(), todo)).collect();
        }
        Ok(())
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let todos: Vec<&TodoItem> = self.todos.values().collect();
        let content = serde_json::to_string_pretty(&todos)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    fn add_todo(&mut self, content: String) -> Result<(), Box<dyn std::error::Error>> {
        let todo = TodoItem::new(content);
        let id = todo.id.clone();
        self.todos.insert(id, todo);
        self.save()?;
        self.display_todos(None)?;
        Ok(())
    }

    fn update_todo_status(&mut self, todo_id: &str, status: TodoStatus) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(todo) = self.todos.get_mut(todo_id) {
            todo.update_status(status);
            self.save()?;
            println!("✅ TODO项状态已更新: {}", todo_id);
        } else {
            println!("❌ 未找到ID为 {} 的TODO项", todo_id);
        }
        Ok(())
    }

    fn delete_todo(&mut self, todo_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(todo) = self.todos.remove(todo_id) {
            self.save()?;
            println!("🗑️ 已删除TODO项: {} - {}", todo_id, todo.content);
        } else {
            println!("❌ 未找到ID为 {} 的TODO项", todo_id);
        }
        Ok(())
    }

    fn clear_todos(&mut self, status_filter: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mut to_remove = Vec::new();
        
        for (id, todo) in &self.todos {
            if let Some(filter) = status_filter {
                let status_str = match &todo.status {
                    TodoStatus::HOLD => "HOLD",
                    TodoStatus::DOING => "DOING",
                    TodoStatus::DONE => "DONE",
                };
                if status_str == filter.to_uppercase() {
                    to_remove.push(id.clone());
                }
            } else {
                to_remove.push(id.clone());
            }
        }

        for id in to_remove {
            self.todos.remove(&id);
        }

        self.save()?;
        
        if let Some(filter) = status_filter {
            println!("🧹 已清空状态为 {} 的所有TODO项", filter);
        } else {
            println!("🧹 已清空所有TODO项");
        }
        Ok(())
    }

    fn display_todos(&self, status_filter: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        if self.todos.is_empty() {
            println!("📝 暂无TODO项");
            return Ok(());
        }

        let mut filtered_todos: Vec<&TodoItem> = self.todos.values().collect();
        
        if let Some(filter) = status_filter {
            let status_filter = match filter.to_uppercase().as_str() {
                "HOLD" => TodoStatus::HOLD,
                "DOING" => TodoStatus::DOING,
                "DONE" => TodoStatus::DONE,
                _ => {
                    println!("❌ 无效的状态过滤: {}. 有效状态: HOLD, DOING, DONE", filter);
                    return Ok(());
                }
            };
            
            filtered_todos.retain(|todo| {
                matches!(&todo.status, status if std::mem::discriminant(status) == std::mem::discriminant(&status_filter))
            });
        }

        if filtered_todos.is_empty() {
            if let Some(filter) = status_filter {
                println!("📝 没有状态为 {} 的TODO项", filter);
            } else {
                println!("📝 暂无TODO项");
            }
            return Ok(());
        }

        // 按创建时间排序
        filtered_todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        println!("\n📋 TODO列表:");
        println!("{:<8} {:<20} {:<20} {:<10} {:<40}", "ID", "创建时间", "更新时间", "状态", "内容");
        println!("{}", "-".repeat(100));

        for todo in &filtered_todos {
            let created_time = todo.created_at.format("%m-%d %H:%M").to_string();
            let updated_time = todo.updated_at.format("%m-%d %H:%M").to_string();
            let status = todo.status.to_colored_string();
            println!(
                "{:<8} {:<20} {:<20} {:<10} {:<40}",
                todo.id,
                created_time,
                updated_time,
                status,
                if todo.content.len() > 38 {
                    format!("{}...", &todo.content[..35])
                } else {
                    todo.content.clone()
                }
            );
        }
        println!("{}", "-".repeat(100));
        println!("总计: {} 项", filtered_todos.len());
        println!();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut manager = TodoManager::new();
    manager.load()?;

    match &cli.command {
        Commands::Add { content } => {
            manager.add_todo(content.clone())?;
        }
        Commands::Done { todo_id } => {
            manager.update_todo_status(todo_id, TodoStatus::DONE)?;
        }
        Commands::Doing { todo_id } => {
            manager.update_todo_status(todo_id, TodoStatus::DOING)?;
        }
        Commands::Undone { todo_id } => {
            manager.update_todo_status(todo_id, TodoStatus::HOLD)?;
        }
        Commands::Show { status } => {
            manager.display_todos(status.as_deref())?;
        }
        Commands::Clear { status } => {
            manager.clear_todos(status.as_deref())?;
        }
        Commands::Delete { todo_id } => {
            manager.delete_todo(todo_id)?;
        }
    }

    Ok(())
}
