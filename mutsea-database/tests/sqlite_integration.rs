use mutsea_database::{DatabaseManager};
use mutsea_database::opensim::models::{UserAccount, Asset};

#[tokio::test]
async fn sqlite_migration_and_queries() -> anyhow::Result<()> {
    let manager = DatabaseManager::new("sqlite://:memory:").await?;
    manager.migrate().await?;
    manager.initialize_ai_schema().await?;
    manager.initialize_opensim_tables().await?;

    let backend = manager.get_backend().await?;
    for table in [
        "ai_decisions",
        "ai_global_mind_state",
        "emergent_behaviors",
        "learning_data",
        "npc_states",
        "user_accounts",
        "assets",
    ] {
        assert!(backend.table_exists(table).await?);
    }

    let user = UserAccount::new("Test".into(), "User".into(), "user1".into());
    manager.insert_user_account(&user).await?;
    let fetched = manager.get_user_account(&user.principal_id).await?;
    assert_eq!(fetched.unwrap().first_name, "Test");

    let asset = Asset::new("asset1".into(), "Test Asset".into(), 0, vec![1,2,3]);
    manager.insert_asset(&asset).await?;
    let fetched_asset = manager.get_asset(&asset.id).await?;
    assert_eq!(fetched_asset.unwrap().name, "Test Asset");

    Ok(())
}
