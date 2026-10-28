use chrono::{Datelike, Local, NaiveDate};
use freya::prelude::*;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

type Rgb = (u8, u8, u8);

#[derive(Clone, Copy, PartialEq)]
struct Palette {
    base: Rgb,
    surface0: Rgb,
    surface1: Rgb,
    surface2: Rgb,
    text: Rgb,
    blue: Rgb,
    red: Rgb,
    pink: Rgb,
    overlay0: Rgb,
}

// Catppuccin Mocha
const DARK: Palette = Palette {
    base: (0x1e, 0x1e, 0x2e),
    surface0: (0x31, 0x32, 0x44),
    surface1: (0x45, 0x47, 0x5a),
    surface2: (0x58, 0x5b, 0x70),
    text: (0xcd, 0xd6, 0xf4),
    blue: (0x89, 0xb4, 0xfa),
    red: (0xf3, 0x8b, 0xa8),
    pink: (0xf5, 0xc2, 0xe7),
    overlay0: (0x6c, 0x70, 0x86),
};

// Catppuccin Latte
const LIGHT: Palette = Palette {
    base: (0xef, 0xf1, 0xf5),
    surface0: (0xcc, 0xd0, 0xda),
    surface1: (0xbc, 0xc0, 0xcc),
    surface2: (0xac, 0xb0, 0xbe),
    text: (0x4c, 0x4f, 0x69),
    blue: (0x1e, 0x66, 0xf5),
    red: (0xd2, 0x0f, 0x39),
    pink: (0xea, 0x76, 0xcb),
    overlay0: (0x9c, 0xa0, 0xb0),
};

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Simple Calendar")
                .with_size(440., 520.)
                .with_background(DARK.base),
        ),
    )
}

