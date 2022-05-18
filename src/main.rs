use mottle::dsl::{s, ThemeBuilder};

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    ui(&mut theme_builder, &Palette::default());
    editor(&mut theme_builder, &Palette::default());
    let theme = theme_builder.build("Apprentice");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn ui(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["foreground", "editor.foreground"], p.lighter_grey);
    t.w(["editor.background"], p.darker_grey);

    t.w(["statusBar.foreground"], p.darker_grey);
    t.w(
        [
            "statusBar.background",
            "statusBar.noFolderBackground",
            "statusBar.debuggingBackground",
        ],
        p.ocre,
    );
}

fn editor(t: &mut ThemeBuilder, p: &Palette) {
    t.a([s("keyword"), s("operator")], p.light_blue);

    t.a([s("number"), s("boolean"), s("character")], p.orange);

    t.a([s("string")], p.light_green);
    t.a([s("formatSpecifier"), s("escapeSequence")], p.green);

    t.a([s("function"), s("method")], p.yellow);

    t.a(
        [
            s("type"),
            s("class"),
            s("struct"),
            s("enum"),
            s("interface"),
            s("union"),
            s("typeParameter"),
            s("typeAlias"),
            s("builtinType"),
        ],
        p.light_purple,
    );

    t.a([s("macro"), s("derive")], p.aqua);

    t.a([s("lifetime")], p.green);

    t.a([s("comment")], p.light_grey);
}

struct Palette {
    almost_black: u32,
    darker_grey: u32,
    dark_grey: u32,
    grey: u32,
    medium_grey: u32,
    light_grey: u32,
    lighter_grey: u32,
    white: u32,
    purple: u32,
    light_purple: u32,
    green: u32,
    light_green: u32,
    aqua: u32,
    light_aqua: u32,
    blue: u32,
    light_blue: u32,
    red: u32,
    orange: u32,
    ocre: u32,
    yellow: u32,
    DiffAdd: u32,
    DiffDelete: u32,
    DiffChange: u32,
    DiffText: u32,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            almost_black: 0x1c1c1c,
            darker_grey: 0x262626,
            dark_grey: 0x303030,
            grey: 0x444444,
            medium_grey: 0x585858,
            light_grey: 0x6c6c6c,
            lighter_grey: 0xbcbcbc,
            white: 0xffffff,
            purple: 0x5f5f87,
            light_purple: 0x8787af,
            green: 0x5f875f,
            light_green: 0x87af87,
            aqua: 0x5f8787,
            light_aqua: 0x5fafaf,
            blue: 0x5f87af,
            light_blue: 0x87afd7,
            red: 0xaf5f5f,
            orange: 0xff8700,
            ocre: 0x87875f,
            yellow: 0xffffaf,
            DiffAdd: 0x87afff,
            DiffDelete: 0xffdf87,
            DiffChange: 0xdfdfdf,
            DiffText: 0xafafaf,
        }
    }
}
