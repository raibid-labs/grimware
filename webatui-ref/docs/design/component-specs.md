# Component Specifications

## Overview

This document provides detailed specifications for all reusable components in the webatui reference application.

## Core Widget Specifications

### 1. Chart Widget

#### Purpose
Render time-series data visualizations in TUI style.

#### API
```rust
pub struct ChartWidget {
    data: Vec<(f64, f64)>,
    chart_type: ChartType,
    style: ChartStyle,
    labels: ChartLabels,
    bounds: Option<(f64, f64, f64, f64)>,
}

impl ChartWidget {
    pub fn new(chart_type: ChartType) -> Self;
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self;
    pub fn style(mut self, style: ChartStyle) -> Self;
    pub fn labels(mut self, labels: ChartLabels) -> Self;
    pub fn bounds(mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self;
}

pub enum ChartType {
    Line { smooth: bool },
    Bar { width: u16 },
    Scatter { marker: char },
    Sparkline,
}
```

#### Features
- Automatic axis scaling
- Grid lines (optional)
- Multiple datasets
- Legend support
- Interactive hover (WASM only)
- Data point labels
- Zoom/pan controls

#### Usage Example
```rust
let data = vec![
    (0.0, 10.0), (1.0, 20.0), (2.0, 15.0), (3.0, 25.0)
];

let chart = ChartWidget::new(ChartType::Line { smooth: true })
    .data(data)
    .style(ChartStyle::default().color(Color::Cyan))
    .labels(ChartLabels {
        title: "CPU Usage".into(),
        x_axis: "Time (s)".into(),
        y_axis: "Usage (%)".into(),
    });

chart.render(frame, area);
```

### 2. Menu Widget

#### Purpose
Provide hierarchical navigation with keyboard and mouse support.

#### API
```rust
pub struct MenuWidget {
    items: Vec<MenuItem>,
    selected: usize,
    orientation: Orientation,
    style: MenuStyle,
}

pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<KeyCode>,
    pub action: MenuAction,
    pub enabled: bool,
    pub icon: Option<char>,
}

pub enum MenuAction {
    Navigate(ScreenType),
    Execute(Box<dyn Fn() -> Result<()>>),
    SubMenu(Vec<MenuItem>),
}

impl MenuWidget {
    pub fn horizontal() -> Self;
    pub fn vertical() -> Self;
    pub fn add_item(&mut self, item: MenuItem);
    pub fn select_next(&mut self);
    pub fn select_prev(&mut self);
    pub fn activate_selected(&self) -> Option<MenuAction>;
}
```

#### Features
- Keyboard navigation (arrows, tab)
- Mouse click support
- Keyboard shortcuts (configurable)
- Visual selection indicator
- Disabled state rendering
- Icons/symbols
- Nested submenus

#### Usage Example
```rust
let mut menu = MenuWidget::horizontal()
    .style(MenuStyle::default());

menu.add_item(MenuItem {
    label: "Dashboard".into(),
    shortcut: Some(KeyCode::Char('1')),
    action: MenuAction::Navigate(ScreenType::Dashboard),
    enabled: true,
    icon: Some('📊'),
});

menu.add_item(MenuItem {
    label: "Settings".into(),
    shortcut: Some(KeyCode::Char('2')),
    action: MenuAction::Navigate(ScreenType::Settings),
    enabled: true,
    icon: Some('⚙'),
});

menu.render(frame, area);
```

### 3. Table Widget

#### Purpose
Display tabular data with sorting, filtering, and selection.

#### API
```rust
pub struct TableWidget<T> {
    columns: Vec<Column>,
    rows: Vec<T>,
    selected: Option<usize>,
    sort_column: usize,
    sort_order: SortOrder,
    filter: Option<Box<dyn Fn(&T) -> bool>>,
    style: TableStyle,
}

pub struct Column {
    pub header: String,
    pub width: Constraint,
    pub alignment: Alignment,
    pub formatter: Box<dyn Fn(&dyn Any) -> String>,
}

impl<T> TableWidget<T> {
    pub fn new(columns: Vec<Column>) -> Self;
    pub fn rows(mut self, rows: Vec<T>) -> Self;
    pub fn sort_by(&mut self, column: usize, order: SortOrder);
    pub fn filter(mut self, predicate: Box<dyn Fn(&T) -> bool>) -> Self;
    pub fn select(&mut self, index: usize);
    pub fn selected(&self) -> Option<&T>;
}
```

#### Features
- Column sorting (ascending/descending)
- Row selection
- Data filtering
- Custom cell formatters
- Fixed header row
- Scrolling for large datasets
- Row highlighting
- Pagination support

