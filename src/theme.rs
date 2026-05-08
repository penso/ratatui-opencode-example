use {ratatui::style::Color, ratatui_opentui_loader::Theme as LoaderTheme};

#[derive(Debug, Clone, Copy)]
pub struct ColorTheme {
    pub bg: Color,
    pub bg_panel: Color,
    pub bg_element: Color,
    pub bg_hover: Color,
    pub bg_user: Color,
    pub text: Color,
    pub text_muted: Color,
    pub primary: Color,
    pub error: Color,
    pub border: Color,
    pub palette_bg: Color,
    pub palette_selected: Color,
}

impl ColorTheme {
    const fn new(
        bg: Color,
        bg_panel: Color,
        bg_element: Color,
        text: Color,
        text_muted: Color,
        primary: Color,
        error: Color,
        border: Color,
    ) -> Self {
        Self {
            bg,
            bg_panel,
            bg_element,
            text,
            text_muted,
            primary,
            error,
            border,
            bg_hover: bg_element,
            bg_user: bg_panel,
            palette_bg: bg_panel,
            palette_selected: Color::Rgb(0, 128, 128),
        }
    }

    #[allow(dead_code)]
    pub fn from_loader_theme(theme: LoaderTheme) -> Self {
        match theme {
            LoaderTheme::Dracula => DRACULA,
            LoaderTheme::Gruvbox => GRUVBOX,
            LoaderTheme::Catppuccin => CATPPUCCIN,
            LoaderTheme::Nord => NORD,
            LoaderTheme::Tokyonight => TOKYONIGHT,
            LoaderTheme::Github => GITHUB,
            _ => OPENCODE,
        }
    }
}

const fn dark_theme(base: u8, accent: Color, err: Color) -> ColorTheme {
    ColorTheme::new(
        Color::Rgb(base, base, base),
        Color::Rgb(base + 10, base + 10, base + 10),
        Color::Rgb(base + 20, base + 20, base + 20),
        Color::Rgb(220, 220, 220),
        Color::Rgb(128, 128, 128),
        accent,
        err,
        Color::Rgb(base + 30, base + 30, base + 30),
    )
}

