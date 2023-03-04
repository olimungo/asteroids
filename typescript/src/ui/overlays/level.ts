import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class Level extends Overlay {
    draw(level: number) {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textFont(Fonts.fontLight);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.textSize(25);

        this.p5.text('LEVEL ' + level, this.centerX, 55);

        this.p5.pop();
    }
}
