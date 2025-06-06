## Reorganized NPC State Architecture

### 1. **Core Module (`npc_state/mod.rs`)**
- Main `NPCState` struct with all components
- Basic enums and types (NPCType, NPCAge, etc.)
- Core implementation methods
- DatabaseModel and AIModel trait implementations

### 2. **Physical Module (`npc_state/physical.rs`)**
- `MovementState`, `PhysicalCondition`, `NPCAppearance`
- Skills and abilities (`SkillSet`, `NPCAbility`)
- Learning progress tracking
- Environmental interactions and territory
- Physical appearance details

### 3. **Cognitive Module (`npc_state/cognitive.rs`)**
- `IntelligenceProfile` with specialized knowledge areas
- Complete memory system (short-term, long-term, working, episodic, semantic, procedural)
- `AttentionFocus` with filters and multitasking
- `PersonalityProfile` with Big Five traits and moral alignment
- `PsychologicalTraits` with mental health and coping mechanisms

### 4. **Social Module (`npc_state/social.rs`)**
- `SocialStatus` with roles, influence, and status symbols
- `NPCRelationship` with emotional bonds and power dynamics
- `GroupMembership` with various group types
- `ReputationProfile` with aspects and events tracking
- `CommunicationState` with language proficiency and styles

### 5. **Behavioral Module (`npc_state/behavioral.rs`)**
- `CurrentBehavior` with triggers and modifiers
- `BehaviorPattern` with conditions and variations
- `NPCGoal` with progress tracking and success criteria
- `Motivation` with satisfaction levels and decay
- `DecisionMakingState` with different decision styles

### 6. **Economic Module (`npc_state/economic.rs`)**
- `NPCInventory` with weight management and organization
- `EconomicStatus` with assets, debts, and financial health
- `TradePreferences` with compatibility calculations
- `ResourceNeed` tracking

## Key Improvements Made:

1. **Eliminated Duplication**: Removed all duplicate code and structures
2. **Logical Organization**: Split into focused modules by domain
3. **Clean Dependencies**: Each module imports only what it needs
4. **Rich Functionality**: Added utility methods for calculations and interactions
5. **Comprehensive Defaults**: Sensible default implementations for all types
6. **Type Safety**: Proper enum usage and validation
7. **AI Integration**: Maintained AI-first architecture throughout

Each module is now focused, maintainable, and provides rich functionality for AI-driven NPCs. The architecture supports complex emergent behaviors while keeping the code organized and extensible.