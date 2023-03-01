import P5 from 'p5';
import Colors from '../colors';
import Fonts from '../fonts';
import Keycap from './keycap';
import Overlay from './overlay';
import Spacebar from './spacebar';

export default class Help extends Overlay {
    private keycapLeft: Keycap;
    private keycapRight: Keycap;
    private keycapUp: Keycap;
    private spacebar: Spacebar;

    constructor(p5: P5) {
        super(p5);

        this.keycapLeft = new Keycap(p5, new P5.Vector(650, 285), -90);
        this.keycapRight = new Keycap(p5, new P5.Vector(770, 285), 90);
        this.keycapUp = new Keycap(p5, new P5.Vector(710, 215), 0);
        this.spacebar = new Spacebar(p5, new P5.Vector(710, 365));
    }

    draw() {
        this.p5.push();

        this.p5.fill(Colors.BACKGROUND);
        this.p5.stroke(Colors.EDGE);
        this.p5.strokeWeight(2);
        this.p5.rect(150, 150, this.p5.width - 300, this.p5.height - 300, 10);

        this.p5.fill(Colors.EDGE);
        this.p5.noStroke();
        this.p5.textFont(Fonts.getInstance().fontLight);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.textSize(35);
        this.p5.text('HELP', this.centerX, 210);

        this.p5.textAlign(this.p5.LEFT);
        this.p5.textSize(15);
        this.p5.text('START', 220, 275);
        this.p5.text('PAUSE', 220, 325);
        this.p5.text('HUB', 220, 375);
        this.p5.text('SHOW EGDES', 220, 425);
        this.p5.text('STARFIELD', 220, 475);

        this.p5.textAlign(this.p5.RIGHT);
        this.p5.text('S', 475, 275);
        this.p5.text('P', 475, 325);
        this.p5.text('U', 475, 375);
        this.p5.text('E', 475, 425);
        this.p5.text('X', 475, 475);

        this.keycapLeft.draw();
        this.keycapRight.draw();
        this.keycapUp.draw();

        this.p5.textSize(15);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.text('BOOST', 710, 250);
        this.p5.text('TURN LEFT', 650, 320);
        this.p5.text('TURN RIGHT', 770, 320);
        this.p5.text('SHOOT', 710, 400);

        this.p5.textAlign(this.p5.LEFT);
        this.p5.text('ASTEROID', 600, 450);
        this.p5.text('UFO', 600, 475);
        this.p5.text('NEW LIFE', 600, 500);

        this.p5.text('10 POINTS', 745, 450);
        this.p5.text('50 POINTS', 745, 475);
        this.p5.text('3000 POINTS', 745, 500);

        this.spacebar.draw();

        this.p5.pop();
    }
}
