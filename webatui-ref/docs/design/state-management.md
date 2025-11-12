# State Management Architecture

## Overview

This document details the state management strategy for the webatui reference application, focusing on predictable state updates, efficient rendering, and cross-platform compatibility.

## Core Principles

1. **Single Source of Truth**: All application state lives in one place
2. **Immutable Updates**: State transitions create new state rather than mutating
3. **Event-Driven**: All state changes triggered by events
4. **Predictable**: Same input always produces same output
5. **Serializable**: State can be persisted and restored

## State Architecture

### 1. State Tree Structure

```rust
pub struct AppState {
    // Core application state
    pub session: SessionState,
    pub navigation: NavigationState,
    pub config: ConfigState,
    pub ui: UiState,

    // Feature-specific state
    pub dashboard: DashboardState,
    pub settings: SettingsState,
    pub data_view: DataViewState,

    // Runtime state
    pub runtime: RuntimeState,
}
```

### 2. State Modules

#### Session State
```rust
pub struct SessionState {
    pub session_id: Uuid,
    pub start_time: Instant,
    pub last_activity: Instant,
    pub user_preferences: UserPreferences,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
            start_time: Instant::now(),
            last_activity: Instant::now(),
            user_preferences: UserPreferences::load().unwrap_or_default(),
        }
    }

    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn duration(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }
}
```

#### Navigation State
```rust
pub struct NavigationState {
    pub current_screen: ScreenType,
    pub history: Vec<ScreenType>,
    pub max_history: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenType {
    Dashboard,
    Settings,
    DataView,
    Help,
}

impl NavigationState {
    pub fn navigate_to(&mut self, screen: ScreenType) {
        if self.current_screen != screen {
            self.history.push(self.current_screen);
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }
            self.current_screen = screen;
        }
    }

    pub fn navigate_back(&mut self) -> Option<ScreenType> {
        self.history.pop().map(|screen| {
            self.current_screen = screen;
            screen
        })
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }
}
```

#### Configuration State
```rust
pub struct ConfigState {
    pub theme: Theme,
    pub performance: PerformanceConfig,
    pub display: DisplayConfig,
    pub data: DataConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub refresh_rate: u32,
    pub max_data_points: usize,
    pub enable_animations: bool,
    pub batch_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub font_size: FontSize,
    pub color_scheme: ColorScheme,
    pub show_borders: bool,
    pub compact_mode: bool,
}

impl ConfigState {
    pub fn load() -> Result<Self> {
        let storage = Storage::new();
        storage.load("config")
            .map(|json| serde_json::from_str(&json))
            .unwrap_or_else(|_| Ok(Self::default()))
    }

    pub fn save(&self) -> Result<()> {
        let storage = Storage::new();
        let json = serde_json::to_string(self)?;
        storage.save("config", &json)
    }
}
```

#### UI State
```rust
pub struct UiState {
    pub focus: FocusState,
    pub selection: SelectionState,
    pub input: InputState,
    pub modal: Option<ModalState>,
}

pub struct FocusState {
    pub focused_widget: Option<WidgetId>,
    pub focus_stack: Vec<WidgetId>,
}

pub struct SelectionState {
    pub selected_items: HashMap<ScreenType, Vec<usize>>,
    pub multi_select: bool,
}

pub struct InputState {
    pub active_inputs: HashMap<WidgetId, String>,
    pub cursor_positions: HashMap<WidgetId, usize>,
}
```

#### Dashboard State
```rust
pub struct DashboardState {
    pub metrics: MetricsState,
    pub processes: Vec<ProcessInfo>,
    pub network: NetworkState,
    pub system: SystemInfo,
}

pub struct MetricsState {
    pub cpu_history: CircularBuffer<f64>,
    pub memory_history: CircularBuffer<f64>,
    pub network_history: CircularBuffer<(f64, f64)>,
    pub last_update: Instant,
    pub update_interval: Duration,
}

impl MetricsState {
    pub fn new(capacity: usize) -> Self {
        Self {
            cpu_history: CircularBuffer::new(capacity),
            memory_history: CircularBuffer::new(capacity),
            network_history: CircularBuffer::new(capacity),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(1000),
        }
    }

    pub fn should_update(&self) -> bool {
        Instant::now().duration_since(self.last_update) >= self.update_interval
    }

    pub fn update(&mut self, metrics: SystemMetrics) {
        self.cpu_history.push(metrics.cpu_usage);
        self.memory_history.push(metrics.memory_usage);
        self.network_history.push((metrics.network_rx, metrics.network_tx));
        self.last_update = Instant::now();
    }
}
```

