//! Application state management
//!
//! Defines the state structure and message types for state updates.

use serde::{Deserialize, Serialize};
use crate::components::metrics_state::MetricsState;

/// Main application state
///
/// This structure holds all the application's state that needs to be
/// synchronized across different screens and components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Current screen being displayed
    pub current_screen: Screen,

    /// Application title
    pub title: String,

    /// Counter for example interactions
    pub counter: i32,

    /// List of items for demonstration
    pub items: Vec<String>,

    /// Selected item index
    pub selected_index: usize,

    /// System metrics (not serialized due to complexity)
    #[serde(skip)]
    pub metrics: Option<MetricsState>,

    /// Selected process index in dashboard
    pub selected_process: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::Home,
            title: String::from("WebATUI Reference"),
            counter: 0,
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            selected_index: 0,
            metrics: Some(MetricsState::default()),
            selected_process: 0,
        }
    }
}

/// Available screens in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    /// Home/landing screen
    Home,

    /// Dashboard with multiple widgets
    Dashboard,

    /// Interactive example screen
    Interactive,

    /// Settings screen
    Settings,
}

impl Screen {
    /// Get the display name for the screen
    pub fn name(&self) -> &'static str {
        match self {
            Screen::Home => "Home",
            Screen::Dashboard => "Dashboard",
            Screen::Interactive => "Interactive",
            Screen::Settings => "Settings",
        }
    }
}

/// Messages for state updates
///
/// These messages represent all possible state changes in the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// Quit the application
    Quit,

    /// Navigate to a specific screen
    Navigate(Screen),

    /// Increment counter
    Increment,

    /// Decrement counter
    Decrement,

    /// Select next item
    SelectNext,

    /// Select previous item
    SelectPrevious,

    /// Add a new item
    AddItem(String),

    /// Remove selected item
    RemoveItem,

    /// Update metrics (for dashboard)
    UpdateMetrics,

    /// Select next process in dashboard
    SelectNextProcess,

    /// Select previous process in dashboard
    SelectPreviousProcess,

    /// Refresh dashboard data
    RefreshDashboard,
}
