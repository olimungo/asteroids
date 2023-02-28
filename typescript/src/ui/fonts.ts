import P5 from 'p5';

export default class Fonts {
    private static instance: Fonts;

    fontThin: P5.Font;
    fontLight: P5.Font;

    private constructor() {}

    static getInstance(): Fonts {
        if (!Fonts.instance) {
            Fonts.instance = new Fonts();
        }

        return Fonts.instance;
    }

    loadFonts(p5: P5) {
        this.fontThin = p5.loadFont('fonts/Exo2-Thin.ttf');
        this.fontLight = p5.loadFont('fonts/Exo2-Light.ttf');
    }
}
