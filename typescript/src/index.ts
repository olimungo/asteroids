import P5 from 'p5';
import Colors from './ui/colors';
import GameManager from './game-manager';
import Fonts from './ui/fonts';

const sketch = (p5: P5) => {
    let gameManager: GameManager;

    p5.preload = () => {
        Fonts.setFontThin(p5.loadFont('fonts/Exo2-Thin.ttf'));
        Fonts.setFontLight(p5.loadFont('fonts/Exo2-Light.ttf'));

        // p5.frameRate(25);
    };

    p5.setup = () => {
        const canvas = p5.createCanvas(1024, 700, 'p2d');
        canvas.parent('p5');

        p5.textFont(Fonts.fontThin);

        gameManager = new GameManager(p5);
    };

    p5.draw = () => {
        p5.background(Colors.BACKGROUND);

        gameManager.update();
        gameManager.draw();
    };

    p5.keyPressed = (event: any) => {
        gameManager.keyPressed(event.keyCode);
    };

    p5.keyReleased = (event: any) => {
        gameManager.keyReleased(event.keyCode);
    };
};

new P5(sketch);
