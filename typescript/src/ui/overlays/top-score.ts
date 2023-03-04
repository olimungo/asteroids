import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class TopScore extends Overlay {
    draw(score: number) {
        if (score > 0) {
            this.p5.push();

            this.p5.fill(Colors.EDGE);
            this.p5.textFont(Fonts.fontLight);
            this.p5.textAlign(this.p5.CENTER);
            this.p5.textSize(25);

            this.p5.text('TOP   ' + score, this.centerX + 220, 55);

            this.p5.pop();
        }
    }
}
