use mottle::dsl::{s, tm, ThemeBuilder};

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    ui(&mut theme_builder, &Palette::APPRENTICE);
    editor(&mut theme_builder, &Palette::APPRENTICE);
    let theme = theme_builder.build("Apprentice");
    mottle::save_theme(&theme)?;

    let mut theme_builder = ThemeBuilder::default();
    ui(&mut theme_builder, &Palette::SORCERER);
    editor(&mut theme_builder, &Palette::SORCERER);
    let theme = theme_builder.build("Sorcerer");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn ui(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["foreground", "editor.foreground"], p.lighter_grey);
    t.w(["editor.background"], p.darker_grey);

    t.w(["editorCursor.foreground", "terminalCursor.foreground"], p.lighter_grey);
    t.w(["editorCursor.background", "terminalCursor.background"], p.darker_grey);

    t.w(["sideBar.background", "activityBar.background"], p.almost_black);

    t.w(["statusBar.foreground"], p.darker_grey);
    t.w(
        ["statusBar.background", "statusBar.noFolderBackground", "statusBar.debuggingBackground"],
        p.ocre,
    );

    t.w(["editorLineNumber.foreground"], p.light_grey);
    t.w(["editorLineNumber.activeForeground"], p.light_aqua);

    t.w(["editor.lineHighlightBackground"], p.dark_grey);

    t.w(["editor.selectionBackground"], p.grey);

    t.w(["list.activeSelectionForeground"], p.darker_grey);
    t.w(["list.focusHighlightForeground"], p.almost_black);
    t.w(["list.highlightForeground"], p.lighter_grey);
    t.w(["list.activeSelectionBackground"], p.aqua);
    t.w(["list.inactiveSelectionBackground"], p.grey);
    t.w(["list.hoverBackground"], (0xFFFFFF, 0x10));
    t.w(["editorWidget.background"], p.dark_grey);

    t.w(["input.placeholderForeground"], p.medium_grey);
    t.w(["input.background", "checkbox.background"], p.almost_black);
    t.w(["input.border", "checkbox.border"], p.grey);
    t.w(["button.foreground"], p.white);
    t.w(["button.background"], p.grey);
    t.w(["button.hoverBackground"], p.blue);

    t.w(["editor.foldBackground"], p.almost_black);

    t.w(["editorInlayHint.foreground"], p.medium_grey);
    t.w(["editorInlayHint.background"], (0x000000, 0x00));

    t.w(["editorWhitespace.foreground"], p.medium_grey);

    t.w(["terminal.foreground"], p.lighter_grey);
    t.w(["terminal.ansiBlack"], p.almost_black);
    t.w(["terminal.ansiRed"], p.red);
    t.w(["terminal.ansiGreen"], p.green);
    t.w(["terminal.ansiYellow"], p.ocre);
    t.w(["terminal.ansiBlue"], p.blue);
    t.w(["terminal.ansiMagenta"], p.purple);
    t.w(["terminal.ansiCyan"], p.aqua);
    t.w(["terminal.ansiWhite"], p.lighter_grey);
    t.w(["terminal.ansiBrightBlack"], p.grey);
    t.w(["terminal.ansiBrightRed"], p.orange);
    t.w(["terminal.ansiBrightGreen"], p.light_green);
    t.w(["terminal.ansiBrightYellow"], p.yellow);
    t.w(["terminal.ansiBrightBlue"], p.light_blue);
    t.w(["terminal.ansiBrightMagenta"], p.light_purple);
    t.w(["terminal.ansiBrightCyan"], p.light_aqua);
    t.w(["terminal.ansiBrightWhite"], p.white);

    t.w(["focusBorder"], p.blue);
}