#### Usage Example
```rust
#[derive(Debug)]
struct Process {
    pid: u32,
    name: String,
    cpu: f64,
    memory: u64,
}

let columns = vec![
    Column {
        header: "PID".into(),
        width: Constraint::Length(8),
        alignment: Alignment::Right,
        formatter: Box::new(|any| {
            any.downcast_ref::<u32>().unwrap().to_string()
        }),
    },
    Column {
        header: "Name".into(),
        width: Constraint::Min(20),
        alignment: Alignment::Left,
        formatter: Box::new(|any| {
            any.downcast_ref::<String>().unwrap().clone()
        }),
    },
    // ... more columns
];

let table = TableWidget::new(columns)
    .rows(processes)
    .sort_by(2, SortOrder::Descending); // Sort by CPU

table.render(frame, area);
```

### 4. Gauge Widget

#### Purpose
Display progress or percentage values visually.

#### API
```rust
pub struct GaugeWidget {
    value: f64,
    max: f64,
    label: String,
    style: GaugeStyle,
    gauge_type: GaugeType,
}

pub enum GaugeType {
    Horizontal,
    Vertical,
    Circular,
}

impl GaugeWidget {
    pub fn new(value: f64, max: f64) -> Self;
    pub fn label(mut self, label: impl Into<String>) -> Self;
    pub fn gauge_type(mut self, gauge_type: GaugeType) -> Self;
    pub fn style(mut self, style: GaugeStyle) -> Self;
}
```

#### Features
- Multiple orientations
- Percentage display
- Color gradients (green -> yellow -> red)
- Threshold markers
- Animated transitions
- Custom fill characters

#### Usage Example
```rust
let cpu_gauge = GaugeWidget::new(cpu_usage, 100.0)
    .label("CPU Usage")
    .gauge_type(GaugeType::Horizontal)
    .style(GaugeStyle {
        filled: "█",
        empty: "░",
        color: Color::Cyan,
    });

cpu_gauge.render(frame, area);
```

### 5. Sparkline Widget

#### Purpose
Compact inline data visualization.

#### API
```rust
pub struct SparklineWidget {
    data: Vec<u64>,
    max: Option<u64>,
    style: Style,
}

impl SparklineWidget {
    pub fn new(data: Vec<u64>) -> Self;
    pub fn max(mut self, max: u64) -> Self;
    pub fn style(mut self, style: Style) -> Self;
}
```

#### Features
- Automatic scaling
- Multiple bars in minimal space
- Custom character set
- Color coding
- Trend indicators

#### Usage Example
```rust
let network_sparkline = SparklineWidget::new(network_data)
    .max(1000)
    .style(Style::default().fg(Color::Green));

network_sparkline.render(frame, area);
```

### 6. Input Widget

#### Purpose
Text input with validation and suggestions.

#### API
```rust
pub struct InputWidget {
    value: String,
    placeholder: String,
    cursor_pos: usize,
    validator: Option<Box<dyn Fn(&str) -> bool>>,
    suggestions: Vec<String>,
    input_type: InputType,
    style: InputStyle,
}

pub enum InputType {
    Text,
    Number,
    Email,
    Password,
}

impl InputWidget {
    pub fn new() -> Self;
    pub fn placeholder(mut self, text: impl Into<String>) -> Self;
    pub fn input_type(mut self, input_type: InputType) -> Self;
    pub fn validator(mut self, validator: Box<dyn Fn(&str) -> bool>) -> Self;
    pub fn handle_key(&mut self, key: KeyCode);
    pub fn value(&self) -> &str;
}
```

#### Features
- Text editing (insert, delete, cursor movement)
- Input validation
- Autocomplete suggestions
- Type-specific keyboards (WASM mobile)
- Placeholder text
- Password masking
- Max length limits

#### Usage Example
```rust
let mut email_input = InputWidget::new()
    .placeholder("Enter email address")
    .input_type(InputType::Email)
    .validator(Box::new(|s| s.contains('@')));

// Handle input
if let Event::Key(key) = event {
    email_input.handle_key(key.code);
}

email_input.render(frame, area);
```

## Layout Components

### 1. Panel Component

#### Purpose
Container for grouping related widgets with borders and titles.

#### API
```rust
pub struct Panel {
    title: Option<String>,
    border: BorderStyle,
    padding: Padding,
    children: Vec<Box<dyn Widget>>,
}

impl Panel {
    pub fn new() -> Self;
    pub fn title(mut self, title: impl Into<String>) -> Self;
    pub fn border(mut self, style: BorderStyle) -> Self;
    pub fn padding(mut self, padding: Padding) -> Self;
    pub fn add_child(&mut self, widget: Box<dyn Widget>);
}
```

### 2. Split Layout

#### Purpose
Divide screen space horizontally or vertically.

