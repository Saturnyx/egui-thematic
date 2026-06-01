use egui::{Color32, Visuals};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct ThemeConfig {
    pub name: String,
    pub dark_mode: bool,

    pub override_text_color: Option<[u8; 4]>,
    pub override_weak_text_color: Option<[u8; 4]>,
    pub override_hyperlink_color: Option<[u8; 4]>,
    pub override_faint_bg_color: Option<[u8; 4]>,
    pub override_extreme_bg_color: Option<[u8; 4]>,
    pub override_code_bg_color: Option<[u8; 4]>,
    pub override_warn_fg_color: Option<[u8; 4]>,
    pub override_error_fg_color: Option<[u8; 4]>,

    pub override_window_fill: Option<[u8; 4]>,
    pub override_window_stroke_color: Option<[u8; 4]>,
    pub override_window_stroke_width: Option<f32>,
    pub override_window_corner_radius: Option<u8>,
    pub override_window_shadow_size: Option<u8>,

    pub override_panel_fill: Option<[u8; 4]>,

    pub override_popup_shadow_size: Option<u8>,

    pub override_selection_bg: Option<[u8; 4]>,
    pub override_selection_stroke_color: Option<[u8; 4]>,
    pub override_selection_stroke_width: Option<f32>,

    pub override_widget_noninteractive_bg_fill: Option<[u8; 4]>,
    pub override_widget_noninteractive_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_noninteractive_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_noninteractive_bg_stroke_width: Option<f32>,
    pub override_widget_noninteractive_corner_radius: Option<u8>,
    pub override_widget_noninteractive_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_noninteractive_fg_stroke_width: Option<f32>,
    pub override_widget_noninteractive_expansion: Option<f32>,

    pub override_widget_inactive_bg_fill: Option<[u8; 4]>,
    pub override_widget_inactive_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_inactive_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_inactive_bg_stroke_width: Option<f32>,
    pub override_widget_inactive_corner_radius: Option<u8>,
    pub override_widget_inactive_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_inactive_fg_stroke_width: Option<f32>,
    pub override_widget_inactive_expansion: Option<f32>,

    pub override_widget_hovered_bg_fill: Option<[u8; 4]>,
    pub override_widget_hovered_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_hovered_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_hovered_bg_stroke_width: Option<f32>,
    pub override_widget_hovered_corner_radius: Option<u8>,
    pub override_widget_hovered_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_hovered_fg_stroke_width: Option<f32>,
    pub override_widget_hovered_expansion: Option<f32>,

    pub override_widget_active_bg_fill: Option<[u8; 4]>,
    pub override_widget_active_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_active_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_active_bg_stroke_width: Option<f32>,
    pub override_widget_active_corner_radius: Option<u8>,
    pub override_widget_active_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_active_fg_stroke_width: Option<f32>,
    pub override_widget_active_expansion: Option<f32>,

    pub override_widget_open_bg_fill: Option<[u8; 4]>,
    pub override_widget_open_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_open_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_open_bg_stroke_width: Option<f32>,
    pub override_widget_open_corner_radius: Option<u8>,
    pub override_widget_open_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_open_fg_stroke_width: Option<f32>,
    pub override_widget_open_expansion: Option<f32>,

    pub override_resize_corner_size: Option<f32>,
    pub override_text_cursor_width: Option<f32>,
    pub override_clip_rect_margin: Option<f32>,
    pub override_button_frame: Option<bool>,
    pub override_collapsing_header_frame: Option<bool>,
    pub override_indent_has_left_vline: Option<bool>,
    pub override_striped: Option<bool>,
    pub override_slider_trailing_fill: Option<bool>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            override_text_color: None,
            override_weak_text_color: None,
            override_hyperlink_color: None,
            override_faint_bg_color: None,
            override_extreme_bg_color: None,
            override_code_bg_color: None,
            override_warn_fg_color: None,
            override_error_fg_color: None,
            override_window_fill: None,
            override_window_stroke_color: None,
            override_window_stroke_width: None,
            override_window_corner_radius: None,
            override_window_shadow_size: None,
            override_panel_fill: None,
            override_popup_shadow_size: None,
            override_selection_bg: None,
            override_selection_stroke_color: None,
            override_selection_stroke_width: None,
            override_widget_noninteractive_bg_fill: None,
            override_widget_noninteractive_weak_bg_fill: None,
            override_widget_noninteractive_bg_stroke_color: None,
            override_widget_noninteractive_bg_stroke_width: None,
            override_widget_noninteractive_corner_radius: None,
            override_widget_noninteractive_fg_stroke_color: None,
            override_widget_noninteractive_fg_stroke_width: None,
            override_widget_noninteractive_expansion: None,
            override_widget_inactive_bg_fill: None,
            override_widget_inactive_weak_bg_fill: None,
            override_widget_inactive_bg_stroke_color: None,
            override_widget_inactive_bg_stroke_width: None,
            override_widget_inactive_corner_radius: None,
            override_widget_inactive_fg_stroke_color: None,
            override_widget_inactive_fg_stroke_width: None,
            override_widget_inactive_expansion: None,
            override_widget_hovered_bg_fill: None,
            override_widget_hovered_weak_bg_fill: None,
            override_widget_hovered_bg_stroke_color: None,
            override_widget_hovered_bg_stroke_width: None,
            override_widget_hovered_corner_radius: None,
            override_widget_hovered_fg_stroke_color: None,
            override_widget_hovered_fg_stroke_width: None,
            override_widget_hovered_expansion: None,
            override_widget_active_bg_fill: None,
            override_widget_active_weak_bg_fill: None,
            override_widget_active_bg_stroke_color: None,
            override_widget_active_bg_stroke_width: None,
            override_widget_active_corner_radius: None,
            override_widget_active_fg_stroke_color: None,
            override_widget_active_fg_stroke_width: None,
            override_widget_active_expansion: None,
            override_widget_open_bg_fill: None,
            override_widget_open_weak_bg_fill: None,
            override_widget_open_bg_stroke_color: None,
            override_widget_open_bg_stroke_width: None,
            override_widget_open_corner_radius: None,
            override_widget_open_fg_stroke_color: None,
            override_widget_open_fg_stroke_width: None,
            override_widget_open_expansion: None,
            override_resize_corner_size: None,
            override_text_cursor_width: None,
            override_clip_rect_margin: None,
            override_button_frame: None,
            override_collapsing_header_frame: None,
            override_indent_has_left_vline: None,
            override_striped: None,
            override_slider_trailing_fill: None,
        }
    }
}

