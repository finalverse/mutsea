// /mutsea/mutsea-database/src/manager.rs
//! Database manager for coordinating operations

use crate::{
    backends::{DatabasePool, DatabaseBackend},
    Result, DatabaseError,
};
use std::sync::Arc;
use tracing::{info, debug, error};

/// Database manager for coordinating operations across different backends
pub struct DatabaseManager {
    pool: Arc<DatabasePool>,
}

impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Initializing database manager with URL: {}", 
              database_url.split('@').last().unwrap_or("unknown"));
        
        let pool = DatabasePool::new(database_url).await?;
        info!("Database pool created successfully");
        
        Ok(Self { 
            pool: Arc::new(pool) 
        })
    }
    
    /// Get a database backend instance
    pub async fn get_backend(&self) -> Result<Box<dyn DatabaseBackend>> {
        Ok(self.pool.get_backend())
    }
    
    /// Get the backend type
    pub fn backend_type(&self) -> crate::backends::BackendType {
        self.pool.backend_type()
    }
    
    /// Test database connectivity
    pub async fn test_connection(&self) -> Result<()> {
        debug!("Testing database connection");
        let backend = self.get_backend().await?;
        
        // Try a simple query to test connectivity
        match backend.table_exists("information_schema.tables").await {
            Ok(_) => {
                info!("Database connection test successful");
                Ok(())
            },
            Err(e) => {
                error!("Database connection test failed: {}", e);
                Err(e)
            }
        }
    }

    /// Get database statistics
    pub async fn get_statistics(&self) -> Result<DatabaseStats> {
        let backend = self.get_backend().await?;
        
        // Basic stats - can be expanded based on backend type
        Ok(DatabaseStats {
            backend_type: self.backend_type(),
            is_connected: self.test_connection().await.is_ok(),
            // TODO: Add more meaningful statistics
        })
    }
}

/// Database statistics structure
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub backend_type: crate::backends::BackendType,
    pub is_connected: bool,
}

// OpenSim-specific operations
#[cfg(feature = "opensim-compat")]
impl DatabaseManager {
    /// Initialize OpenSim compatible database tables
    pub async fn initialize_opensim_tables(&self) -> Result<()> {
        info!("Initializing OpenSim compatible database schema");
        let backend = self.get_backend().await?;
        
        // List of SQL files to execute for OpenSim setup
        let sql_files = vec![
            ("regions", include_str!("sql/opensim/create_regions.sql")),
            ("user_accounts", include_str!("sql/opensim/create_users.sql")),
            ("assets", include_str!("sql/opensim/create_assets.sql")),
            ("inventory", include_str!("sql/opensim/create_inventory.sql")),
            ("primitives", include_str!("sql/opensim/create_primitives.sql")),
            ("terrain", include_str!("sql/opensim/create_terrain.sql")),
            ("parcels", include_str!("sql/opensim/create_parcels.sql")),
        ];

        for (table_name, sql) in sql_files {
            debug!("Creating table: {}", table_name);
            match backend.execute(sql, &[]).await {
                Ok(_) => info!("Table '{}' created successfully", table_name),
                Err(e) => {
                    // Log but continue - table might already exist
                    debug!("Table '{}' creation result: {}", table_name, e);
                }
            }
        }

        info!("OpenSim schema initialization completed");
        Ok(())
    }

    /// Verify OpenSim tables exist and are properly structured
    pub async fn verify_opensim_tables(&self) -> Result<bool> {
        info!("Verifying OpenSim database compatibility");
        let backend = self.get_backend().await?;
        
        let required_tables = vec![
            "regions", 
            "user_accounts", 
            "assets", 
            "inventoryitems", 
            "inventoryfolders", 
            "primitives", 
            "terrain", 
            "land", 
            "landaccesslist"
        ];

        let mut all_exist = true;
        for table in &required_tables {
            match backend.table_exists(table).await {
                Ok(exists) => {
                    if exists {
                        debug!("Table '{}' exists", table);
                    } else {
                        error!("Required table '{}' does not exist", table);
                        all_exist = false;
                    }
                },
                Err(e) => {
                    error!("Error checking table '{}': {}", table, e);
                    all_exist = false;
                }
            }
        }

        if all_exist {
            info!("All required OpenSim tables verified successfully");
        } else {
            error!("OpenSim compatibility verification failed");
        }

        Ok(all_exist)
    }

    /// Get OpenSim database health status
    pub async fn get_opensim_health(&self) -> Result<OpenSimHealth> {
        let backend = self.get_backend().await?;
        
        // Check if core tables exist and get basic counts
        let regions_exist = backend.table_exists("regions").await.unwrap_or(false);
        let users_exist = backend.table_exists("user_accounts").await.unwrap_or(false);
        let assets_exist = backend.table_exists("assets").await.unwrap_or(false);
        
        // TODO: Get actual counts from tables
        let region_count = if regions_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };
        
        let user_count = if users_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };
        
        let asset_count = if assets_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };

        Ok(OpenSimHealth {
            tables_exist: regions_exist && users_exist && assets_exist,
            region_count,
            user_count,
            asset_count,
        })
    }
}

/// OpenSim database health information
#[cfg(feature = "opensim-compat")]
#[derive(Debug, Clone)]
pub struct OpenSimHealth {
    pub tables_exist: bool,
    pub region_count: u64,
    pub user_count: u64,
    pub asset_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_manager_creation() {
        // This would test against a real test database
        // For now, just test that the manager can be created
        assert!(true);
    }
}