import P5 from 'p5';
import Colors from '../ui/colors';
import Sprite from './sprite';

export default class Patatoid extends Sprite {
    private shape: P5.Graphics;

    sides = 0;

    constructor(
        p5: P5,
        position: P5.Vector,
        diameter: number,
        sides: number,
        rotationStep: number
    ) {
        super(p5, position, diameter, rotationStep);

        this.sides = sides;
        this.shape = this.generateShape(this.generateVertices());
    }

    draw() {
        this.p5.push();

        this.p5.translate(this.position.x, this.position.y);

        this.p5.rotate(this.rotation);

        this.p5.image(this.shape, -this.diameter / 2, -this.diameter / 2);

        this.p5.pop();
    }

    private generateVertices(): P5.Vector[] {
        const vertices: P5.Vector[] = [];

        for (let side = 0; side < this.sides; side++) {
            const radius = this.p5.random(
                this.diameter * 0.35,
                this.diameter * 0.5
            );
            const angle = this.p5.map(side, 0, this.sides, 0, this.p5.TWO_PI);
            const x = radius * this.p5.cos(angle);
            const y = radius * this.p5.sin(angle);

            vertices.push(new P5.Vector(x, y));
        }

        return vertices;
    }

    private generateShape(vertices: P5.Vector[]): P5.Graphics {
        const shape = this.p5.createGraphics(this.diameter, this.diameter);

        shape.translate(this.diameter / 2, this.diameter / 2);

        shape.strokeWeight(1.4);
        shape.stroke(Colors.EDGE);
        shape.noFill();

        shape.beginShape();

        for (let vertex of vertices) {
            shape.vertex(vertex.x, vertex.y);
        }

        shape.endShape(this.p5.CLOSE);

        return shape;
    }
}