#### Runtime State
```rust
pub struct RuntimeState {
    pub fps: f64,
    pub frame_time: Duration,
    pub render_time: Duration,
    pub dirty_flags: DirtyFlags,
    pub pending_events: VecDeque<AppEvent>,
}

pub struct DirtyFlags {
    pub full_render: bool,
    pub screens: HashSet<ScreenType>,
    pub widgets: HashSet<WidgetId>,
}

impl DirtyFlags {
    pub fn mark_dirty(&mut self, target: DirtyTarget) {
        match target {
            DirtyTarget::FullScreen => {
                self.full_render = true;
            }
            DirtyTarget::Screen(screen) => {
                self.screens.insert(screen);
            }
            DirtyTarget::Widget(widget) => {
                self.widgets.insert(widget);
            }
        }
    }

    pub fn clear(&mut self) {
        self.full_render = false;
        self.screens.clear();
        self.widgets.clear();
    }
}
```

## Event System

### 1. Event Types

```rust
#[derive(Debug, Clone)]
pub enum AppEvent {
    // Input events
    KeyPress(KeyEvent),
    MouseClick(MouseEvent),
    MouseScroll(i32),
    Resize(u16, u16),

    // Navigation events
    NavigateTo(ScreenType),
    NavigateBack,

    // State update events
    ConfigUpdate(ConfigChange),
    ThemeChange(Theme),
    MetricsUpdate(SystemMetrics),

    // UI events
    FocusChange(WidgetId),
    SelectionChange(SelectionChange),
    InputChange(WidgetId, String),

    // System events
    TimerTick,
    Refresh,
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: u16,
    pub y: u16,
    pub button: MouseButton,
    pub modifiers: KeyModifiers,
}
```

### 2. Event Handling

```rust
pub trait EventHandler {
    fn handle_event(&mut self, event: AppEvent) -> Result<EventResult>;
}

pub enum EventResult {
    Handled,
    NotHandled,
    StateChanged,
    ShouldRender,
}

impl AppState {
    pub fn handle_event(&mut self, event: AppEvent) -> Result<EventResult> {
        // Update session activity
        self.session.touch();

        // Route event to appropriate handler
        let result = match &event {
            AppEvent::KeyPress(key) => self.handle_key_press(key)?,
            AppEvent::MouseClick(mouse) => self.handle_mouse_click(mouse)?,
            AppEvent::NavigateTo(screen) => {
                self.navigation.navigate_to(*screen);
                EventResult::StateChanged
            }
            AppEvent::ConfigUpdate(change) => {
                self.apply_config_change(change)?;
                EventResult::StateChanged
            }
            AppEvent::MetricsUpdate(metrics) => {
                if self.dashboard.metrics.should_update() {
                    self.dashboard.metrics.update(metrics.clone());
                    EventResult::StateChanged
                } else {
                    EventResult::Handled
                }
            }
            _ => EventResult::NotHandled,
        };

        // Mark dirty if state changed
        if matches!(result, EventResult::StateChanged) {
            self.runtime.dirty_flags.mark_dirty(DirtyTarget::FullScreen);
        }

        Ok(result)
    }

    fn handle_key_press(&mut self, key: &KeyEvent) -> Result<EventResult> {
        // Global shortcuts
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.handle_event(AppEvent::Shutdown)?;
                return Ok(EventResult::Handled);
            }
            KeyCode::Char('1') => {
                self.handle_event(AppEvent::NavigateTo(ScreenType::Dashboard))?;
                return Ok(EventResult::Handled);
            }
            KeyCode::Char('2') => {
                self.handle_event(AppEvent::NavigateTo(ScreenType::Settings))?;
                return Ok(EventResult::Handled);
            }
            KeyCode::Esc => {
                if self.navigation.can_go_back() {
                    self.handle_event(AppEvent::NavigateBack)?;
                    return Ok(EventResult::Handled);
                }
            }
            _ => {}
        }

        // Delegate to current screen
        self.handle_screen_event(key)
    }

    fn handle_mouse_click(&mut self, mouse: &MouseEvent) -> Result<EventResult> {
        // Find widget at position
        if let Some(widget_id) = self.find_widget_at(mouse.x, mouse.y) {
            self.ui.focus.focused_widget = Some(widget_id);
            // Trigger widget callback
            self.trigger_widget_callback(widget_id)?;
            return Ok(EventResult::StateChanged);
        }

        Ok(EventResult::NotHandled)
    }
}
```

