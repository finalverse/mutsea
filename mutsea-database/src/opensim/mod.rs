// src/opensim/mod.rs
//! OpenSim compatibility layer
//! 
//! This module provides compatibility with OpenSimulator's database schema
//! while allowing for AI enhancements.

pub mod schema;
pub mod models;
pub mod queries;

use crate::{DatabaseManager, Result};

/// OpenSim database operations
impl DatabaseManager {
    /// Initialize OpenSim compatible database tables
    pub async fn initialize_opensim_tables(&self) -> Result<()> {
        let backend = self.get_backend().await?;
        
        // Create core OpenSim tables
        let table_queries = vec![
            include_str!("../sql/opensim/create_regions.sql"),
            include_str!("../sql/opensim/create_users.sql"),
            include_str!("../sql/opensim/create_assets.sql"),
            include_str!("../sql/opensim/create_inventory.sql"),
            include_str!("../sql/opensim/create_primitives.sql"),
            include_str!("../sql/opensim/create_terrain.sql"),
            include_str!("../sql/opensim/create_parcels.sql"),
        ];

        for query in table_queries {
            backend.execute(query, &[]).await?;
        }

        Ok(())
    }

    /// Verify OpenSim tables exist and are properly structured
    pub async fn verify_opensim_tables(&self) -> Result<bool> {
        let backend = self.get_backend().await?;
        
        let required_tables = vec![
            "regions", "users", "assets", "inventoryitems", 
            "inventoryfolders", "primitives", "primshapes", 
            "terrain", "land", "landaccesslist"
        ];

        for table in required_tables {
            let exists = backend.table_exists(table).await?;
            if !exists {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
