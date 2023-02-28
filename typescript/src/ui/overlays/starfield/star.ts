import P5 from 'p5';

export default class Star {
    p5: P5;
    x: number;
    y: number;
    z: number;
    pz: number;

    constructor(p5: P5) {
        this.p5 = p5;

        this.x = p5.random(-p5.width, p5.width);
        this.y = p5.random(-p5.height, p5.height);
        this.z = p5.random(p5.width);
        this.pz = this.z;
    }

    update() {
        this.z = this.z - 4;

        if (this.z < 1) {
            this.x = this.p5.random(-this.p5.width, this.p5.width);
            this.y = this.p5.random(-this.p5.height, this.p5.height);
            this.z = this.p5.width;
            this.pz = this.z;
        }
    }

    draw() {
        const sx = this.p5.map(this.x / this.z, 0, 1, 0, this.p5.width);
        const sy = this.p5.map(this.y / this.z, 0, 1, 0, this.p5.height);
        const radius = this.p5.map(this.z, 0, this.p5.width, 8, 0);
        const px = this.p5.map(this.x / this.pz, 0, 1, 0, this.p5.width);
        const py = this.p5.map(this.y / this.pz, 0, 1, 0, this.p5.height);

        this.pz = this.z;

        this.p5.push();

        this.p5.fill('#999');
        this.p5.stroke('#999');

        this.p5.line(px, py, sx, sy);

        this.p5.pop();
    }
}