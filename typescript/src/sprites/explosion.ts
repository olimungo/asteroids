import P5 from 'p5';
import Particle from './particle';

export default class Explosion {
    particles: Particle[] = [];

    constructor(p5: P5, position: P5.Vector) {
        for (let index = 0; index < 8; index++) {
            this.particles.push(
                new Particle(p5, position.copy(), p5.random(1, 4))
            );
        }
    }

    update() {
        this.particles = this.particles.filter((particle) => particle.update());
    }

    draw() {
        for (const particle of this.particles) {
            particle.draw();
        }
    }

    isOver(): boolean {
        return this.particles.length === 0;
    }
}