## State Persistence

### 1. Storage Interface

```rust
pub trait StorageBackend {
    fn save(&self, key: &str, value: &str) -> Result<()>;
    fn load(&self, key: &str) -> Result<Option<String>>;
    fn delete(&self, key: &str) -> Result<()>;
    fn list_keys(&self) -> Result<Vec<String>>;
}

// WASM implementation
#[cfg(target_arch = "wasm32")]
pub struct LocalStorage;

impl StorageBackend for LocalStorage {
    fn save(&self, key: &str, value: &str) -> Result<()> {
        let window = web_sys::window().ok_or_else(|| anyhow!("No window"))?;
        let storage = window
            .local_storage()?
            .ok_or_else(|| anyhow!("No local storage"))?;
        storage.set_item(key, value)?;
        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<String>> {
        let window = web_sys::window().ok_or_else(|| anyhow!("No window"))?;
        let storage = window
            .local_storage()?
            .ok_or_else(|| anyhow!("No local storage"))?;
        Ok(storage.get_item(key)?)
    }
}

// Native implementation
#[cfg(not(target_arch = "wasm32"))]
pub struct FileStorage {
    base_path: PathBuf,
}

impl StorageBackend for FileStorage {
    fn save(&self, key: &str, value: &str) -> Result<()> {
        let path = self.base_path.join(format!("{}.json", key));
        std::fs::write(path, value)?;
        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<String>> {
        let path = self.base_path.join(format!("{}.json", key));
        if path.exists() {
            Ok(Some(std::fs::read_to_string(path)?))
        } else {
            Ok(None)
        }
    }
}
```

### 2. State Serialization

```rust
pub trait Persistable: Serialize + DeserializeOwned {
    fn persist_key() -> &'static str;

    fn save_to_storage(&self) -> Result<()> {
        let storage = Storage::new();
        let json = serde_json::to_string(self)?;
        storage.save(Self::persist_key(), &json)
    }

    fn load_from_storage() -> Result<Option<Self>> {
        let storage = Storage::new();
        storage.load(Self::persist_key())
            .and_then(|opt| opt.map(|json| serde_json::from_str(&json)).transpose())
    }
}

impl Persistable for ConfigState {
    fn persist_key() -> &'static str {
        "app_config"
    }
}

impl Persistable for UserPreferences {
    fn persist_key() -> &'static str {
        "user_preferences"
    }
}
```

### 3. Auto-save Strategy

```rust
pub struct AutoSaver {
    interval: Duration,
    last_save: Instant,
    pending_changes: bool,
}

impl AutoSaver {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_save: Instant::now(),
            pending_changes: false,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.pending_changes = true;
    }

    pub fn should_save(&self) -> bool {
        self.pending_changes
            && Instant::now().duration_since(self.last_save) >= self.interval
    }

    pub fn saved(&mut self) {
        self.pending_changes = false;
        self.last_save = Instant::now();
    }
}

// Usage in app
impl AppState {
    pub fn tick(&mut self) -> Result<()> {
        if self.auto_saver.should_save() {
            self.config.save_to_storage()?;
            self.session.user_preferences.save_to_storage()?;
            self.auto_saver.saved();
        }
        Ok(())
    }
}
```

## State Updates

### 1. Immutable Updates

```rust
// Use builder pattern for state updates
impl ConfigState {
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_refresh_rate(mut self, rate: u32) -> Self {
        self.performance.refresh_rate = rate;
        self
    }
}

// Or use update methods that return Result
impl MetricsState {
    pub fn add_sample(&mut self, metric: MetricType, value: f64) -> Result<()> {
        match metric {
            MetricType::Cpu => self.cpu_history.push(value),
            MetricType::Memory => self.memory_history.push(value),
            MetricType::Network => return Err(anyhow!("Use add_network_sample")),
        }
        self.last_update = Instant::now();
        Ok(())
    }
}
```

### 2. Batch Updates

```rust
pub struct StateUpdateBatch {
    updates: Vec<Box<dyn Fn(&mut AppState) -> Result<()>>>,
}

impl StateUpdateBatch {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, update: F)
    where
        F: Fn(&mut AppState) -> Result<()> + 'static
    {
        self.updates.push(Box::new(update));
    }

    pub fn apply(self, state: &mut AppState) -> Result<()> {
        for update in self.updates {
            update(state)?;
        }
        Ok(())
    }
}

// Usage
let mut batch = StateUpdateBatch::new();
batch.add(|state| {
    state.config.theme = Theme::Dark;
    Ok(())
});
batch.add(|state| {
    state.navigation.navigate_to(ScreenType::Dashboard);
    Ok(())
});
batch.apply(&mut app_state)?;
```

