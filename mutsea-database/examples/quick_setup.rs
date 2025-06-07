
// /mutsea/mutsea-database/examples/quick_setup.rs
//! Quick setup example for Mutsea database with OpenSim compatibility

use mutsea_database::{MutseaDatabase, Result};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Mutsea Database Quick Setup");
    println!("================================");
    
    // Database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            println!("⚠️  DATABASE_URL not set, using default PostgreSQL connection");
            "postgresql://postgres:password@localhost:5432/mutsea_dev".to_string()
        });
    
    println!("📡 Connecting to database...");
    
    // Create database instance with OpenSim compatibility
    match MutseaDatabase::new_opensim_compatible(&database_url).await {
        Ok(db) => {
            println!("✅ Database connection established!");
            
            // Test connection
            println!("🔍 Testing connection...");
            match db.manager().test_connection().await {
                Ok(_) => println!("✅ Connection test passed!"),
                Err(e) => {
                    println!("❌ Connection test failed: {}", e);
                    return Err(e);
                }
            }
            
            // Initialize OpenSim schema
            #[cfg(feature = "opensim-compat")]
            {
                println!("🏗️  Initializing OpenSim schema...");
                match db.initialize_opensim_schema().await {
                    Ok(_) => println!("✅ OpenSim schema initialized!"),
                    Err(e) => {
                        println!("⚠️  Schema initialization had issues: {}", e);
                        // Continue anyway - tables might already exist
                    }
                }
                
                // Verify compatibility
                println!("🔍 Verifying OpenSim compatibility...");
                match db.verify_opensim_compatibility().await {
                    Ok(true) => println!("✅ OpenSim compatibility verified!"),
                    Ok(false) => println!("❌ OpenSim compatibility check failed!"),
                    Err(e) => println!("⚠️  Compatibility check error: {}", e),
                }
                
                // Get health status
                match db.manager().get_opensim_health().await {
                    Ok(health) => {
                        println!("📊 OpenSim Database Health:");
                        println!("   Tables exist: {}", health.tables_exist);
                        println!("   Regions: {}", health.region_count);
                        println!("   Users: {}", health.user_count);
                        println!("   Assets: {}", health.asset_count);
                    },
                    Err(e) => println!("⚠️  Could not get health status: {}", e),
                }
            }
            
            println!("\n🎉 Mutsea database setup complete!");
            println!("💡 You can now start using the database with OpenSim compatibility.");
            
        },
        Err(e) => {
            println!("❌ Failed to connect to database: {}", e);
            println!("\n💡 Make sure:");
            println!("   1. PostgreSQL is running");
            println!("   2. Database exists: createdb mutsea_dev");
            println!("   3. DATABASE_URL is correct");
            return Err(e);
        }
    }
    
    Ok(())
}