// Full palette themes (extracted from opencode theme JSONs)
pub const OPENCODE: ColorTheme = ColorTheme::new(
    Color::Rgb(10, 10, 10),
    Color::Rgb(20, 20, 20),
    Color::Rgb(30, 30, 30),
    Color::Rgb(238, 238, 238),
    Color::Rgb(128, 128, 128),
    Color::Rgb(250, 178, 131),
    Color::Rgb(224, 108, 117),
    Color::Rgb(72, 72, 72),
);
pub const DRACULA: ColorTheme = ColorTheme::new(
    Color::Rgb(40, 42, 54),
    Color::Rgb(33, 34, 44),
    Color::Rgb(68, 71, 90),
    Color::Rgb(248, 248, 242),
    Color::Rgb(98, 114, 164),
    Color::Rgb(189, 147, 249),
    Color::Rgb(255, 85, 85),
    Color::Rgb(68, 71, 90),
);
pub const GRUVBOX: ColorTheme = ColorTheme::new(
    Color::Rgb(40, 40, 40),
    Color::Rgb(60, 56, 54),
    Color::Rgb(80, 73, 69),
    Color::Rgb(235, 219, 178),
    Color::Rgb(146, 131, 116),
    Color::Rgb(131, 165, 152),
    Color::Rgb(251, 73, 52),
    Color::Rgb(102, 92, 84),
);
pub const CATPPUCCIN: ColorTheme = ColorTheme::new(
    Color::Rgb(30, 30, 46),
    Color::Rgb(24, 24, 37),
    Color::Rgb(17, 17, 27),
    Color::Rgb(205, 214, 244),
    Color::Rgb(147, 153, 178),
    Color::Rgb(137, 180, 250),
    Color::Rgb(243, 139, 168),
    Color::Rgb(49, 50, 68),
);
pub const CATPPUCCIN_FRAPPE: ColorTheme = ColorTheme::new(
    Color::Rgb(48, 52, 70),
    Color::Rgb(41, 44, 60),
    Color::Rgb(35, 38, 52),
    Color::Rgb(198, 208, 245),
    Color::Rgb(131, 139, 167),
    Color::Rgb(141, 164, 226),
    Color::Rgb(231, 130, 132),
    Color::Rgb(65, 68, 89),
);
pub const CATPPUCCIN_MACCHIATO: ColorTheme = ColorTheme::new(
    Color::Rgb(36, 39, 58),
    Color::Rgb(30, 32, 48),
    Color::Rgb(24, 25, 38),
    Color::Rgb(202, 211, 245),
    Color::Rgb(139, 141, 152),
    Color::Rgb(138, 173, 244),
    Color::Rgb(237, 135, 150),
    Color::Rgb(54, 57, 78),
);
pub const NORD: ColorTheme = ColorTheme::new(
    Color::Rgb(46, 52, 64),
    Color::Rgb(59, 66, 82),
    Color::Rgb(67, 76, 94),
    Color::Rgb(236, 239, 244),
    Color::Rgb(139, 149, 167),
    Color::Rgb(136, 192, 208),
    Color::Rgb(191, 97, 106),
    Color::Rgb(67, 76, 94),
);
pub const TOKYONIGHT: ColorTheme = ColorTheme::new(
    Color::Rgb(26, 27, 38),
    Color::Rgb(30, 32, 48),
    Color::Rgb(34, 36, 54),
    Color::Rgb(200, 211, 245),
    Color::Rgb(130, 139, 184),
    Color::Rgb(130, 170, 255),
    Color::Rgb(255, 117, 127),
    Color::Rgb(115, 122, 162),
);
pub const SOLARIZED: ColorTheme = ColorTheme::new(
    Color::Rgb(0, 43, 54),
    Color::Rgb(7, 54, 66),
    Color::Rgb(10, 64, 76),
    Color::Rgb(253, 246, 227),
    Color::Rgb(131, 148, 150),
    Color::Rgb(108, 113, 196),
    Color::Rgb(220, 50, 47),
    Color::Rgb(88, 110, 117),
);
pub const GITHUB: ColorTheme = ColorTheme::new(
    Color::Rgb(13, 17, 23),
    Color::Rgb(1, 4, 9),
    Color::Rgb(22, 27, 34),
    Color::Rgb(201, 209, 217),
    Color::Rgb(139, 148, 158),
    Color::Rgb(88, 166, 255),
    Color::Rgb(248, 81, 73),
    Color::Rgb(48, 54, 61),
);

// Generated dark themes (derived from accent + base brightness)
pub const ROSEPINE: ColorTheme =
    dark_theme(25, Color::Rgb(156, 207, 216), Color::Rgb(235, 111, 146));
pub const AYU: ColorTheme = dark_theme(10, Color::Rgb(63, 183, 227), Color::Rgb(240, 113, 120));
pub const MONOKAI: ColorTheme = dark_theme(30, Color::Rgb(174, 129, 255), Color::Rgb(249, 38, 114));
pub const ONE_DARK: ColorTheme =
    dark_theme(30, Color::Rgb(97, 175, 239), Color::Rgb(224, 108, 117));
pub const KANAGAWA: ColorTheme = dark_theme(31, Color::Rgb(126, 156, 216), Color::Rgb(195, 74, 71));
pub const MATERIAL: ColorTheme =
    dark_theme(26, Color::Rgb(130, 170, 255), Color::Rgb(240, 113, 120));
pub const EVERFOREST: ColorTheme =
    dark_theme(39, Color::Rgb(167, 192, 128), Color::Rgb(230, 126, 128));
pub const AMOLED: ColorTheme = dark_theme(0, Color::Rgb(179, 136, 255), Color::Rgb(255, 85, 85));
pub const AURA: ColorTheme = dark_theme(21, Color::Rgb(162, 119, 255), Color::Rgb(255, 102, 103));
pub const CARBONFOX: ColorTheme =
    dark_theme(22, Color::Rgb(51, 177, 255), Color::Rgb(238, 85, 150));