impl ThemeConfig {
    pub fn dark_preset() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            ..Default::default()
        }
    }

    pub fn light_preset() -> Self {
        Self {
            name: "Light".to_string(),
            dark_mode: false,
            ..Default::default()
        }
    }

    // ── Dracula ─────────────────────────────────────────────────────────────
    // Palette: Background #282a36, Current Line #44475a, Foreground #f8f8f2,
    //          Comment #6272a4, Cyan #8be9fd, Purple #bd93f9, Pink #ff79c6,
    //          Red #ff5555, Orange #ffb86c, Yellow #f1fa8c, Green #50fa7b
    pub fn dracula_preset() -> Self {
        Self {
            name: "Dracula".to_string(),
            dark_mode: true,

            override_text_color: Some([248, 248, 242, 255]), // Foreground
            override_weak_text_color: Some([98, 114, 164, 255]), // Comment
            override_hyperlink_color: Some([139, 233, 253, 255]), // Cyan
            override_faint_bg_color: Some([68, 71, 90, 255]), // Current Line
            override_extreme_bg_color: Some([21, 22, 30, 255]), // darker than bg
            override_code_bg_color: Some([68, 71, 90, 255]), // Current Line
            override_warn_fg_color: Some([241, 250, 140, 255]), // Yellow
            override_error_fg_color: Some([255, 85, 85, 255]), // Red

            override_window_fill: Some([40, 42, 54, 255]), // Background
            override_window_stroke_color: Some([68, 71, 90, 255]), // Current Line
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(6),
            override_window_shadow_size: Some(20),

            override_panel_fill: Some([68, 71, 90, 255]), // Current Line
            override_popup_shadow_size: Some(12),

            override_selection_bg: Some([98, 114, 164, 255]), // Comment as selection
            override_selection_stroke_color: Some([189, 147, 249, 255]), // Purple
            override_selection_stroke_width: Some(0.0),

            // noninteractive: labels, separators — essentially invisible bg
            override_widget_noninteractive_bg_fill: Some([40, 42, 54, 255]), // Background
            override_widget_noninteractive_weak_bg_fill: Some([50, 52, 66, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([68, 71, 90, 255]), // Current Line
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(4),
            override_widget_noninteractive_fg_stroke_color: Some([98, 114, 164, 255]), // Comment (muted)
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive: buttons/sliders at rest
            override_widget_inactive_bg_fill: Some([68, 71, 90, 255]), // Current Line
            override_widget_inactive_weak_bg_fill: Some([57, 59, 78, 255]),
            override_widget_inactive_bg_stroke_color: Some([98, 114, 164, 120]), // Comment, semi
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(4),
            override_widget_inactive_fg_stroke_color: Some([248, 248, 242, 255]), // Foreground
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([90, 93, 118, 255]),
            override_widget_hovered_weak_bg_fill: Some([80, 83, 106, 255]),
            override_widget_hovered_bg_stroke_color: Some([189, 147, 249, 255]), // Purple accent
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(4),
            override_widget_hovered_fg_stroke_color: Some([248, 248, 242, 255]),
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active (pressed)
            override_widget_active_bg_fill: Some([98, 114, 164, 255]), // Comment/selection
            override_widget_active_weak_bg_fill: Some([80, 96, 142, 255]),
            override_widget_active_bg_stroke_color: Some([189, 147, 249, 255]), // Purple
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(4),
            override_widget_active_fg_stroke_color: Some([248, 248, 242, 255]),
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open (combo boxes, menus)
            override_widget_open_bg_fill: Some([98, 114, 164, 255]),
            override_widget_open_weak_bg_fill: Some([68, 71, 90, 255]),
            override_widget_open_bg_stroke_color: Some([189, 147, 249, 255]),
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(4),
            override_widget_open_fg_stroke_color: Some([248, 248, 242, 255]),
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Nord ─────────────────────────────────────────────────────────────────
    // Polar Night: nord0 #2e3440, nord1 #3b4252, nord2 #434c5e, nord3 #4c566a
    // Snow Storm:  nord4 #d8dee9, nord5 #e5e9f0, nord6 #eceff4
    // Frost:       nord7 #8fbcbb, nord8 #88c0d0, nord9 #81a1c1, nord10 #5e81ac
    // Aurora:      nord11 #bf616a, nord12 #d08770, nord13 #ebcb8b, nord14 #a3be8c, nord15 #b48ead
    pub fn nord_preset() -> Self {
        Self {
            name: "Nord".to_string(),
            dark_mode: true,

            override_text_color: Some([216, 222, 233, 255]), // nord4
            override_weak_text_color: Some([76, 86, 106, 255]), // nord3
            override_hyperlink_color: Some([136, 192, 208, 255]), // nord8 (primary frost)
            override_faint_bg_color: Some([59, 66, 82, 255]), // nord1
            override_extreme_bg_color: Some([29, 33, 42, 255]), // darker than nord0
            override_code_bg_color: Some([59, 66, 82, 255]), // nord1
            override_warn_fg_color: Some([235, 203, 139, 255]), // nord13
            override_error_fg_color: Some([191, 97, 106, 255]), // nord11

            override_window_fill: Some([46, 52, 64, 255]), // nord0
            override_window_stroke_color: Some([67, 76, 94, 255]), // nord2
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(4),
            override_window_shadow_size: Some(16),

            override_panel_fill: Some([59, 66, 82, 255]), // nord1
            override_popup_shadow_size: Some(10),

            override_selection_bg: Some([136, 192, 208, 255]), // nord8
            override_selection_stroke_color: Some([143, 188, 187, 255]), // nord7
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([46, 52, 64, 255]), // nord0
            override_widget_noninteractive_weak_bg_fill: Some([52, 58, 72, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([67, 76, 94, 255]), // nord2
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(4),
            override_widget_noninteractive_fg_stroke_color: Some([76, 86, 106, 255]), // nord3
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([59, 66, 82, 255]), // nord1
            override_widget_inactive_weak_bg_fill: Some([52, 59, 74, 255]),
            override_widget_inactive_bg_stroke_color: Some([76, 86, 106, 255]), // nord3
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(4),
            override_widget_inactive_fg_stroke_color: Some([216, 222, 233, 255]), // nord4
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([67, 76, 94, 255]), // nord2
            override_widget_hovered_weak_bg_fill: Some([63, 72, 89, 255]),
            override_widget_hovered_bg_stroke_color: Some([136, 192, 208, 255]), // nord8
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(4),
            override_widget_hovered_fg_stroke_color: Some([229, 233, 240, 255]), // nord5
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active
            override_widget_active_bg_fill: Some([129, 161, 193, 255]), // nord9
            override_widget_active_weak_bg_fill: Some([94, 129, 172, 255]), // nord10
            override_widget_active_bg_stroke_color: Some([136, 192, 208, 255]), // nord8
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(4),
            override_widget_active_fg_stroke_color: Some([236, 239, 244, 255]), // nord6
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([94, 129, 172, 255]), // nord10
            override_widget_open_weak_bg_fill: Some([59, 66, 82, 255]), // nord1
            override_widget_open_bg_stroke_color: Some([136, 192, 208, 255]), // nord8
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(4),
            override_widget_open_fg_stroke_color: Some([229, 233, 240, 255]), // nord5
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Gruvbox Dark ─────────────────────────────────────────────────────────
    // bg0_hard #1d2021, bg0 #282828, bg1 #3c3836, bg2 #504945, bg3 #665c54, bg4 #7c6f64
    // gray #928374, fg0 #fbf1c7, fg1 #ebdbb2, fg2 #d5c4a1, fg4 #a89984
    // bright: red #fb4934, green #b8bb26, yellow #fabd2f, blue #83a598,
    //         purple #d3869b, aqua #8ec07c, orange #fe8019
    // neutral: orange #d65d0e
    pub fn gruvbox_dark_preset() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            dark_mode: true,

            override_text_color: Some([235, 219, 178, 255]), // fg1 #ebdbb2
            override_weak_text_color: Some([146, 131, 116, 255]), // gray #928374
            override_hyperlink_color: Some([131, 165, 152, 255]), // bright_blue #83a598
            override_faint_bg_color: Some([60, 56, 54, 255]), // bg1 #3c3836
            override_extreme_bg_color: Some([29, 32, 33, 255]), // bg0_hard #1d2021
            override_code_bg_color: Some([60, 56, 54, 255]), // bg1
            override_warn_fg_color: Some([250, 189, 47, 255]), // bright_yellow #fabd2f
            override_error_fg_color: Some([251, 73, 52, 255]), // bright_red #fb4934

            override_window_fill: Some([40, 40, 40, 255]), // bg0 #282828
            override_window_stroke_color: Some([80, 73, 69, 255]), // bg2 #504945
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(3), // Gruvbox is intentionally boxy
            override_window_shadow_size: Some(16),

            override_panel_fill: Some([60, 56, 54, 255]), // bg1 #3c3836
            override_popup_shadow_size: Some(10),

            override_selection_bg: Some([102, 92, 84, 255]), // bg3 #665c54
            override_selection_stroke_color: Some([254, 128, 25, 255]), // bright_orange
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([40, 40, 40, 255]), // bg0
            override_widget_noninteractive_weak_bg_fill: Some([50, 47, 46, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([60, 56, 54, 255]), // bg1
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(3),
            override_widget_noninteractive_fg_stroke_color: Some([146, 131, 116, 255]), // gray
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([60, 56, 54, 255]), // bg1
            override_widget_inactive_weak_bg_fill: Some([50, 47, 46, 255]),
            override_widget_inactive_bg_stroke_color: Some([80, 73, 69, 255]), // bg2
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(3),
            override_widget_inactive_fg_stroke_color: Some([235, 219, 178, 255]), // fg1
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([80, 73, 69, 255]), // bg2
            override_widget_hovered_weak_bg_fill: Some([70, 64, 61, 255]),
            override_widget_hovered_bg_stroke_color: Some([254, 128, 25, 255]), // bright_orange
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(3),
            override_widget_hovered_fg_stroke_color: Some([251, 241, 199, 255]), // fg0
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — use neutral orange for a punchy pressed state
            override_widget_active_bg_fill: Some([214, 93, 14, 255]), // neutral_orange #d65d0e
            override_widget_active_weak_bg_fill: Some([160, 70, 10, 255]),
            override_widget_active_bg_stroke_color: Some([254, 128, 25, 255]), // bright_orange
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(3),
            override_widget_active_fg_stroke_color: Some([251, 241, 199, 255]), // fg0
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([102, 92, 84, 255]), // bg3
            override_widget_open_weak_bg_fill: Some([80, 73, 69, 255]), // bg2
            override_widget_open_bg_stroke_color: Some([254, 128, 25, 255]), // bright_orange
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(3),
            override_widget_open_fg_stroke_color: Some([251, 241, 199, 255]), // fg0
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_striped: Some(true),
            ..Default::default()
        }
    }

    // ── Solarized Dark ───────────────────────────────────────────────────────
    // base03 #002b36, base02 #073642, base01 #586e75, base00 #657b83,
    // base0 #839496,  base1  #93a1a1
    // yellow #b58900, orange #cb4b16, red #dc322f, magenta #d33682,
    // violet #6c71c4, blue #268bd2, cyan #2aa198, green #859900
    pub fn solarized_dark_preset() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            dark_mode: true,

            override_text_color: Some([131, 148, 150, 255]), // base0  #839496
            override_weak_text_color: Some([88, 110, 117, 255]), // base01 #586e75
            override_hyperlink_color: Some([42, 161, 152, 255]), // cyan   #2aa198
            override_faint_bg_color: Some([7, 54, 66, 255]), // base02 #073642
            override_extreme_bg_color: Some([0, 22, 29, 255]), // darker than base03
            override_code_bg_color: Some([7, 54, 66, 255]),  // base02
            override_warn_fg_color: Some([181, 137, 0, 255]), // yellow #b58900
            override_error_fg_color: Some([220, 50, 47, 255]), // red    #dc322f

            override_window_fill: Some([0, 43, 54, 255]), // base03 #002b36
            override_window_stroke_color: Some([7, 54, 66, 255]), // base02
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(4),
            override_window_shadow_size: Some(16),

            override_panel_fill: Some([7, 54, 66, 255]), // base02
            override_popup_shadow_size: Some(10),

            override_selection_bg: Some([88, 110, 117, 255]), // base01
            override_selection_stroke_color: Some([38, 139, 210, 255]), // blue #268bd2
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([0, 43, 54, 255]), // base03
            override_widget_noninteractive_weak_bg_fill: Some([4, 48, 59, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([7, 54, 66, 255]), // base02
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(4),
            override_widget_noninteractive_fg_stroke_color: Some([88, 110, 117, 255]), // base01
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([7, 54, 66, 255]), // base02
            override_widget_inactive_weak_bg_fill: Some([4, 48, 59, 255]),
            override_widget_inactive_bg_stroke_color: Some([88, 110, 117, 255]), // base01
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(4),
            override_widget_inactive_fg_stroke_color: Some([131, 148, 150, 255]), // base0
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([15, 70, 83, 255]),
            override_widget_hovered_weak_bg_fill: Some([11, 62, 74, 255]),
            override_widget_hovered_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(4),
            override_widget_hovered_fg_stroke_color: Some([147, 161, 161, 255]), // base1
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — blue is the primary action color
            override_widget_active_bg_fill: Some([38, 139, 210, 255]), // blue #268bd2
            override_widget_active_weak_bg_fill: Some([30, 112, 170, 255]),
            override_widget_active_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(4),
            override_widget_active_fg_stroke_color: Some([0, 43, 54, 255]), // base03 (inverted)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([38, 139, 210, 255]), // blue
            override_widget_open_weak_bg_fill: Some([7, 54, 66, 255]), // base02
            override_widget_open_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(4),
            override_widget_open_fg_stroke_color: Some([147, 161, 161, 255]), // base1
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Solarized Light ──────────────────────────────────────────────────────
    // Inverted roles: bg is now base3 #fdf6e3 / base2 #eee8d5
    // text is base00 #657b83, comments/secondary is base1 #93a1a1
    pub fn solarized_light_preset() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            dark_mode: false,

            override_text_color: Some([101, 123, 131, 255]), // base00 #657b83
            override_weak_text_color: Some([147, 161, 161, 255]), // base1  #93a1a1
            override_hyperlink_color: Some([38, 139, 210, 255]), // blue   #268bd2
            override_faint_bg_color: Some([238, 232, 213, 255]), // base2  #eee8d5
            override_extreme_bg_color: Some([253, 246, 227, 255]), // base3  #fdf6e3
            override_code_bg_color: Some([238, 232, 213, 255]), // base2
            override_warn_fg_color: Some([181, 137, 0, 255]), // yellow
            override_error_fg_color: Some([220, 50, 47, 255]), // red

            override_window_fill: Some([253, 246, 227, 255]), // base3 #fdf6e3
            override_window_stroke_color: Some([210, 204, 188, 255]), // darker base2
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(4),
            override_window_shadow_size: Some(12),

            override_panel_fill: Some([238, 232, 213, 255]), // base2
            override_popup_shadow_size: Some(8),

            override_selection_bg: Some([147, 161, 161, 255]), // base1
            override_selection_stroke_color: Some([38, 139, 210, 255]), // blue
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([253, 246, 227, 255]), // base3
            override_widget_noninteractive_weak_bg_fill: Some([246, 239, 218, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([210, 204, 188, 255]),
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(4),
            override_widget_noninteractive_fg_stroke_color: Some([147, 161, 161, 255]), // base1
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([238, 232, 213, 255]), // base2
            override_widget_inactive_weak_bg_fill: Some([245, 239, 222, 255]),
            override_widget_inactive_bg_stroke_color: Some([210, 204, 188, 255]),
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(4),
            override_widget_inactive_fg_stroke_color: Some([101, 123, 131, 255]), // base00
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([226, 219, 200, 255]),
            override_widget_hovered_weak_bg_fill: Some([232, 226, 207, 255]),
            override_widget_hovered_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(4),
            override_widget_hovered_fg_stroke_color: Some([88, 110, 117, 255]), // base01
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active
            override_widget_active_bg_fill: Some([38, 139, 210, 255]), // blue
            override_widget_active_weak_bg_fill: Some([30, 112, 170, 255]),
            override_widget_active_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(4),
            override_widget_active_fg_stroke_color: Some([253, 246, 227, 255]), // base3 (light on dark)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([38, 139, 210, 255]), // blue
            override_widget_open_weak_bg_fill: Some([238, 232, 213, 255]), // base2
            override_widget_open_bg_stroke_color: Some([42, 161, 152, 255]), // cyan
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(4),
            override_widget_open_fg_stroke_color: Some([88, 110, 117, 255]), // base01
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Monokai ──────────────────────────────────────────────────────────────
    // Background #272822, Foreground #f8f8f2, Comment #75715e,
    // Selection/Line #49483e, Red/Pink #f92672, Orange #fd971f,
    // Yellow #e6db74, Green #a6e22e, Cyan #66d9e8, Purple #ae81ff
    pub fn monokai_preset() -> Self {
        Self {
            name: "Monokai".to_string(),
            dark_mode: true,

            override_text_color: Some([248, 248, 242, 255]), // Foreground #f8f8f2
            override_weak_text_color: Some([117, 113, 94, 255]), // Comment   #75715e
            override_hyperlink_color: Some([102, 217, 239, 255]), // Cyan      #66d9e8 (adjusted)
            override_faint_bg_color: Some([73, 72, 62, 255]), // Selection #49483e
            override_extreme_bg_color: Some([30, 31, 25, 255]), // darker
            override_code_bg_color: Some([73, 72, 62, 255]), // Selection
            override_warn_fg_color: Some([230, 219, 116, 255]), // Yellow    #e6db74
            override_error_fg_color: Some([249, 38, 114, 255]), // Red/Pink  #f92672

            override_window_fill: Some([39, 40, 34, 255]), // Background #272822
            override_window_stroke_color: Some([62, 61, 50, 255]), // line highlight
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(4),
            override_window_shadow_size: Some(16),

            override_panel_fill: Some([73, 72, 62, 255]), // Selection
            override_popup_shadow_size: Some(10),

            override_selection_bg: Some([73, 72, 62, 255]), // #49483e
            override_selection_stroke_color: Some([174, 129, 255, 255]), // Purple #ae81ff
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([39, 40, 34, 255]), // bg
            override_widget_noninteractive_weak_bg_fill: Some([50, 50, 44, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([62, 61, 50, 255]),
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(4),
            override_widget_noninteractive_fg_stroke_color: Some([117, 113, 94, 255]), // Comment
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([62, 61, 50, 255]), // line highlight
            override_widget_inactive_weak_bg_fill: Some([55, 54, 45, 255]),
            override_widget_inactive_bg_stroke_color: Some([117, 113, 94, 120]), // Comment semi
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(4),
            override_widget_inactive_fg_stroke_color: Some([248, 248, 242, 255]), // Foreground
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([86, 84, 71, 255]),
            override_widget_hovered_weak_bg_fill: Some([73, 72, 62, 255]),
            override_widget_hovered_bg_stroke_color: Some([174, 129, 255, 255]), // Purple
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(4),
            override_widget_hovered_fg_stroke_color: Some([248, 248, 242, 255]),
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — orange for a warm pressed state
            override_widget_active_bg_fill: Some([253, 151, 31, 255]), // Orange #fd971f
            override_widget_active_weak_bg_fill: Some([200, 120, 25, 255]),
            override_widget_active_bg_stroke_color: Some([174, 129, 255, 255]), // Purple
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(4),
            override_widget_active_fg_stroke_color: Some([39, 40, 34, 255]), // bg (inverted)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([117, 113, 94, 255]), // Comment
            override_widget_open_weak_bg_fill: Some([73, 72, 62, 255]),
            override_widget_open_bg_stroke_color: Some([174, 129, 255, 255]),
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(4),
            override_widget_open_fg_stroke_color: Some([248, 248, 242, 255]),
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            ..Default::default()
        }
    }

    // ── One Dark ─────────────────────────────────────────────────────────────
    // bg #282c34, sidebar #21252b, gutter #636d83, fg #abb2bf,
    // comment #5c6370, selection #3e4451, highlight #2c313a
    // red #e06c75, orange #d19a66, yellow #e5c07b, green #98c379,
    // cyan #56b6c2, blue #61afef, purple #c678dd
    pub fn one_dark_preset() -> Self {
        Self {
            name: "One Dark".to_string(),
            dark_mode: true,

            override_text_color: Some([171, 178, 191, 255]), // fg      #abb2bf
            override_weak_text_color: Some([92, 99, 112, 255]), // comment #5c6370
            override_hyperlink_color: Some([97, 175, 239, 255]), // blue    #61afef
            override_faint_bg_color: Some([33, 37, 43, 255]), // sidebar #21252b
            override_extreme_bg_color: Some([21, 23, 27, 255]),
            override_code_bg_color: Some([33, 37, 43, 255]), // sidebar
            override_warn_fg_color: Some([229, 192, 123, 255]), // yellow  #e5c07b
            override_error_fg_color: Some([224, 108, 117, 255]), // red     #e06c75

            override_window_fill: Some([40, 44, 52, 255]), // bg #282c34
            override_window_stroke_color: Some([62, 68, 81, 255]), // selection #3e4451
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(6),
            override_window_shadow_size: Some(16),

            override_panel_fill: Some([33, 37, 43, 255]), // sidebar #21252b
            override_popup_shadow_size: Some(10),

            override_selection_bg: Some([61, 66, 77, 255]), // #3d4250
            override_selection_stroke_color: Some([97, 175, 239, 255]), // blue
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([40, 44, 52, 255]), // bg
            override_widget_noninteractive_weak_bg_fill: Some([36, 39, 47, 255]),
            override_widget_noninteractive_bg_stroke_color: Some([62, 68, 81, 255]),
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(6),
            override_widget_noninteractive_fg_stroke_color: Some([92, 99, 112, 255]), // comment
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([44, 49, 58, 255]), // highlight #2c313a
            override_widget_inactive_weak_bg_fill: Some([40, 44, 52, 255]),
            override_widget_inactive_bg_stroke_color: Some([62, 68, 81, 255]),
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(6),
            override_widget_inactive_fg_stroke_color: Some([171, 178, 191, 255]), // fg
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([55, 60, 72, 255]),
            override_widget_hovered_weak_bg_fill: Some([48, 52, 63, 255]),
            override_widget_hovered_bg_stroke_color: Some([97, 175, 239, 255]), // blue
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(6),
            override_widget_hovered_fg_stroke_color: Some([171, 178, 191, 255]),
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — blue primary action
            override_widget_active_bg_fill: Some([97, 175, 239, 255]), // blue
            override_widget_active_weak_bg_fill: Some([75, 140, 196, 255]),
            override_widget_active_bg_stroke_color: Some([86, 182, 194, 255]), // cyan
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(6),
            override_widget_active_fg_stroke_color: Some([40, 44, 52, 255]), // bg (inverted)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([62, 68, 81, 255]),
            override_widget_open_weak_bg_fill: Some([44, 49, 58, 255]),
            override_widget_open_bg_stroke_color: Some([97, 175, 239, 255]),
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(6),
            override_widget_open_fg_stroke_color: Some([171, 178, 191, 255]),
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Tokyo Night ──────────────────────────────────────────────────────────
    // bg #1a1b26, bg_dark #15161e, fg #c0caf5, comment #565f89,
    // line_highlight #292e42, border #3b4261, sidebar #1f2335
    // red #f7768e, orange #ff9e64, yellow #e0af68, green #9ece6a,
    // blue #7aa2f7, purple #bb9af7, cyan #7dcfff
    pub fn tokyo_night_preset() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            dark_mode: true,

            override_text_color: Some([192, 202, 245, 255]), // fg      #c0caf5
            override_weak_text_color: Some([86, 95, 137, 255]), // comment #565f89
            override_hyperlink_color: Some([122, 162, 247, 255]), // blue    #7aa2f7
            override_faint_bg_color: Some([31, 35, 53, 255]), // sidebar #1f2335
            override_extreme_bg_color: Some([16, 17, 28, 255]), // bg_dark derived
            override_code_bg_color: Some([31, 35, 53, 255]), // sidebar
            override_warn_fg_color: Some([224, 175, 104, 255]), // yellow  #e0af68
            override_error_fg_color: Some([247, 118, 142, 255]), // red     #f7768e

            override_window_fill: Some([26, 27, 38, 255]), // bg #1a1b26
            override_window_stroke_color: Some([59, 66, 97, 255]), // border #3b4261
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(8),
            override_window_shadow_size: Some(20),

            override_panel_fill: Some([21, 22, 30, 255]), // bg_dark #15161e
            override_popup_shadow_size: Some(12),

            override_selection_bg: Some([40, 52, 87, 255]), // selection #283457
            override_selection_stroke_color: Some([122, 162, 247, 255]), // blue
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([26, 27, 38, 255]), // bg
            override_widget_noninteractive_weak_bg_fill: Some([31, 35, 53, 255]), // sidebar
            override_widget_noninteractive_bg_stroke_color: Some([59, 66, 97, 255]), // border
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(8),
            override_widget_noninteractive_fg_stroke_color: Some([86, 95, 137, 255]), // comment
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([41, 46, 66, 255]), // line_highlight #292e42
            override_widget_inactive_weak_bg_fill: Some([36, 40, 59, 255]),
            override_widget_inactive_bg_stroke_color: Some([59, 66, 97, 255]), // border
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(8),
            override_widget_inactive_fg_stroke_color: Some([169, 177, 214, 255]), // terminal_white
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([59, 66, 97, 255]), // border
            override_widget_hovered_weak_bg_fill: Some([50, 57, 85, 255]),
            override_widget_hovered_bg_stroke_color: Some([122, 162, 247, 255]), // blue
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(8),
            override_widget_hovered_fg_stroke_color: Some([192, 202, 245, 255]), // fg
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — blue for primary interaction
            override_widget_active_bg_fill: Some([122, 162, 247, 255]), // blue
            override_widget_active_weak_bg_fill: Some([86, 120, 200, 255]),
            override_widget_active_bg_stroke_color: Some([125, 207, 255, 255]), // cyan #7dcfff
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(8),
            override_widget_active_fg_stroke_color: Some([26, 27, 38, 255]), // bg (inverted)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([59, 66, 97, 255]), // border
            override_widget_open_weak_bg_fill: Some([41, 46, 66, 255]), // line_highlight
            override_widget_open_bg_stroke_color: Some([122, 162, 247, 255]), // blue
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(8),
            override_widget_open_fg_stroke_color: Some([192, 202, 245, 255]),
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    // ── Catppuccin Mocha ─────────────────────────────────────────────────────
    // Crust #11111b, Mantle #181825, Base #1e1e2e, Surface0 #313244,
    // Surface1 #45475a, Surface2 #585b70, Overlay0 #6c7086,
    // Overlay1 #7f849c, Overlay2 #9399b2, Subtext0 #a6adc8,
    // Subtext1 #bac2de, Text #cdd6f4
    // Red #f38ba8, Maroon #eba0ac, Peach #fab387, Yellow #f9e2af,
    // Green #a6e3a1, Teal #94e2d5, Blue #89b4fa, Mauve #cba6f7, Pink #f5c2e7
    pub fn catppuccin_mocha_preset() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            dark_mode: true,

            override_text_color: Some([205, 214, 244, 255]), // Text      #cdd6f4
            override_weak_text_color: Some([108, 112, 134, 255]), // Overlay0  #6c7086
            override_hyperlink_color: Some([137, 180, 250, 255]), // Blue      #89b4fa
            override_faint_bg_color: Some([49, 50, 68, 255]), // Surface0  #313244
            override_extreme_bg_color: Some([17, 17, 27, 255]), // Crust     #11111b
            override_code_bg_color: Some([49, 50, 68, 255]), // Surface0
            override_warn_fg_color: Some([249, 226, 175, 255]), // Yellow    #f9e2af
            override_error_fg_color: Some([243, 139, 168, 255]), // Red       #f38ba8

            override_window_fill: Some([30, 30, 46, 255]), // Base    #1e1e2e
            override_window_stroke_color: Some([69, 71, 90, 255]), // Surface1 #45475a
            override_window_stroke_width: Some(1.0),
            override_window_corner_radius: Some(8),
            override_window_shadow_size: Some(20),

            override_panel_fill: Some([24, 24, 37, 255]), // Mantle #181825
            override_popup_shadow_size: Some(12),

            override_selection_bg: Some([88, 91, 112, 255]), // Surface2 #585b70
            override_selection_stroke_color: Some([203, 166, 247, 255]), // Mauve #cba6f7
            override_selection_stroke_width: Some(0.0),

            // noninteractive
            override_widget_noninteractive_bg_fill: Some([30, 30, 46, 255]), // Base
            override_widget_noninteractive_weak_bg_fill: Some([24, 24, 37, 255]), // Mantle
            override_widget_noninteractive_bg_stroke_color: Some([49, 50, 68, 255]), // Surface0
            override_widget_noninteractive_bg_stroke_width: Some(1.0),
            override_widget_noninteractive_corner_radius: Some(8),
            override_widget_noninteractive_fg_stroke_color: Some([108, 112, 134, 255]), // Overlay0
            override_widget_noninteractive_fg_stroke_width: Some(1.0),
            override_widget_noninteractive_expansion: Some(0.0),

            // inactive
            override_widget_inactive_bg_fill: Some([49, 50, 68, 255]), // Surface0
            override_widget_inactive_weak_bg_fill: Some([41, 42, 57, 255]),
            override_widget_inactive_bg_stroke_color: Some([69, 71, 90, 255]), // Surface1
            override_widget_inactive_bg_stroke_width: Some(1.0),
            override_widget_inactive_corner_radius: Some(8),
            override_widget_inactive_fg_stroke_color: Some([186, 194, 222, 255]), // Subtext1
            override_widget_inactive_fg_stroke_width: Some(1.5),
            override_widget_inactive_expansion: Some(0.0),

            // hovered
            override_widget_hovered_bg_fill: Some([69, 71, 90, 255]), // Surface1
            override_widget_hovered_weak_bg_fill: Some([58, 60, 78, 255]),
            override_widget_hovered_bg_stroke_color: Some([203, 166, 247, 255]), // Mauve
            override_widget_hovered_bg_stroke_width: Some(1.0),
            override_widget_hovered_corner_radius: Some(8),
            override_widget_hovered_fg_stroke_color: Some([205, 214, 244, 255]), // Text
            override_widget_hovered_fg_stroke_width: Some(1.5),
            override_widget_hovered_expansion: Some(1.0),

            // active — blue for primary action, inverted text
            override_widget_active_bg_fill: Some([137, 180, 250, 255]), // Blue
            override_widget_active_weak_bg_fill: Some([110, 150, 210, 255]),
            override_widget_active_bg_stroke_color: Some([203, 166, 247, 255]), // Mauve
            override_widget_active_bg_stroke_width: Some(1.0),
            override_widget_active_corner_radius: Some(8),
            override_widget_active_fg_stroke_color: Some([30, 30, 46, 255]), // Base (inverted)
            override_widget_active_fg_stroke_width: Some(2.0),
            override_widget_active_expansion: Some(1.0),

            // open
            override_widget_open_bg_fill: Some([88, 91, 112, 255]), // Surface2
            override_widget_open_weak_bg_fill: Some([69, 71, 90, 255]), // Surface1
            override_widget_open_bg_stroke_color: Some([203, 166, 247, 255]), // Mauve
            override_widget_open_bg_stroke_width: Some(1.0),
            override_widget_open_corner_radius: Some(8),
            override_widget_open_fg_stroke_color: Some([205, 214, 244, 255]), // Text
            override_widget_open_fg_stroke_width: Some(1.5),
            override_widget_open_expansion: Some(0.0),

            override_slider_trailing_fill: Some(true),
            override_button_frame: Some(true),
            override_indent_has_left_vline: Some(true),
            ..Default::default()
        }
    }

    pub fn all_presets() -> Vec<Self> {
        vec![
            Self::dark_preset(),
            Self::light_preset(),
            Self::dracula_preset(),
            Self::nord_preset(),
            Self::gruvbox_dark_preset(),
            Self::solarized_dark_preset(),
            Self::solarized_light_preset(),
            Self::monokai_preset(),
            Self::one_dark_preset(),
            Self::tokyo_night_preset(),
            Self::catppuccin_mocha_preset(),
        ]
    }

    pub fn to_visuals(&self) -> Visuals {
        let mut visuals = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        if let Some(color) = self.override_text_color {
            visuals.override_text_color = Some(Color32::from_rgba_unmultiplied(
                color[0], color[1], color[2], color[3],
            ));
        }

        if let Some(color) = self.override_weak_text_color {
            visuals.weak_text_color = Some(Color32::from_rgba_unmultiplied(
                color[0], color[1], color[2], color[3],
            ));
        }

        if let Some(color) = self.override_hyperlink_color {
            visuals.hyperlink_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_faint_bg_color {
            visuals.faint_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_extreme_bg_color {
            visuals.extreme_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_code_bg_color {
            visuals.code_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_warn_fg_color {
            visuals.warn_fg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_error_fg_color {
            visuals.error_fg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_window_fill {
            visuals.window_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_window_stroke_color {
            visuals.window_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_window_stroke_width {
            visuals.window_stroke.width = width;
        }

        if let Some(radius) = self.override_window_corner_radius {
            visuals.window_corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(size) = self.override_window_shadow_size {
            visuals.window_shadow.spread = size;
        }

        if let Some(color) = self.override_panel_fill {
            visuals.panel_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(size) = self.override_popup_shadow_size {
            visuals.popup_shadow.spread = size;
        }

        if let Some(color) = self.override_selection_bg {
            visuals.selection.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_selection_stroke_color {
            visuals.selection.stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_selection_stroke_width {
            visuals.selection.stroke.width = width;
        }

        if let Some(color) = self.override_widget_noninteractive_bg_fill {
            visuals.widgets.noninteractive.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_noninteractive_weak_bg_fill {
            visuals.widgets.noninteractive.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_noninteractive_bg_stroke_color {
            visuals.widgets.noninteractive.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_noninteractive_bg_stroke_width {
            visuals.widgets.noninteractive.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_noninteractive_corner_radius {
            visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_noninteractive_fg_stroke_color {
            visuals.widgets.noninteractive.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_noninteractive_fg_stroke_width {
            visuals.widgets.noninteractive.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_noninteractive_expansion {
            visuals.widgets.noninteractive.expansion = expansion;
        }

        if let Some(color) = self.override_widget_inactive_bg_fill {
            visuals.widgets.inactive.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_inactive_weak_bg_fill {
            visuals.widgets.inactive.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_inactive_bg_stroke_color {
            visuals.widgets.inactive.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_inactive_bg_stroke_width {
            visuals.widgets.inactive.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_inactive_corner_radius {
            visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_inactive_fg_stroke_color {
            visuals.widgets.inactive.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_inactive_fg_stroke_width {
            visuals.widgets.inactive.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_inactive_expansion {
            visuals.widgets.inactive.expansion = expansion;
        }

        if let Some(color) = self.override_widget_hovered_bg_fill {
            visuals.widgets.hovered.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_hovered_weak_bg_fill {
            visuals.widgets.hovered.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_hovered_bg_stroke_color {
            visuals.widgets.hovered.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_hovered_bg_stroke_width {
            visuals.widgets.hovered.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_hovered_corner_radius {
            visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_hovered_fg_stroke_color {
            visuals.widgets.hovered.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_hovered_fg_stroke_width {
            visuals.widgets.hovered.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_hovered_expansion {
            visuals.widgets.hovered.expansion = expansion;
        }

        if let Some(color) = self.override_widget_active_bg_fill {
            visuals.widgets.active.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_active_weak_bg_fill {
            visuals.widgets.active.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_active_bg_stroke_color {
            visuals.widgets.active.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_active_bg_stroke_width {
            visuals.widgets.active.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_active_corner_radius {
            visuals.widgets.active.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_active_fg_stroke_color {
            visuals.widgets.active.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_active_fg_stroke_width {
            visuals.widgets.active.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_active_expansion {
            visuals.widgets.active.expansion = expansion;
        }

        if let Some(color) = self.override_widget_open_bg_fill {
            visuals.widgets.open.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_open_weak_bg_fill {
            visuals.widgets.open.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_open_bg_stroke_color {
            visuals.widgets.open.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_open_bg_stroke_width {
            visuals.widgets.open.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_open_corner_radius {
            visuals.widgets.open.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_open_fg_stroke_color {
            visuals.widgets.open.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_open_fg_stroke_width {
            visuals.widgets.open.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_open_expansion {
            visuals.widgets.open.expansion = expansion;
        }

        if let Some(size) = self.override_resize_corner_size {
            visuals.resize_corner_size = size;
        }

        if let Some(width) = self.override_text_cursor_width {
            visuals.text_cursor.stroke.width = width;
        }

        if let Some(margin) = self.override_clip_rect_margin {
            visuals.clip_rect_margin = margin;
        }

        if let Some(button_frame) = self.override_button_frame {
            visuals.button_frame = button_frame;
        }

        if let Some(collapsing_header_frame) = self.override_collapsing_header_frame {
            visuals.collapsing_header_frame = collapsing_header_frame;
        }

        if let Some(indent_has_left_vline) = self.override_indent_has_left_vline {
            visuals.indent_has_left_vline = indent_has_left_vline;
        }

        if let Some(striped) = self.override_striped {
            visuals.striped = striped;
        }

        if let Some(slider_trailing_fill) = self.override_slider_trailing_fill {
            visuals.slider_trailing_fill = slider_trailing_fill;
        }

        visuals
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }

    pub fn to_rust_code(&self) -> String {
        let mut code = String::new();
        code.push_str("fn apply_theme(ctx: &egui::Context) {\n");
        code.push_str(&format!("    let mut visuals = if {} {{\n", self.dark_mode));
        code.push_str("        egui::Visuals::dark()\n");
        code.push_str("    } else {\n");
        code.push_str("        egui::Visuals::light()\n");
        code.push_str("    };\n\n");

        macro_rules! emit_color {
            ($field:expr, $path:expr) => {
                if let Some(color) = $field {
                    code.push_str(&format!(
                        "    {} = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                        $path, color[0], color[1], color[2], color[3]
                    ));
                }
            };
        }

        emit_color!(self.override_text_color.map(|c| {
            code.push_str(&format!(
                "    visuals.override_text_color = Some(egui::Color32::from_rgba_unmultiplied({}, {}, {}, {}));\n",
                c[0], c[1], c[2], c[3]
            ));
            c // not actually used; side-effect above
        }), "/* handled */");

        emit_color!(self.override_window_fill, "visuals.window_fill");
        emit_color!(self.override_panel_fill, "visuals.panel_fill");
        emit_color!(self.override_selection_bg, "visuals.selection.bg_fill");
        emit_color!(self.override_hyperlink_color, "visuals.hyperlink_color");
        emit_color!(self.override_faint_bg_color, "visuals.faint_bg_color");
        emit_color!(self.override_extreme_bg_color, "visuals.extreme_bg_color");
        emit_color!(self.override_code_bg_color, "visuals.code_bg_color");
        emit_color!(self.override_warn_fg_color, "visuals.warn_fg_color");
        emit_color!(self.override_error_fg_color, "visuals.error_fg_color");
        emit_color!(
            self.override_widget_inactive_bg_fill,
            "visuals.widgets.inactive.bg_fill"
        );
        emit_color!(
            self.override_widget_inactive_fg_stroke_color,
            "visuals.widgets.inactive.fg_stroke.color"
        );
        emit_color!(
            self.override_widget_hovered_bg_fill,
            "visuals.widgets.hovered.bg_fill"
        );
        emit_color!(
            self.override_widget_hovered_bg_stroke_color,
            "visuals.widgets.hovered.bg_stroke.color"
        );
        emit_color!(
            self.override_widget_hovered_fg_stroke_color,
            "visuals.widgets.hovered.fg_stroke.color"
        );
        emit_color!(
            self.override_widget_active_bg_fill,
            "visuals.widgets.active.bg_fill"
        );
        emit_color!(
            self.override_widget_active_bg_stroke_color,
            "visuals.widgets.active.bg_stroke.color"
        );
        emit_color!(
            self.override_widget_active_fg_stroke_color,
            "visuals.widgets.active.fg_stroke.color"
        );

        if let Some(r) = self.override_window_corner_radius {
            code.push_str(&format!(
                "    visuals.window_corner_radius = egui::CornerRadius::same({});\n",
                r
            ));
        }
        if let Some(v) = self.override_slider_trailing_fill {
            code.push_str(&format!("    visuals.slider_trailing_fill = {};\n", v));
        }
        if let Some(v) = self.override_button_frame {
            code.push_str(&format!("    visuals.button_frame = {};\n", v));
        }

        code.push_str("\n    ctx.set_visuals(visuals);\n");
        code.push_str("}\n");
        code
    }

    pub fn randomize() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let random_color =
            |rng: &mut rand::rngs::ThreadRng| -> [u8; 4] { [rng.gen(), rng.gen(), rng.gen(), 255] };

        let dark_mode = rng.gen_bool(0.5);

        Self {
            name: "Random".to_string(),
            dark_mode,
            override_text_color: Some(random_color(&mut rng)),
            override_window_fill: Some(random_color(&mut rng)),
            override_panel_fill: Some(random_color(&mut rng)),
            override_selection_bg: Some(random_color(&mut rng)),
            override_hyperlink_color: Some(random_color(&mut rng)),
            override_faint_bg_color: Some(random_color(&mut rng)),
            override_extreme_bg_color: Some(random_color(&mut rng)),
            override_code_bg_color: Some(random_color(&mut rng)),
            override_warn_fg_color: Some(random_color(&mut rng)),
            override_error_fg_color: Some(random_color(&mut rng)),
            ..Default::default()
        }
    }
}
