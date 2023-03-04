import P5 from 'p5';

export default class Fonts {
    static fontThin: P5.Font;
    static fontLight: P5.Font;

    static setFontThin(font: P5.Font) {
        Fonts.fontThin = font;
    }

    static setFontLight(font: P5.Font) {
        Fonts.fontLight = font;
    }
}
