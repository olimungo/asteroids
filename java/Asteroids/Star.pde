public class Star {
    private float x;
    private float y;
    private float z;
    private float pz;

    Star() {
        this.x = random(-width, width);
        this.y = random(-height, height);
        this.z = random(width);
        this.pz = this.z;
    }

    void update() {
        this.z = this.z - 4;

        if (this.z < 1) {
            this.x = random(-width, width);
            this.y = random(-height, height);
            this.z = width;
            this.pz = this.z;
        }
    }

    void draw() {
        float sx = map(this.x / this.z, 0, 1, 0, width);
        float sy = map(this.y / this.z, 0, 1, 0, height);
        float radius = map(this.z, 0, width, 8, 0);
        float px = map(this.x / this.pz, 0, 1, 0, width);
        float py = map(this.y / this.pz, 0, 1, 0, height);

        this.pz = this.z;

        push();

        fill(Colors.LIGHT);
        stroke(Colors.LIGHT);

        line(px, py, sx, sy);

        pop();
    }
}