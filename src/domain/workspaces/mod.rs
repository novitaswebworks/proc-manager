#![allow(dead_code)]

pub mod models;

use models::{WorkspaceInfo, WorkspaceItem, WorkspaceItemType};
use crate::infrastructure::storage::database::Database;
use crate::errors::Result;
use sqlx::Row;

#[derive(Clone)]
pub struct WorkspaceManager {
    db: Database,
    workspaces: Vec<WorkspaceInfo>,
}

impl WorkspaceManager {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            workspaces: Vec::new(),
        }
    }

    pub async fn refresh(&mut self) -> Result<()> {
        let pool = &self.db.pool;
        
        let ws_rows = sqlx::query("SELECT id, name, description FROM workspaces")
            .fetch_all(pool)
            .await
            ?;

        let mut workspaces = Vec::new();

        for row in ws_rows {
            let id: i64 = row.get("id");
            let name: String = row.get("name");
            let description: Option<String> = row.try_get("description").unwrap_or_default();

            let item_rows = sqlx::query("SELECT id, item_type, item_name FROM workspace_items WHERE workspace_id = ?")
                .bind(id)
                .fetch_all(pool)
                .await
                ?;

            let mut items = Vec::new();
            for i_row in item_rows {
                let item_id: i64 = i_row.get("id");
                let item_type_str: String = i_row.get("item_type");
                let item_name: String = i_row.get("item_name");
                
                if let Some(item_type) = WorkspaceItemType::from_str(&item_type_str) {
                    items.push(WorkspaceItem {
                        id: item_id,
                        item_type,
                        item_name,
                    });
                }
            }

            workspaces.push(WorkspaceInfo {
                id,
                name,
                description,
                items,
            });
        }

        self.workspaces = workspaces;
        Ok(())
    }

    pub fn get_workspaces(&self) -> Vec<WorkspaceInfo> {
        self.workspaces.clone()
    }

    pub async fn create_workspace(&mut self, name: &str, description: Option<&str>) -> Result<()> {
        sqlx::query("INSERT INTO workspaces (name, description) VALUES (?, ?)")
            .bind(name)
            .bind(description)
            .execute(&self.db.pool)
            .await
            ?;
        
        self.refresh().await?;
        Ok(())
    }

    pub async fn delete_workspace(&mut self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM workspaces WHERE id = ?")
            .bind(id)
            .execute(&self.db.pool)
            .await
            ?;
        
        self.refresh().await?;
        Ok(())
    }

    pub async fn add_item(&mut self, workspace_id: i64, item_type: WorkspaceItemType, item_name: &str) -> Result<()> {
        // Prevent duplicate items
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM workspace_items WHERE workspace_id = ? AND item_type = ? AND item_name = ?")
            .bind(workspace_id)
            .bind(item_type.as_str())
            .bind(item_name)
            .fetch_one(&self.db.pool)
            .await
            ?;

        if count.0 > 0 {
            return Ok(());
        }

        sqlx::query("INSERT INTO workspace_items (workspace_id, item_type, item_name) VALUES (?, ?, ?)")
            .bind(workspace_id)
            .bind(item_type.as_str())
            .bind(item_name)
            .execute(&self.db.pool)
            .await
            ?;

        self.refresh().await?;
        Ok(())
    }

    pub async fn remove_item(&mut self, item_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM workspace_items WHERE id = ?")
            .bind(item_id)
            .execute(&self.db.pool)
            .await
            ?;
        
        self.refresh().await?;
        Ok(())
    }
}
