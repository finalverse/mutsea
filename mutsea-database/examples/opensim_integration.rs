// /mutsea/mutsea-database/examples/opensim_integration.rs
//! OpenSim integration example showing basic operations

use mutsea_database::{MutseaDatabase, Result};
#[cfg(feature = "opensim-compat")]
use mutsea_database::opensim::schema::{Region, UserAccount, Asset};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/mutsea_dev".to_string());
    
    println!("🔗 Connecting to Mutsea database for OpenSim integration...");
    let db = MutseaDatabase::new_opensim_compatible(&database_url).await?;
    
    #[cfg(feature = "opensim-compat")]
    {
        // Ensure schema exists
        db.initialize_opensim_schema().await?;
        
        println!("🌍 Creating a test region...");
        let region_uuid = Uuid::new_v4().to_string();
        let mut region = Region::new(
            "Mutsea Test Region".to_string(),
            region_uuid.clone(),
            1000, // X coordinate
            1000, // Y coordinate
        );
        
        // Set additional region properties for OpenSim compatibility
        region.server_ip = "127.0.0.1".to_string();
        region.server_port = 9000;
        region.size_x = 256;
        region.size_y = 256;
        
        match db.manager().insert_region(&region).await {
            Ok(_) => println!("✅ Region '{}' created successfully!", region.region_name),
            Err(e) => println!("⚠️  Region creation: {}", e),
        }
        
        // Retrieve and verify the region
        match db.manager().get_region(&region_uuid).await? {
            Some(retrieved) => {
                println!("📍 Retrieved region: {}", retrieved.region_name);
                println!("   Location: ({}, {})", retrieved.loc_x, retrieved.loc_y);
                println!("   Size: {}x{}", retrieved.size_x, retrieved.size_y);
                println!("   Server: {}:{}", retrieved.server_ip, retrieved.server_port);
            },
            None => println!("❌ Could not retrieve created region"),
        }
        
        println!("\n👤 Creating a test user...");
        let user_uuid = Uuid::new_v4().to_string();
        let user = UserAccount::new(
            "Test".to_string(),
            "Resident".to_string(),
            user_uuid.clone(),
        );
        
        match db.manager().insert_user_account(&user).await {
            Ok(_) => println!("✅ User '{} {}' created successfully!", user.first_name, user.last_name),
            Err(e) => println!("⚠️  User creation: {}", e),
        }
        
        // Retrieve and verify the user
        match db.manager().get_user_account(&user_uuid).await? {
            Some(retrieved) => {
                println!("👤 Retrieved user: {} {}", retrieved.first_name, retrieved.last_name);
                println!("   ID: {}", retrieved.principal_id);
                println!("   Created: {}", retrieved.created);
                println!("   Active: {}", retrieved.active == 1);
            },
            None => println!("❌ Could not retrieve created user"),
        }
        
        println!("\n🖼️  Creating a test asset...");
        let asset_uuid = Uuid::new_v4().to_string();
        let asset_data = b"This is test texture data for Mutsea".to_vec();
        let asset = Asset::new(
            asset_uuid.clone(),
            "Mutsea Test Texture".to_string(),
            0, // Texture asset type
            asset_data,
        );
        
        match db.manager().insert_asset(&asset).await {
            Ok(_) => println!("✅ Asset '{}' created successfully!", asset.name),
            Err(e) => println!("⚠️  Asset creation: {}", e),
        }
        
        // Retrieve and verify the asset
        match db.manager().get_asset(&asset_uuid).await? {
            Some(retrieved) => {
                println!("🖼️  Retrieved asset: {}", retrieved.name);
                println!("   Type: {}", retrieved.asset_type);
                println!("   Size: {} bytes", retrieved.data.len());
                println!("   Creator: {}", retrieved.creator_id);
            },
            None => println!("❌ Could not retrieve created asset"),
        }
        
        println!("\n📊 Getting database health status...");
        match db.manager().get_opensim_health().await {
            Ok(health) => {
                println!("✅ Database Health Report:");
                println!("   All tables exist: {}", health.tables_exist);
                println!("   Regions in database: {}", health.region_count);
                println!("   Users in database: {}", health.user_count);
                println!("   Assets in database: {}", health.asset_count);
            },
            Err(e) => println!("⚠️  Could not get health status: {}", e),
        }
    }
    
    #[cfg(not(feature = "opensim-compat"))]
    {
        println!("⚠️  OpenSim compatibility feature not enabled!");
        println!("💡 Run with: cargo run --example opensim_integration --features opensim-compat");
    }
    
    println!("\n🎉 OpenSim integration example completed!");
    Ok(())
}