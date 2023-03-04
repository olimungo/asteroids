import P5 from 'p5';
import Ship from '../../sprites/ship';
import Overlay from './overlay';

export default class Lifes extends Overlay {
    private ships: Ship[] = [];

    constructor(p5: P5) {
        super(p5);
    }

    draw() {
        this.p5.push();

        for (let ship of this.ships) {
            ship.draw();
        }

        this.p5.pop();
    }

    setLifeCount(count: number) {
        const y = 50;
        let x = 50;

        this.ships = [];

        for (let counter = 0; counter < count; counter++) {
            this.ships.push(new Ship(this.p5, new P5.Vector(x, y), true));

            x += 40;
        }
    }
}
