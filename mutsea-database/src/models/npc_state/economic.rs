// mutsea-database/src/models/npc_state/economic.rs
//! Economic state models for NPCs including inventory, trade, and resource management

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inventory and possessions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCInventory {
    pub items: Vec<InventoryItem>,
    pub carrying_capacity: f32, // kg
    pub current_weight: f32,
    pub organization_system: OrganizationSystem,
    pub access_speed: f32, // How quickly items can be accessed
    pub security_level: f32,
    pub storage_locations: Vec<StorageLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: EntityId,
    pub item_name: String,
    pub item_type: String,
    pub quantity: f32,
    pub weight_per_unit: f32,
    pub condition: ItemCondition,
    pub value: f32,
    pub sentimental_value: f32,
    pub utility_rating: f32,
    pub last_used: Option<Timestamp>,
    pub acquisition_method: AcquisitionMethod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AcquisitionMethod {
    Purchased,
    Found,
    Gifted,
    Inherited,
    Created,
    Stolen,
    Traded,
    Reward,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrganizationSystem {
    None,
    ByType,
    ByValue,
    ByFrequencyOfUse,
    ByWeight,
    BySize,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageLocation {
    pub location_name: String,
    pub location_type: StorageType,
    pub capacity: f32,
    pub security: f32,
    pub accessibility: f32,
    pub environmental_protection: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    OnPerson,
    Home,
    Workplace,
    Bank,
    Warehouse,
    Hidden,
    Magical,
    Digital,
}

/// Economic status and trade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicStatus {
    pub wealth_level: WealthLevel,
    pub liquid_assets: f32,
    pub fixed_assets: Vec<Asset>,
    pub debts: Vec<Debt>,
    pub income_sources: Vec<IncomeSource>,
    pub expenses: Vec<Expense>,
    pub financial_literacy: f32,
    pub investment_behavior: InvestmentBehavior,
    pub economic_goals: Vec<EconomicGoal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WealthLevel {
    Destitute,
    Poor,
    LowerMiddle,
    Middle,
    UpperMiddle,
    Wealthy,
    VeryWealthy,
    Elite,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub asset_name: String,
    pub asset_type: AssetType,
    pub current_value: f32,
    pub acquisition_cost: f32,
    pub liquidity: f32, // How easily converted to cash
    pub appreciation_rate: f32,
    pub maintenance_cost: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    Real_Estate,
    Vehicle,
    Equipment,
    Inventory,
    Investment,
    Collectible,
    Intellectual_Property,
    Business,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debt {
    pub debt_type: String,
    pub principal_amount: f32,
    pub current_balance: f32,
    pub interest_rate: f32,
    pub creditor: EntityId,
    pub payment_schedule: PaymentSchedule,
    pub collateral: Option<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentSchedule {
    OnDemand,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeSource {
    pub source_name: String,
    pub source_type: IncomeType,
    pub amount_per_period: f32,
    pub period: TimePeriod,
    pub reliability: f32,
    pub growth_potential: f32,
    pub required_effort: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IncomeType {
    Salary,
    Wages,
    Business_Profit,
    Investment_Return,
    Rental_Income,
    Royalties,
    Pension,
    Government_Benefit,
    Gifts,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    Irregular,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    pub expense_name: String,
    pub expense_type: ExpenseType,
    pub amount_per_period: f32,
    pub period: TimePeriod,
    pub necessity_level: f32, // 0.0 luxury to 1.0 necessity
    pub variability: f32, // How much this expense varies
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpenseType {
    Housing,
    Food,
    Transportation,
    Healthcare,
    Education,
    Entertainment,
    Clothing,
    Utilities,
    Insurance,
    Taxes,
    Debt_Service,
    Savings,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvestmentBehavior {
    Conservative,
    Moderate,
    Aggressive,
    Speculative,
    None,
    Inconsistent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicGoal {
    pub goal_name: String,
    pub target_amount: f32,
    pub target_date: Option<Timestamp>,
    pub current_progress: f32,
    pub priority: f32,
    pub strategies: Vec<String>,
}

/// Trade preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradePreferences {
    pub preferred_trade_types: Vec<TradeType>,
    pub haggling_ability: f32,
    pub trust_requirements: f32,
    pub quality_vs_price_preference: f32, // 0.0 price focused, 1.0 quality focused
    pub bulk_vs_individual_preference: f32,
    pub preferred_payment_methods: Vec<PaymentMethod>,
    pub trade_reputation: f32,
    pub specialized_markets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
    Barter,
    Rent,
    Lease,
    Commission,
    Auction,
    Subscription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    Credit,
    Barter,
    Service,
    Future_Favor,
    Reputation,
    Information,
    Magical_Currency,
}

/// Resource needs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceNeed {
    pub resource_type: String,
    pub urgency: f32, // 0.0 to 1.0
    pub quantity_needed: f32,
    pub acceptable_quality: f32,
    pub maximum_price: Option<f32>,
    pub alternative_resources: Vec<String>,
    pub consequences_of_not_obtaining: Vec<String>,
}

// Default implementations
impl Default for NPCInventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            carrying_capacity: 50.0, // kg
            current_weight: 0.0,
            organization_system: OrganizationSystem::ByType,
            access_speed: 0.7,
            security_level: 0.3,
            storage_locations: Vec::new(),
        }
    }
}

impl Default for EconomicStatus {
    fn default() -> Self {
        Self {
            wealth_level: WealthLevel::LowerMiddle,
            liquid_assets: 1000.0,
            fixed_assets: Vec::new(),
            debts: Vec::new(),
            income_sources: Vec::new(),
            expenses: Vec::new(),
            financial_literacy: 0.5,
            investment_behavior: InvestmentBehavior::Conservative,
            economic_goals: Vec::new(),
        }
    }
}

impl Default for TradePreferences {
    fn default() -> Self {
        Self {
            preferred_trade_types: vec![TradeType::Buy, TradeType::Sell],
            haggling_ability: 0.5,
            trust_requirements: 0.6,
            quality_vs_price_preference: 0.5,
            bulk_vs_individual_preference: 0.4,
            preferred_payment_methods: vec![PaymentMethod::Cash],
            trade_reputation: 0.5,
            specialized_markets: Vec::new(),
        }
    }
}

// Utility implementations for economic functions
impl NPCInventory {
    /// Add item to inventory
    pub fn add_item(&mut self, item: InventoryItem) -> Result<(), String> {
        let item_weight = item.weight_per_unit * item.quantity;
        
        if self.current_weight + item_weight > self.carrying_capacity {
            return Err("Inventory capacity exceeded".to_string());
        }
        
        // Check if item already exists and can be stacked
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.item_name == item.item_name) {
            existing_item.quantity += item.quantity;
            self.current_weight += item_weight;
        } else {
            self.current_weight += item_weight;
            self.items.push(item);
        }
        
        Ok(())
    }
    
    /// Remove item from inventory
    pub fn remove_item(&mut self, item_name: &str, quantity: f32) -> Result<(), String> {
        if let Some(item_index) = self.items.iter().position(|i| i.item_name == item_name) {
            let item = &mut self.items[item_index];
            
            if item.quantity < quantity {
                return Err("Insufficient quantity".to_string());
            }
            
            let weight_removed = item.weight_per_unit * quantity;
            item.quantity -= quantity;
            self.current_weight -= weight_removed;
            
            // Remove item if quantity reaches zero
            if item.quantity <= 0.0 {
                self.items.remove(item_index);
            }
            
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }
    
    /// Get item by name
    pub fn get_item(&self, item_name: &str) -> Option<&InventoryItem> {
        self.items.iter().find(|i| i.item_name == item_name)
    }
    
    /// Calculate total inventory value
    pub fn calculate_total_value(&self) -> f32 {
        self.items.iter().map(|item| item.value * item.quantity).sum()
    }
    
    /// Check if inventory has space for item
    pub fn has_space_for(&self, weight: f32) -> bool {
        self.current_weight + weight <= self.carrying_capacity
    }
    
    /// Get capacity utilization percentage
    pub fn capacity_utilization(&self) -> f32 {
        if self.carrying_capacity > 0.0 {
            self.current_weight / self.carrying_capacity
        } else {
            0.0
        }
    }
    
    /// Find items by type
    pub fn find_items_by_type(&self, item_type: &str) -> Vec<&InventoryItem> {
        self.items.iter().filter(|item| item.item_type == item_type).collect()
    }
    
    /// Get most valuable items
    pub fn get_most_valuable_items(&self, count: usize) -> Vec<&InventoryItem> {
        let mut items = self.items.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| (b.value * b.quantity).partial_cmp(&(a.value * a.quantity)).unwrap());
        items.into_iter().take(count).collect()
    }
}

impl EconomicStatus {
    /// Calculate net worth
    pub fn calculate_net_worth(&self) -> f32 {
        let asset_value: f32 = self.fixed_assets.iter().map(|a| a.current_value).sum();
        let debt_value: f32 = self.debts.iter().map(|d| d.current_balance).sum();
        
        self.liquid_assets + asset_value - debt_value
    }
    
    /// Calculate monthly income
    pub fn calculate_monthly_income(&self) -> f32 {
        self.income_sources.iter().map(|income| {
            match income.period {
                TimePeriod::Daily => income.amount_per_period * 30.0,
                TimePeriod::Weekly => income.amount_per_period * 4.33,
                TimePeriod::Monthly => income.amount_per_period,
                TimePeriod::Quarterly => income.amount_per_period / 3.0,
                TimePeriod::Annually => income.amount_per_period / 12.0,
                TimePeriod::Irregular => income.amount_per_period * 0.5, // Estimate
            }
        }).sum()
    }
    
    /// Calculate monthly expenses
    pub fn calculate_monthly_expenses(&self) -> f32 {
        self.expenses.iter().map(|expense| {
            match expense.period {
                TimePeriod::Daily => expense.amount_per_period * 30.0,
                TimePeriod::Weekly => expense.amount_per_period * 4.33,
                TimePeriod::Monthly => expense.amount_per_period,
                TimePeriod::Quarterly => expense.amount_per_period / 3.0,
                TimePeriod::Annually => expense.amount_per_period / 12.0,
                TimePeriod::Irregular => expense.amount_per_period * 0.5, // Estimate
            }
        }).sum()
    }
    
    /// Calculate financial health score
    pub fn calculate_financial_health(&self) -> f32 {
        let net_worth = self.calculate_net_worth();
        let monthly_income = self.calculate_monthly_income();
        let monthly_expenses = self.calculate_monthly_expenses();
        
        let cash_flow_ratio = if monthly_expenses > 0.0 {
            monthly_income / monthly_expenses
        } else {
            2.0 // Good if no expenses
        };
        
        let debt_to_income_ratio = if monthly_income > 0.0 {
            let monthly_debt_payments: f32 = self.debts.iter()
                .map(|debt| debt.current_balance * debt.interest_rate / 12.0)
                .sum();
            monthly_debt_payments / monthly_income
        } else {
            1.0 // Bad if no income but have debt
        };
        
        let liquidity_score = if net_worth > 0.0 {
            self.liquid_assets / net_worth
        } else {
            0.0
        };
        
        // Combine factors (higher is better)
        let cash_flow_score = cash_flow_ratio.min(2.0) / 2.0;
        let debt_score = (1.0 - debt_to_income_ratio.min(1.0)).max(0.0);
        let liquidity_score_normalized = liquidity_score.min(1.0);
        
        (cash_flow_score + debt_score + liquidity_score_normalized) / 3.0
    }
    
    /// Add income source
    pub fn add_income_source(&mut self, income: IncomeSource) {
        self.income_sources.push(income);
    }
    
    /// Add expense
    pub fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }
    
    /// Add debt
    pub fn add_debt(&mut self, debt: Debt) {
        self.debts.push(debt);
        self.update_wealth_level();
    }
    
    /// Add asset
    pub fn add_asset(&mut self, asset: Asset) {
        self.fixed_assets.push(asset);
        self.update_wealth_level();
    }
    
    /// Update wealth level based on net worth
    fn update_wealth_level(&mut self) {
        let net_worth = self.calculate_net_worth();
        
        self.wealth_level = match net_worth {
            n if n < 0.0 => WealthLevel::Destitute,
            n if n < 1000.0 => WealthLevel::Poor,
            n if n < 10000.0 => WealthLevel::LowerMiddle,
            n if n < 100000.0 => WealthLevel::Middle,
            n if n < 500000.0 => WealthLevel::UpperMiddle,
            n if n < 1000000.0 => WealthLevel::Wealthy,
            n if n < 10000000.0 => WealthLevel::VeryWealthy,
            _ => WealthLevel::Elite,
        };
    }
}

impl TradePreferences {
    /// Calculate trade compatibility with another NPC
    pub fn calculate_trade_compatibility(&self, other: &TradePreferences) -> f32 {
        // Check overlapping trade types
        let common_types = self.preferred_trade_types.iter()
            .filter(|&t1| other.preferred_trade_types.iter().any(|t2| t1 == t2))
            .count();
        
        let type_compatibility = if self.preferred_trade_types.is_empty() || other.preferred_trade_types.is_empty() {
            0.5
        } else {
            common_types as f32 / self.preferred_trade_types.len().max(other.preferred_trade_types.len()) as f32
        };
        
        // Check payment method compatibility
        let common_payments = self.preferred_payment_methods.iter()
            .filter(|&p1| other.preferred_payment_methods.iter().any(|p2| p1 == p2))
            .count();
        
        let payment_compatibility = if self.preferred_payment_methods.is_empty() || other.preferred_payment_methods.is_empty() {
            0.5
        } else {
            common_payments as f32 / self.preferred_payment_methods.len().max(other.preferred_payment_methods.len()) as f32
        };
        
        // Factor in trust and reputation
        let trust_compatibility = 1.0 - (self.trust_requirements - other.trade_reputation).abs();
        let reputation_compatibility = 1.0 - (other.trust_requirements - self.trade_reputation).abs();
        
        (type_compatibility + payment_compatibility + trust_compatibility + reputation_compatibility) / 4.0
    }
    
    /// Check if willing to trade with reputation level
    pub fn willing_to_trade_with(&self, reputation: f32) -> bool {
        reputation >= self.trust_requirements
    }
    
    /// Calculate expected haggling duration
    pub fn estimate_haggling_duration(&self, other_haggling_ability: f32) -> f32 {
        let skill_difference = (self.haggling_ability - other_haggling_ability).abs();
        let base_duration = 60.0; // seconds
        
        base_duration * (1.0 + skill_difference)
    }
}