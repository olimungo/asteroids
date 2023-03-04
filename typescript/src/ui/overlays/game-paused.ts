import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class GamePaused extends Overlay {
    draw() {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textFont(Fonts.fontLight);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.textSize(60);

        this.p5.text('GAME PAUSED ', this.centerX, this.centerY - 100);

        this.p5.pop();
    }
}