fn app() -> impl IntoElement {
    let p = match *Platform::get().current_window().preferred_theme.read() {
        PreferredTheme::Dark => DARK,
        PreferredTheme::Light => LIGHT,
    };
    let now = Local::now();
    let mut year = use_state(|| now.year());
    let mut month = use_state(|| now.month());
    let mut year_text = use_state(|| now.year().to_string());
    let selected = use_state(|| None::<NaiveDate>);
    let mut month_text = use_state(|| MONTH_NAMES[now.month0() as usize].to_string());

    let mut set_view = move |new_year: i32, new_month: u32| {
        year.set(new_year);
        month.set(new_month);
        year_text.set(new_year.to_string());
        month_text.set(MONTH_NAMES[(new_month - 1) as usize].to_string());
    };

    let mut prev_month = move || {
        if month() == 1 {
            set_view(year() - 1, 12)
        } else {
            set_view(year(), month() - 1)
        }
    };
    let mut next_month = move || {
        if month() == 12 {
            set_view(year() + 1, 1)
        } else {
            set_view(year(), month() + 1)
        }
    };
    let mut go_to_today = move || {
        let now = Local::now();
        set_view(now.year(), now.month())
    };
    let mut prev_year = move || set_view(year() - 1, month());
    let mut next_year = move || set_view(year() + 1, month());

    let on_year_submit = move |text: String| {
        match text.trim().parse::<i32>() {
            Ok(y) if y > 0 && y < 10000 => set_view(y, month()),
            _ => set_view(year(), month()),
        }
    };
    let on_month_submit = move |text: String| {
        let text = text.trim().to_lowercase();
        let parsed = text
            .parse::<u32>()
            .ok()
            .filter(|m| (1..=12).contains(m))
            .or_else(|| {
                MONTH_NAMES
                    .iter()
                    .position(|name| name.to_lowercase() == text)
                    .map(|idx| idx as u32 + 1)
            });
        set_view(year(), parsed.unwrap_or(month()));
    };

    rect()
        .expanded()
        .background(p.base)
        .vertical()
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .spacing(8.)
        .padding(12.)
        .on_global_key_down(move |e: Event<KeyboardEventData>| match e.key {
            Key::Named(NamedKey::ArrowLeft) => prev_month(),
            Key::Named(NamedKey::ArrowRight) => next_month(),
            Key::Named(NamedKey::ArrowUp) => next_year(),
            Key::Named(NamedKey::ArrowDown) => prev_year(),
            Key::Named(NamedKey::Home) => go_to_today(),
            Key::Character(ref c) if c == "t" => go_to_today(),
            _ => {}
        })
        .child(
            nav_row(8.)
                .child(nav_button(p, "‹", p.surface0, p.surface1, 20., move || prev_year()))
                .child(
                    nav_input(p, year_text, 80., p.surface0, p.surface2).on_submit(on_year_submit),
                )
                .child(nav_button(p, "›", p.surface0, p.surface1, 20., move || next_year())),
        )
        .child(
            nav_row(16.)
                .child(nav_button(p, "‹", p.surface1, p.surface2, 18., move || prev_month()))
                .child(
                    nav_input(p, month_text, 120., p.surface1, p.surface2).on_submit(on_month_submit),
                )
                .child(nav_button(p, "›", p.surface1, p.surface2, 18., move || next_month())),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .children(DAY_NAMES.iter().map(|name| -> Element {
                    rect()
                        .width(Size::px(48.))
                        .height(Size::px(32.))
                        .center()
                        .child(
                            label()
                                .text(*name)
                                .font_size(12.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .font_family("monospace")
                                .color(p.blue),
                        )
                        .into()
                })),
        )
        .child(calendar_grid(p, year(), month(), selected))
        .child(
            nav_row(12.)
                .child(
                    nav_button(p, "Today", p.surface0, p.surface1, 14., move || go_to_today())
                        .width(Size::px(64.))
                        .height(Size::px(28.)),
                )
                .child(
                    label()
                        .text(match selected() {
                            Some(date) => date.format("%A, %B %-d, %Y").to_string(),
                            None => "←→ Months  ↑↓ Years  T Today".to_string(),
                        })
                        .font_size(15.)
                        .font_family("monospace")
                        .color(p.overlay0),
                ),
        )
}

fn nav_row(spacing: f32) -> Rect {
    rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .spacing(spacing)
}

fn nav_button(
    p: Palette,
    glyph: &'static str,
    bg: Rgb,
    border: Rgb,
    size: f32,
    mut on_press: impl FnMut() + 'static,
) -> Rect {
    rect()
        .width(Size::px(40.))
        .height(Size::px(36.))
        .center()
        .background(bg)
        .corner_radius(6.)
        .border(Border::new().fill(border).width(1.))
        .on_press(move |_| on_press())
        .child(label().text(glyph).font_size(size).color(p.blue))
}

fn nav_input(p: Palette, value: State<String>, width: f32, bg: Rgb, border: Rgb) -> Input {
    Input::new(value)
        .width(Size::px(width))
        .text_align(TextAlign::Center)
        .theme_colors(InputColorsThemePartial {
            background: Some(Color::from(bg).into()),
            focus_background: Some(Color::from(bg).into()),
            border_fill: Some(Color::from(border).into()),
            focus_border_fill: Some(Color::from(p.blue).into()),
            color: Some(Color::from(p.text).into()),
            ..Default::default()
        })
}

fn calendar_grid(
    p: Palette,
    year: i32,
    month: u32,
    mut selected: State<Option<NaiveDate>>,
) -> Rect {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let start = first.weekday().num_days_from_sunday();
    let days = days_in_month(year, month);
    let today = Local::now();
    let today_day = (year == today.year() && month == today.month()).then(|| today.day());

    // 7 rows x 7 columns, like the original layout.
    rect().vertical().spacing(2.).children((0..7u32).map(move |row| -> Element {
        rect()
            .horizontal()
            .spacing(8.)
            .children((0..7u32).map(move |col| -> Element {
                let idx = row * 7 + col;
                let day = idx
                    .checked_sub(start)
                    .map(|d| d + 1)
                    .filter(|d| *d <= days);
                let is_today = day.is_some() && day == today_day;
                let date = day.and_then(|d| NaiveDate::from_ymd_opt(year, month, d));
                let is_selected = date.is_some() && date == selected();
                day_cell(p, day, is_today, is_selected)
                    .on_press(move |_| {
                        if let Some(date) = date {
                            // clicking the selected day again clears the selection
                            selected.set((selected() != Some(date)).then_some(date));
                        }
                    })
                    .into()
            }))
            .into()
    }))
}

fn day_cell(p: Palette, day: Option<u32>, is_today: bool, is_selected: bool) -> Rect {
    let cell = rect()
        .width(Size::px(48.))
        .height(Size::px(40.))
        .center()
        .corner_radius(8.);
    let Some(day) = day else { return cell };
    let cell = if is_today {
        cell.background(p.red)
            .border(Border::new().fill(p.pink).width(2.))
    } else if is_selected {
        cell.background(p.surface1)
            .border(Border::new().fill(p.blue).width(2.))
    } else {
        cell.background(p.surface0)
    };
    cell.child(
        label()
            .text(day.to_string())
            .font_size(14.)
            .font_family("monospace")
            .font_weight(if is_today {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            })
            .color(if is_today { p.base } else { p.text }),
    )
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day()
}
