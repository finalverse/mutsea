// mutsea-database/src/models/npc_state/social.rs
//! Social state models for NPCs including relationships, communication, and reputation

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Social status and relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialStatus {
    pub hierarchy_level: f32, // 0.0 (bottom) to 1.0 (top)
    pub social_roles: Vec<SocialRole>,
    pub influence_level: f32,
    pub social_capital: f32,
    pub network_centrality: f32,
    pub status_symbols: Vec<StatusSymbol>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialRole {
    pub role_name: String,
    pub role_type: RoleType,
    pub responsibilities: Vec<String>,
    pub privileges: Vec<String>,
    pub performance_rating: f32,
    pub role_satisfaction: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoleType {
    Professional,
    Family,
    Community,
    Religious,
    Political,
    Recreational,
    Temporary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusSymbol {
    pub symbol_type: String,
    pub status_value: f32,
    pub recognition_level: f32, // How widely recognized this symbol is
    pub authenticity: f32, // How authentic/earned this symbol is
}

/// NPC relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCRelationship {
    pub relationship_id: EntityId,
    pub target_id: EntityId,
    pub relationship_type: String,
    pub emotional_bond: f32, // -1.0 (hatred) to 1.0 (love)
    pub trust_level: f32,    // 0.0 to 1.0
    pub familiarity: f32,    // 0.0 to 1.0
    pub power_dynamic: PowerDynamic,
    pub interaction_frequency: f32,
    pub relationship_stability: f32,
    pub shared_history: Vec<EntityId>, // Memory references
    pub conflict_history: Vec<ConflictRecord>,
    pub cooperation_history: Vec<CooperationRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PowerDynamic {
    Equal,
    Dominant,
    Submissive,
    Contextual, // Changes based on situation
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictRecord {
    pub conflict_id: EntityId,
    pub conflict_type: String,
    pub severity: f32,
    pub resolution_status: ResolutionStatus,
    pub lasting_effects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolutionStatus {
    Unresolved,
    Resolved,
    PartiallyResolved,
    Escalated,
    Forgiven,
    Forgotten,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CooperationRecord {
    pub cooperation_id: EntityId,
    pub cooperation_type: String,
    pub success_level: f32,
    pub benefits_gained: Vec<String>,
    pub lessons_learned: Vec<String>,
}

/// Group memberships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMembership {
    pub group_id: EntityId,
    pub group_type: GroupType,
    pub membership_status: MembershipStatus,
    pub role_in_group: String,
    pub influence_level: f32,
    pub commitment_level: f32,
    pub benefits_received: Vec<String>,
    pub contributions_made: Vec<String>,
    pub membership_duration: u64, // seconds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroupType {
    Family,
    Professional,
    Social,
    Religious,
    Political,
    Educational,
    Military,
    Criminal,
    Hobby,
    Temporary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MembershipStatus {
    Active,
    Inactive,
    Probationary,
    Leadership,
    Expelled,
    Honorary,
    Suspended,
}

/// Reputation profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReputationProfile {
    pub overall_reputation: f32, // -1.0 to 1.0
    pub reputation_by_group: HashMap<EntityId, f32>,
    pub reputation_aspects: Vec<ReputationAspect>,
    pub reputation_events: Vec<ReputationEvent>,
    pub reputation_trends: ReputationTrend,
    pub reputation_volatility: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReputationAspect {
    pub aspect_name: String, // e.g., "honesty", "competence", "reliability"
    pub rating: f32,         // -1.0 to 1.0
    pub certainty: f32,      // 0.0 to 1.0 - how certain others are about this
    pub importance: f32,     // 0.0 to 1.0 - how important this aspect is
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub event_id: EntityId,
    pub event_type: String,
    pub impact: f32,
    pub affected_groups: Vec<EntityId>,
    pub decay_rate: f32, // How quickly this event's impact fades
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReputationTrend {
    Rising,
    Falling,
    Stable,
    Volatile,
    Recovering,
    Declining,
}

/// Communication state and abilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunicationState {
    pub languages_known: Vec<LanguageProficiency>,
    pub communication_style: CommunicationStyle,
    pub current_conversation: Option<ConversationState>,
    pub communication_preferences: CommunicationPreferences,
    pub nonverbal_communication: NonverbalCommunication,
    pub listening_skills: f32,
    pub persuasion_ability: f32,
    pub deception_ability: f32,
    pub empathy_in_communication: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageProficiency {
    pub language: String,
    pub speaking_level: ProficiencyLevel,
    pub listening_level: ProficiencyLevel,
    pub reading_level: ProficiencyLevel,
    pub writing_level: ProficiencyLevel,
    pub accent: Option<String>,
    pub formal_register: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProficiencyLevel {
    None,
    Beginner,
    Elementary,
    Intermediate,
    UpperIntermediate,
    Advanced,
    Native,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Direct,
    Indirect,
    Passive,
    Aggressive,
    PassiveAggressive,
    Assertive,
    Diplomatic,
    Blunt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationState {
    pub conversation_id: EntityId,
    pub participants: Vec<EntityId>,
    pub topic: String,
    pub conversation_type: ConversationType,
    pub emotional_tone: f32,
    pub formality_level: f32,
    pub engagement_level: f32,
    pub turn_taking_pattern: TurnTakingPattern,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationType {
    Casual,
    Formal,
    Negotiation,
    Conflict,
    Instruction,
    Storytelling,
    Information_Exchange,
    Emotional_Support,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TurnTakingPattern {
    Balanced,
    Dominant,
    Submissive,
    Chaotic,
    Structured,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunicationPreferences {
    pub preferred_distance: f32, // Physical distance during conversation
    pub eye_contact_comfort: f32,
    pub touch_tolerance: TouchTolerance,
    pub volume_preference: VolumePreference,
    pub topic_preferences: Vec<TopicPreference>,
    pub communication_channels: Vec<CommunicationChannel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TouchTolerance {
    NoTouch,
    Minimal,
    Moderate,
    High,
    Cultural,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolumePreference {
    Whisper,
    Quiet,
    Normal,
    Loud,
    Variable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicPreference {
    pub topic: String,
    pub comfort_level: f32, // How comfortable discussing this topic
    pub expertise_level: f32,
    pub interest_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommunicationChannel {
    Verbal,
    Written,
    Gestural,
    Telepathic,
    Digital,
    Artistic,
    Musical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonverbalCommunication {
    pub body_language_expressiveness: f32,
    pub facial_expressiveness: f32,
    pub gesture_frequency: f32,
    pub personal_space_needs: f32,
    pub posture_patterns: Vec<PosturePattern>,
    pub micro_expressions: bool,
    pub cultural_gestures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosturePattern {
    pub pattern_name: String,
    pub emotional_association: String,
    pub frequency: f32,
    pub cultural_meaning: Option<String>,
}

// Default implementations
impl Default for SocialStatus {
    fn default() -> Self {
        Self {
            hierarchy_level: 0.5,
            social_roles: Vec::new(),
            influence_level: 0.3,
            social_capital: 0.4,
            network_centrality: 0.3,
            status_symbols: Vec::new(),
        }
    }
}

impl Default for ReputationProfile {
    fn default() -> Self {
        Self {
            overall_reputation: 0.0,
            reputation_by_group: HashMap::new(),
            reputation_aspects: Vec::new(),
            reputation_events: Vec::new(),
            reputation_trends: ReputationTrend::Stable,
            reputation_volatility: 0.3,
        }
    }
}

impl Default for CommunicationState {
    fn default() -> Self {
        Self {
            languages_known: vec![LanguageProficiency {
                language: "Common".to_string(),
                speaking_level: ProficiencyLevel::Native,
                listening_level: ProficiencyLevel::Native,
                reading_level: ProficiencyLevel::Native,
                writing_level: ProficiencyLevel::Native,
                accent: None,
                formal_register: true,
            }],
            communication_style: CommunicationStyle::Direct,
            current_conversation: None,
            communication_preferences: CommunicationPreferences::default(),
            nonverbal_communication: NonverbalCommunication::default(),
            listening_skills: 0.7,
            persuasion_ability: 0.5,
            deception_ability: 0.3,
            empathy_in_communication: 0.6,
        }
    }
}

impl Default for CommunicationPreferences {
    fn default() -> Self {
        Self {
            preferred_distance: 1.5, // meters
            eye_contact_comfort: 0.7,
            touch_tolerance: TouchTolerance::Moderate,
            volume_preference: VolumePreference::Normal,
            topic_preferences: Vec::new(),
            communication_channels: vec![CommunicationChannel::Verbal],
        }
    }
}

impl Default for NonverbalCommunication {
    fn default() -> Self {
        Self {
            body_language_expressiveness: 0.6,
            facial_expressiveness: 0.7,
            gesture_frequency: 0.5,
            personal_space_needs: 1.5,
            posture_patterns: Vec::new(),
            micro_expressions: true,
            cultural_gestures: Vec::new(),
        }
    }
}

// Utility implementations for social functions
impl NPCRelationship {
    /// Create a new relationship
    pub fn new(target_id: EntityId, relationship_type: String) -> Self {
        Self {
            relationship_id: Uuid::new_v4(),
            target_id,
            relationship_type,
            emotional_bond: 0.0,
            trust_level: 0.5,
            familiarity: 0.1,
            power_dynamic: PowerDynamic::Equal,
            interaction_frequency: 0.0,
            relationship_stability: 0.5,
            shared_history: Vec::new(),
            conflict_history: Vec::new(),
            cooperation_history: Vec::new(),
        }
    }
    
    /// Update relationship based on interaction
    pub fn update_from_interaction(&mut self, interaction_outcome: f32, trust_change: f32) {
        self.emotional_bond = (self.emotional_bond + interaction_outcome * 0.1).clamp(-1.0, 1.0);
        self.trust_level = (self.trust_level + trust_change).clamp(0.0, 1.0);
        self.familiarity = (self.familiarity + 0.01).min(1.0);
        self.interaction_frequency += 1.0;
    }
    
    /// Calculate relationship quality
    pub fn calculate_quality(&self) -> f32 {
        let emotional_component = (self.emotional_bond + 1.0) / 2.0; // Normalize from -1,1 to 0,1
        let trust_component = self.trust_level;
        let stability_component = self.relationship_stability;
        
        (emotional_component + trust_component + stability_component) / 3.0
    }
    
    /// Check if relationship is healthy
    pub fn is_healthy(&self) -> bool {
        self.emotional_bond > -0.5 && 
        self.trust_level > 0.3 && 
        self.relationship_stability > 0.4
    }
}

impl ReputationProfile {
    /// Update reputation based on event
    pub fn add_reputation_event(&mut self, event: ReputationEvent) {
        self.overall_reputation = (self.overall_reputation + event.impact * 0.1).clamp(-1.0, 1.0);
        
        // Update group-specific reputations
        for group_id in &event.affected_groups {
            let current_rep = self.reputation_by_group.get(group_id).unwrap_or(&0.0);
            let new_rep = (current_rep + event.impact * 0.2).clamp(-1.0, 1.0);
            self.reputation_by_group.insert(*group_id, new_rep);
        }
        
        self.reputation_events.push(event);
        
        // Update trend based on recent events
        self.update_reputation_trend();
    }
    
    /// Update reputation trend based on recent events
    fn update_reputation_trend(&mut self) {
        if self.reputation_events.len() < 3 {
            self.reputation_trends = ReputationTrend::Stable;
            return;
        }
        
        let recent_events: Vec<_> = self.reputation_events.iter()
            .rev()
            .take(5)
            .collect();
        
        let average_impact: f32 = recent_events.iter()
            .map(|e| e.impact)
            .sum::<f32>() / recent_events.len() as f32;
        
        self.reputation_trends = if average_impact > 0.2 {
            ReputationTrend::Rising
        } else if average_impact < -0.2 {
            ReputationTrend::Falling
        } else {
            ReputationTrend::Stable
        };
    }
    
    /// Get reputation with specific group
    pub fn get_group_reputation(&self, group_id: EntityId) -> f32 {
        self.reputation_by_group.get(&group_id).unwrap_or(&self.overall_reputation).clone()
    }
    
    /// Calculate reputation score for a specific aspect
    pub fn get_aspect_reputation(&self, aspect_name: &str) -> Option<f32> {
        self.reputation_aspects.iter()
            .find(|aspect| aspect.aspect_name == aspect_name)
            .map(|aspect| aspect.rating)
    }
}

impl CommunicationState {
    /// Check if NPC can communicate in a language
    pub fn can_communicate_in(&self, language: &str) -> bool {
        self.languages_known.iter()
            .any(|lang| lang.language == language && 
                       matches!(lang.speaking_level, ProficiencyLevel::Elementary | 
                               ProficiencyLevel::Intermediate | ProficiencyLevel::UpperIntermediate |
                               ProficiencyLevel::Advanced | ProficiencyLevel::Native))
    }
    
    /// Get communication effectiveness with another NPC
    pub fn calculate_communication_effectiveness(&self, other: &CommunicationState) -> f32 {
        // Find common languages
        let mut common_languages = Vec::new();
        for lang1 in &self.languages_known {
            for lang2 in &other.languages_known {
                if lang1.language == lang2.language {
                    let effectiveness = (lang1.speaking_level as i32).min(lang2.listening_level as i32) as f32 / 6.0;
                    common_languages.push(effectiveness);
                }
            }
        }
        
        let language_effectiveness = common_languages.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
        
        // Factor in communication styles
        let style_compatibility = match (&self.communication_style, &other.communication_style) {
            (CommunicationStyle::Direct, CommunicationStyle::Direct) => 0.9,
            (CommunicationStyle::Diplomatic, CommunicationStyle::Diplomatic) => 0.9,
            (CommunicationStyle::Direct, CommunicationStyle::Indirect) => 0.6,
            (CommunicationStyle::Aggressive, CommunicationStyle::Passive) => 0.3,
            _ => 0.7,
        };
        
        // Factor in empathy and listening skills
        let empathy_factor = (self.empathy_in_communication + other.empathy_in_communication) / 2.0;
        let listening_factor = (self.listening_skills + other.listening_skills) / 2.0;
        
        (language_effectiveness + style_compatibility + empathy_factor + listening_factor) / 4.0
    }
    
    /// Start a conversation
    pub fn start_conversation(&mut self, conversation: ConversationState) {
        self.current_conversation = Some(conversation);
    }
    
    /// End current conversation
    pub fn end_conversation(&mut self) {
        self.current_conversation = None;
    }
    
    /// Check if currently in conversation
    pub fn is_in_conversation(&self) -> bool {
        self.current_conversation.is_some()
    }
}

impl SocialStatus {
    /// Calculate overall social influence
    pub fn calculate_social_influence(&self) -> f32 {
        let hierarchy_factor = self.hierarchy_level;
        let role_factor = if self.social_roles.is_empty() {
            0.3
        } else {
            self.social_roles.iter()
                .map(|role| role.performance_rating)
                .sum::<f32>() / self.social_roles.len() as f32
        };
        
        let network_factor = self.network_centrality;
        let capital_factor = self.social_capital;
        let influence_factor = self.influence_level;
        
        (hierarchy_factor + role_factor + network_factor + capital_factor + influence_factor) / 5.0
    }
    
    /// Add a new social role
    pub fn add_role(&mut self, role: SocialRole) {
        // Remove existing role of same type if it exists
        self.social_roles.retain(|r| r.role_name != role.role_name);
        self.social_roles.push(role);
    }
    
    /// Remove a social role
    pub fn remove_role(&mut self, role_name: &str) {
        self.social_roles.retain(|r| r.role_name != role_name);
    }
    
    /// Get role by name
    pub fn get_role(&self, role_name: &str) -> Option<&SocialRole> {
        self.social_roles.iter().find(|r| r.role_name == role_name)
    }
    
    /// Update role performance
    pub fn update_role_performance(&mut self, role_name: &str, performance: f32) {
        if let Some(role) = self.social_roles.iter_mut().find(|r| r.role_name == role_name) {
            role.performance_rating = performance.clamp(0.0, 1.0);
        }
    }
    
    /// Calculate status from symbols
    pub fn calculate_symbol_status(&self) -> f32 {
        if self.status_symbols.is_empty() {
            return 0.0;
        }
        
        self.status_symbols.iter()
            .map(|symbol| symbol.status_value * symbol.recognition_level * symbol.authenticity)
            .sum::<f32>() / self.status_symbols.len() as f32
    }
}

impl GroupMembership {
    /// Create new group membership
    pub fn new(group_id: EntityId, group_type: GroupType, role: String) -> Self {
        Self {
            group_id,
            group_type,
            membership_status: MembershipStatus::Active,
            role_in_group: role,
            influence_level: 0.1,
            commitment_level: 0.5,
            benefits_received: Vec::new(),
            contributions_made: Vec::new(),
            membership_duration: 0,
        }
    }
    
    /// Update membership duration
    pub fn update_duration(&mut self, additional_seconds: u64) {
        self.membership_duration += additional_seconds;
    }
    
    /// Add contribution
    pub fn add_contribution(&mut self, contribution: String) {
        self.contributions_made.push(contribution);
        // Increase influence based on contributions
        self.influence_level = (self.influence_level + 0.01).min(1.0);
    }
    
    /// Add benefit received
    pub fn add_benefit(&mut self, benefit: String) {
        self.benefits_received.push(benefit);
    }
    
    /// Calculate membership value
    pub fn calculate_membership_value(&self) -> f32 {
        let status_value = match self.membership_status {
            MembershipStatus::Leadership => 1.0,
            MembershipStatus::Active => 0.8,
            MembershipStatus::Honorary => 0.6,
            MembershipStatus::Inactive => 0.3,
            MembershipStatus::Probationary => 0.5,
            MembershipStatus::Suspended => 0.1,
            MembershipStatus::Expelled => 0.0,
        };
        
        let contribution_value = self.contributions_made.len() as f32 * 0.1;
        let benefit_value = self.benefits_received.len() as f32 * 0.05;
        let duration_value = (self.membership_duration as f32 / 86400.0).min(10.0) * 0.1; // Days capped at 10
        
        (status_value + contribution_value + benefit_value + duration_value + 
         self.influence_level + self.commitment_level) / 6.0
    }
    
    /// Check if membership is beneficial
    pub fn is_beneficial(&self) -> bool {
        matches!(self.membership_status, MembershipStatus::Active | MembershipStatus::Leadership | MembershipStatus::Honorary) &&
        self.benefits_received.len() >= self.contributions_made.len() / 2
    }
}