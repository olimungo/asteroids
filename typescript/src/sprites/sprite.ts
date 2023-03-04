import P5 from 'p5';
import Colors from '../ui/colors';

export default class Sprite {
    p5: P5;
    position: P5.Vector;
    velocity = new P5.Vector();
    diameter: number;
    rotation = 0;
    rotationStep = 0;

    constructor(
        p5: P5,
        position: P5.Vector,
        diameter: number,
        velocity: P5.Vector,
        rotationStep: number = 0
    ) {
        this.p5 = p5;
        this.position = position;
        this.diameter = diameter;
        this.velocity = velocity;
        this.rotationStep = rotationStep;
    }

    update(...args: any): boolean {
        this.position.add(this.velocity);
        this.checkWindowEdges();

        this.rotation += this.rotationStep;

        return true;
    }

    draw(...args: any) {
        this.p5.push();

        this.p5.translate(this.position.x, this.position.y);
        this.p5.rotate(this.rotation);

        this.p5.fill(Colors.EDGE);
        this.p5.noStroke();

        this.p5.ellipse(0, 0, this.diameter);

        this.p5.fill(Colors.DARK);
        this.p5.rectMode(this.p5.CENTER);
        this.p5.rect(0, 0, this.diameter / 4, this.diameter / 4);

        this.p5.pop();
    }

    checkWindowEdges(): boolean {
        const radius = this.diameter / 2;
        const width = this.p5.width;
        const height = this.p5.height;
        const x = this.position.x;
        const y = this.position.y;

        if (x > width + radius) {
            this.position.x = -radius;
            return true;
        } else if (x < -radius) {
            this.position.x = width + radius;
            return true;
        }

        if (y > height + radius) {
            this.position.y = -radius;
            return true;
        } else if (y < -radius) {
            this.position.y = height + radius;
            return true;
        }

        return false;
    }

    collideWith(sprite: Sprite): boolean {
        const distance = this.p5.dist(
            this.position.x,
            this.position.y,
            sprite.position.x,
            sprite.position.y
        );

        if (distance < this.diameter / 2 + sprite.diameter / 2) {
            return true;
        }

        return false;
    }
}