### 3. Transactional Updates

```rust
pub struct Transaction<'a> {
    state: &'a mut AppState,
    snapshot: Option<AppState>,
}

impl<'a> Transaction<'a> {
    pub fn begin(state: &'a mut AppState) -> Self {
        let snapshot = state.clone();
        Self {
            state,
            snapshot: Some(snapshot),
        }
    }

    pub fn commit(mut self) {
        self.snapshot = None;
    }

    pub fn rollback(mut self) {
        if let Some(snapshot) = self.snapshot.take() {
            *self.state = snapshot;
        }
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if self.snapshot.is_some() {
            // Implicit rollback if not committed
            self.rollback();
        }
    }
}

// Usage
{
    let mut txn = Transaction::begin(&mut app_state);
    txn.state.config.theme = Theme::Dark;

    if validate_theme_change(&txn.state)? {
        txn.commit();
    } else {
        // Implicit rollback on drop
    }
}
```

## Performance Optimizations

### 1. Lazy Evaluation

```rust
pub struct LazyState<T> {
    value: Option<T>,
    compute: Box<dyn Fn() -> T>,
}

impl<T> LazyState<T> {
    pub fn new<F>(compute: F) -> Self
    where
        F: Fn() -> T + 'static
    {
        Self {
            value: None,
            compute: Box::new(compute),
        }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            self.value = Some((self.compute)());
        }
        self.value.as_ref().unwrap()
    }

    pub fn invalidate(&mut self) {
        self.value = None;
    }
}
```

### 2. Memoization

```rust
pub struct MemoizedValue<T, F>
where
    F: Fn() -> T,
    T: Clone + PartialEq,
{
    compute: F,
    cached: Option<T>,
    dependencies: Vec<u64>, // Hash of dependencies
}

impl<T, F> MemoizedValue<T, F>
where
    F: Fn() -> T,
    T: Clone + PartialEq,
{
    pub fn get(&mut self, deps: &[u64]) -> T {
        if self.cached.is_none() || self.dependencies != deps {
            self.cached = Some((self.compute)());
            self.dependencies = deps.to_vec();
        }
        self.cached.as_ref().unwrap().clone()
    }
}
```

### 3. Circular Buffers

```rust
pub struct CircularBuffer<T> {
    data: Vec<T>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T: Default + Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![T::default(); capacity],
            capacity,
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data[self.head] = value;
        self.head = (self.head + 1) % self.capacity;
        if self.len < self.capacity {
            self.len += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let start = if self.len == self.capacity {
            self.head
        } else {
            0
        };

        (0..self.len)
            .map(move |i| &self.data[(start + i) % self.capacity])
    }
}
```

## Testing State Management

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_state() {
        let mut nav = NavigationState {
            current_screen: ScreenType::Dashboard,
            history: Vec::new(),
            max_history: 10,
        };

        nav.navigate_to(ScreenType::Settings);
        assert_eq!(nav.current_screen, ScreenType::Settings);
        assert_eq!(nav.history.len(), 1);

        nav.navigate_back();
        assert_eq!(nav.current_screen, ScreenType::Dashboard);
        assert_eq!(nav.history.len(), 0);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4); // Overwrites 1

        let values: Vec<_> = buffer.iter().cloned().collect();
        assert_eq!(values, vec![2, 3, 4]);
    }
}
```

### 2. Property Tests

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_state_serialization(config: ConfigState) {
            let json = serde_json::to_string(&config).unwrap();
            let restored: ConfigState = serde_json::from_str(&json).unwrap();
            assert_eq!(config, restored);
        }

        #[test]
        fn test_circular_buffer_size(capacity in 1usize..100, values: Vec<i32>) {
            let mut buffer = CircularBuffer::new(capacity);
            for value in values.iter() {
                buffer.push(*value);
            }
            assert!(buffer.iter().count() <= capacity);
        }
    }
}
```

## Summary

This state management architecture provides:

1. **Centralized State**: Single source of truth
2. **Event-Driven Updates**: Predictable state transitions
3. **Persistence**: Cross-session state preservation
4. **Performance**: Optimized updates and rendering
5. **Testability**: Easy to test and debug
6. **Type Safety**: Compile-time guarantees

The design balances simplicity with power, making it easy to add new features while maintaining predictability and performance.

---

**Document Version**: 1.0.0
**Last Updated**: 2025-11-11