#### API
```rust
pub struct SplitLayout {
    direction: Direction,
    ratio: f32,
    left_child: Box<dyn Widget>,
    right_child: Box<dyn Widget>,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

impl SplitLayout {
    pub fn horizontal(ratio: f32) -> Self;
    pub fn vertical(ratio: f32) -> Self;
    pub fn left(mut self, widget: Box<dyn Widget>) -> Self;
    pub fn right(mut self, widget: Box<dyn Widget>) -> Self;
}
```

### 3. Grid Layout

#### Purpose
Arrange widgets in a responsive grid.

#### API
```rust
pub struct GridLayout {
    columns: usize,
    row_height: Constraint,
    column_gap: u16,
    row_gap: u16,
    children: Vec<GridItem>,
}

pub struct GridItem {
    widget: Box<dyn Widget>,
    span: (usize, usize), // (columns, rows)
}

impl GridLayout {
    pub fn new(columns: usize) -> Self;
    pub fn add_item(&mut self, widget: Box<dyn Widget>, span: (usize, usize));
    pub fn gaps(mut self, column: u16, row: u16) -> Self;
}
```

## Interactive Components

### 1. Button Widget

#### Purpose
Clickable action trigger.

#### API
```rust
pub struct ButtonWidget {
    label: String,
    action: Box<dyn Fn() -> Action>,
    enabled: bool,
    style: ButtonStyle,
    hotkey: Option<KeyCode>,
}

impl ButtonWidget {
    pub fn new(label: impl Into<String>) -> Self;
    pub fn on_click(mut self, callback: Box<dyn Fn() -> Action>) -> Self;
    pub fn hotkey(mut self, key: KeyCode) -> Self;
    pub fn enabled(mut self, enabled: bool) -> Self;
}
```

### 2. Hyperlink Widget

#### Purpose
Clickable link (opens in new tab in WASM).

#### API
```rust
pub struct HyperlinkWidget {
    text: String,
    url: String,
    style: Style,
}

impl HyperlinkWidget {
    pub fn new(text: impl Into<String>, url: impl Into<String>) -> Self;
    pub fn style(mut self, style: Style) -> Self;
}
```

### 3. Modal Dialog

#### Purpose
Overlay dialog for confirmations and forms.

#### API
```rust
pub struct ModalDialog {
    title: String,
    content: Box<dyn Widget>,
    buttons: Vec<ButtonWidget>,
    backdrop: bool,
}

impl ModalDialog {
    pub fn new(title: impl Into<String>) -> Self;
    pub fn content(mut self, widget: Box<dyn Widget>) -> Self;
    pub fn add_button(&mut self, button: ButtonWidget);
    pub fn show(&mut self);
    pub fn close(&mut self);
}
```

## Style System

### Base Style
```rust
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifiers: Modifiers,
}

pub struct Modifiers {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub reversed: bool,
}
```

### Theme System
```rust
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub styles: StyleMap,
}

pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub foreground: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
}

pub type StyleMap = HashMap<String, Style>;
```

## Component Lifecycle

### Initialization
```rust
trait Widget {
    fn on_mount(&mut self) {
        // Called when widget is first added to screen
    }

    fn on_unmount(&mut self) {
        // Called when widget is removed from screen
    }
}
```

### Updates
```rust
trait Widget {
    fn should_update(&self, event: &Event) -> bool {
        // Return true if widget needs re-render
        true
    }

    fn update(&mut self, event: Event) {
        // Handle state changes
    }
}
```

### Rendering
```rust
trait Widget {
    fn render(&self, frame: &mut Frame, area: Rect);
}
```

## Testing Components

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_data_bounds() {
        let chart = ChartWidget::new(ChartType::Line { smooth: false })
            .data(vec![(0.0, 10.0), (1.0, 20.0)]);

        let bounds = chart.calculate_bounds();
        assert_eq!(bounds, (0.0, 1.0, 10.0, 20.0));
    }
}
```

### Visual Regression Tests
```rust
#[test]
fn test_menu_rendering() {
    let menu = MenuWidget::vertical()
        .add_item(MenuItem::new("Item 1"))
        .add_item(MenuItem::new("Item 2"));

    let output = render_to_string(&menu, Rect::new(0, 0, 20, 10));
    assert_snapshot!(output);
}
```

## Performance Considerations

1. **Lazy Rendering**: Only render visible portions
2. **Caching**: Cache computed layouts
3. **Dirty Tracking**: Only re-render changed components
4. **Virtualization**: For large lists/tables
5. **Batch Updates**: Group state changes

## Accessibility

1. **Keyboard Navigation**: All components keyboard accessible
2. **Screen Reader**: ARIA labels (WASM)
3. **Focus Management**: Clear focus indicators
4. **Color Contrast**: Minimum 4.5:1 ratio
5. **Text Alternatives**: For visual elements

---

**Document Version**: 1.0.0
**Last Updated**: 2025-11-11
