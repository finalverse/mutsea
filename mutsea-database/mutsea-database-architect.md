# Mutsea Database Architecture Plan

## Possible Issues
- Single 2000+ line file with embedded SQL
- Mixed responsibilities (query building, analytics, batch operations)
- Hard to maintain and test SQL statements
- Poor separation of concerns

## Proposed Architecture

```
mutsea-database/
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── world_state.rs
│   │   ├── player_behavior.rs
│   │   ├── ai_decision.rs
│   │   ├── emergent_behavior.rs
│   │   ├── npc_state.rs
│   │   ├── ecosystem_state.rs
│   │   ├── learning_data.rs
│   │   └── performance_metrics.rs
│   ├── queries/
│   │   ├── mod.rs
│   │   ├── builder.rs          # Query builder traits and core logic
│   │   ├── world_queries.rs    # World state queries
│   │   ├── player_queries.rs   # Player behavior queries  
│   │   ├── ai_queries.rs       # AI decision queries
│   │   ├── ecosystem_queries.rs # Ecosystem queries
│   │   ├── performance_queries.rs # Performance queries
│   │   └── batch_operations.rs # Batch insert/update operations
│   ├── analytics/
│   │   ├── mod.rs
│   │   ├── player_analytics.rs # Player behavior patterns
│   │   ├── ai_analytics.rs     # AI decision effectiveness
│   │   ├── ecosystem_analytics.rs # Ecosystem health metrics
│   │   ├── performance_analytics.rs # Performance optimizations
│   │   └── cache.rs           # Analytics caching
│   ├── sql/                   # External SQL files
│   │   ├── postgresql/
│   │   │   ├── world_state/
│   │   │   │   ├── select_world_states.sql
│   │   │   │   ├── insert_world_state.sql
│   │   │   │   └── batch_insert_world_states.sql
│   │   │   ├── player_behavior/
│   │   │   │   ├── select_player_behaviors.sql
│   │   │   │   ├── player_behavior_patterns.sql
│   │   │   │   └── batch_insert_behaviors.sql
│   │   │   ├── ai_decisions/
│   │   │   │   ├── select_ai_decisions.sql
│   │   │   │   ├── ai_effectiveness_metrics.sql
│   │   │   │   └── batch_update_feedback.sql
│   │   │   ├── emergent_behavior/
│   │   │   │   ├── select_emergent_behaviors.sql
│   │   │   │   ├── emergent_analysis.sql
│   │   │   │   └── batch_insert_emergent.sql
│   │   │   ├── ecosystem/
│   │   │   │   ├── select_ecosystem_states.sql
│   │   │   │   ├── ecosystem_health_metrics.sql
│   │   │   │   └── batch_insert_ecosystem.sql
│   │   │   ├── performance/
│   │   │   │   ├── select_performance_metrics.sql
│   │   │   │   ├── performance_optimization.sql
│   │   │   │   └── batch_insert_metrics.sql
│   │   │   └── analytics/
│   │   │       ├── player_patterns_analysis.sql
│   │   │       ├── ai_decision_effectiveness.sql
│   │   │       ├── emergent_behavior_analysis.sql
│   │   │       ├── ecosystem_health_analysis.sql
│   │   │       ├── performance_optimization_analysis.sql
│   │   │       ├── npc_learning_analysis.sql
│   │   │       └── world_generation_metrics.sql
│   │   └── sqlite/
│   │       └── [same structure as postgresql]
│   ├── traits/
│   │   ├── mod.rs
│   │   ├── database_backend.rs
│   │   ├── query_builder.rs
│   │   └── analytics_provider.rs
│   └── utils/
│       ├── mod.rs
│       ├── sql_loader.rs       # Load SQL from files
│       ├── parameter_binding.rs # Parameter binding utilities
│       └── result_parsing.rs   # Result parsing utilities
└── migrations/
    ├── postgresql/
    └── sqlite/
```

## Key Improvements

### 1. **Separation of Concerns**
- **Models**: Data structures and domain logic
- **Queries**: Basic CRUD operations per entity
- **Analytics**: Complex analytical queries
- **Batch Operations**: Bulk data operations
- **SQL Files**: External, versioned SQL statements

### 2. **SQL Externalization**
- SQL statements moved to `.sql` files
- Organized by database dialect and functionality
- Version controlled and easily maintainable
- Better syntax highlighting and validation

### 3. **Modular Architecture**
- Single responsibility per module
- Clear dependencies between modules
- Easier testing and mocking
- Better code reusability

### 4. **Enhanced Maintainability**
- SQL changes don't require Rust recompilation
- Database-specific optimizations
- Clear separation of business logic and data access
- Improved debugging and profiling

## Implementation Strategy

### Phase 1: Core Infrastructure
1. Create the new directory structure
2. Implement SQL loader utility
3. Define core traits and interfaces
4. Create base query builder

### Phase 2: Entity Queries
1. Extract and externalize basic CRUD queries
2. Implement entity-specific query modules
3. Add parameter binding and result parsing utilities
4. Migrate batch operations

### Phase 3: Analytics Refactoring
1. Extract analytics queries to separate modules
2. Implement caching layer
3. Add performance monitoring
4. Create analytics-specific result types

### Phase 4: Testing & Optimization
1. Comprehensive unit tests for each module
2. Integration tests for query combinations
3. Performance benchmarking
4. Documentation and examples

## Benefits

### **Developer Experience**
- Easier to navigate and understand code
- SQL syntax highlighting in IDEs
- Clearer separation of database and business logic
- Faster development and debugging

### **Maintainability**
- SQL changes without Rust recompilation
- Version controlled SQL schemas
- Easier database migrations
- Better code organization

### **Performance**
- SQL query optimization without code changes
- Better caching strategies
- Reduced compilation times
- Improved query planning

### **Testing**
- Unit tests for individual components
- Mock implementations for testing
- SQL-specific testing tools
- Better error isolation

### **Scalability**
- Easy to add new query types
- Database-specific optimizations
- Modular feature development
- Clear upgrade paths

This refactoring transforms the monolithic queries.rs into a well-organized, maintainable database layer that aligns with the AI-driven architecture goals of the Mutsea engine.