fn editor(t: &mut ThemeBuilder, p: &Palette) {
    t.a(
        [
            s("keyword"),
            s("operator"),
            tm("keyword"),
            tm("storage"),
            tm("keyword.type.go"),
            tm("punctuation.separator"),
            tm("punctuation.definition.keyValue"),
        ],
        p.light_blue,
    );

    t.a([s("variable"), s("parameter"), s("property"), s("enumMember")], p.lighter_grey);

    t.a(
        [
            s("number"),
            s("boolean"),
            s("character"),
            s("variable.constant"),
            tm("constant"),
            tm("keyword.other.unit"),
            tm("string.quoted.single.char"),
            tm("string.quoted.single.c"),
            tm("string.quoted.single.cpp"),
            tm("string.quoted.single.java"),
            tm("string.quoted.single.cs"),
            tm("string.quoted.rune.go"),
        ],
        p.orange,
    );

    t.a([s("string"), tm("string")], p.light_green);
    t.a(
        [
            s("formatSpecifier"),
            s("escapeSequence"),
            tm("constant.character.escape"),
            tm("constant.other.placeholder"),
            tm("constant.character.format.placeholder"),
            tm("punctuation.section.embedded"),
            tm("punctuation.definition.template-expression"),
            tm("punctuation.definition.interpolation"),
        ],
        p.green,
    );

    t.a([s("function"), s("method"), tm("entity.name.function"), tm("support.function")], p.yellow);

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
            tm("entity.name.type"),
            tm("keyword.type"),
            tm("storage.type.built-in"),
            tm("storage.type.primitive"),
            tm("storage.type.java"),
            tm("storage.type.numeric.go"),
            tm("storage.type.byte.go"),
            tm("storage.type.boolean.go"),
            tm("storage.type.string.go"),
            tm("storage.type.uintptr.go"),
            tm("storage.type.error.go"),
            tm("storage.type.rune.go"),
            tm("support.type"),
        ],
        p.light_purple,
    );

    t.a([s("selfKeyword"), tm("variable.language"), tm("keyword.other.this")], p.blue);

    t.a(
        [
            s("macro"),
            s("*.attribute"),
            tm("entity.name.function.macro"),
            tm("keyword.control.directive"),
            tm("keyword.control.at-rule"),
            tm("variable.scss"),
        ],
        p.aqua,
    );

    t.a(
        [s("lifetime"), tm("entity.name.type.lifetime"), tm("punctuation.definition.lifetime")],
        p.green,
    );

    t.a([s("comment"), tm("comment")], p.light_grey);

    t.a([s("punctuation")], p.lighter_grey);

    t.a([tm("entity.name.tag"), tm("punctuation.definition.tag")], p.light_blue);
    t.a([tm("entity.other.attribute-name")], p.light_purple);

    t.a([tm("meta.tag.table.toml"), tm("entity.other.attribute-name.table.toml")], p.white);
    t.a([tm("entity.name.tag.toml")], p.blue);

    t.a([tm("markup.inserted")], p.light_green);
    t.a([tm("markup.deleted")], p.red);

    t.a(
        [
            tm("punctuation.definition.heading.markdown"),
            tm("punctuation.definition.raw.markdown"),
            tm("punctuation.definition.markdown"),
            tm("fenced_code.block.language.markdown"),
        ],
        p.green,
    );
    t.a([tm("markup.italic.markdown")], p.aqua);
    t.a(
        [
            tm("punctuation.definition.list.begin.markdown"),
            tm("punctuation.definition.quote.begin.markdown"),
        ],
        p.light_blue,
    );
    t.a([tm("entity.name.section.markdown")], p.white);
    t.a([tm("string.other.link.title.markdown")], p.yellow);
    t.a([tm("constant.other.reference.link.markdown")], p.light_purple);
    t.a([tm("markup.underline.link.markdown")], p.orange);

    t.a([s("unresolvedReference")], p.lighter_grey);
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
}

impl Palette {
    const APPRENTICE: Self = Self {
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
    };

    const SORCERER: Self = Self {
        almost_black: 0x171717,
        darker_grey: 0x202020,
        dark_grey: 0x2a2a2a,
        grey: 0x444444,
        medium_grey: 0x585858,
        light_grey: 0x6c6c6c,
        lighter_grey: 0xc2c2b0,
        white: 0xffffe8,
        purple: 0x5d6a85,
        light_purple: 0x7e8aa2,
        green: 0x719611,
        light_green: 0x779b70,
        aqua: 0x528b8b,
        light_aqua: 0x58b8b8,
        blue: 0x6689ad,
        light_blue: 0x90b0d1,
        red: 0x996666,
        orange: 0xcc8800,
        ocre: 0x808070,
        yellow: 0xfaf4c6,
    };
}