pub const COBALT2: ColorTheme = dark_theme(25, Color::Rgb(0, 136, 255), Color::Rgb(255, 98, 140));
pub const CURSOR: ColorTheme = dark_theme(26, Color::Rgb(136, 192, 208), Color::Rgb(191, 97, 106));
pub const FLEXOKI: ColorTheme = dark_theme(16, Color::Rgb(218, 112, 44), Color::Rgb(209, 77, 65));
pub const MATRIX: ColorTheme = dark_theme(10, Color::Rgb(46, 255, 106), Color::Rgb(255, 51, 51));
pub const MERCURY: ColorTheme =
    dark_theme(26, Color::Rgb(141, 164, 245), Color::Rgb(240, 113, 120));
pub const NIGHTOWL: ColorTheme = dark_theme(1, Color::Rgb(130, 170, 255), Color::Rgb(239, 83, 80));
pub const PALENIGHT: ColorTheme =
    dark_theme(29, Color::Rgb(130, 170, 255), Color::Rgb(240, 113, 120));
pub const SHADES_OF_PURPLE: ColorTheme =
    dark_theme(30, Color::Rgb(199, 146, 255), Color::Rgb(255, 98, 140));
pub const SYNTHWAVE84: ColorTheme =
    dark_theme(26, Color::Rgb(54, 249, 246), Color::Rgb(254, 68, 80));
pub const VESPER: ColorTheme = dark_theme(16, Color::Rgb(255, 199, 153), Color::Rgb(245, 110, 110));
pub const ZENBURN: ColorTheme =
    dark_theme(40, Color::Rgb(140, 208, 211), Color::Rgb(220, 140, 140));
pub const VERCEL: ColorTheme = dark_theme(10, Color::Rgb(0, 112, 243), Color::Rgb(255, 0, 0));
pub const ORNG: ColorTheme = dark_theme(10, Color::Rgb(236, 91, 43), Color::Rgb(255, 68, 68));
pub const OSAKA_JADE: ColorTheme =
    dark_theme(10, Color::Rgb(45, 213, 183), Color::Rgb(247, 118, 142));

pub fn all_themes() -> &'static [(LoaderTheme, ColorTheme)] {
    &[
        (LoaderTheme::Opencode, OPENCODE),
        (LoaderTheme::Dracula, DRACULA),
        (LoaderTheme::Gruvbox, GRUVBOX),
        (LoaderTheme::Catppuccin, CATPPUCCIN),
        (LoaderTheme::CatppuccinFrappe, CATPPUCCIN_FRAPPE),
        (LoaderTheme::CatppuccinMacchiato, CATPPUCCIN_MACCHIATO),
        (LoaderTheme::Nord, NORD),
        (LoaderTheme::Tokyonight, TOKYONIGHT),
        (LoaderTheme::Solarized, SOLARIZED),
        (LoaderTheme::Rosepine, ROSEPINE),
        (LoaderTheme::Ayu, AYU),
        (LoaderTheme::Monokai, MONOKAI),
        (LoaderTheme::OneDark, ONE_DARK),
        (LoaderTheme::Kanagawa, KANAGAWA),
        (LoaderTheme::Material, MATERIAL),
        (LoaderTheme::Everforest, EVERFOREST),
        (LoaderTheme::Github, GITHUB),
        (LoaderTheme::Amoled, AMOLED),
        (LoaderTheme::Aura, AURA),
        (LoaderTheme::Carbonfox, CARBONFOX),
        (LoaderTheme::Cobalt2, COBALT2),
        (LoaderTheme::Cursor, CURSOR),
        (LoaderTheme::Flexoki, FLEXOKI),
        (LoaderTheme::Matrix, MATRIX),
        (LoaderTheme::Mercury, MERCURY),
        (LoaderTheme::Nightowl, NIGHTOWL),
        (LoaderTheme::Palenight, PALENIGHT),
        (LoaderTheme::ShadesOfPurple, SHADES_OF_PURPLE),
        (LoaderTheme::Synthwave84, SYNTHWAVE84),
        (LoaderTheme::Vesper, VESPER),
        (LoaderTheme::Zenburn, ZENBURN),
        (LoaderTheme::Vercel, VERCEL),
        (LoaderTheme::Orng, ORNG),
        (LoaderTheme::OsakaJade, OSAKA_JADE),
    ]
}
