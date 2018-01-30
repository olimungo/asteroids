public class Star {
    float x;
    float y;
    float z;
    float pz;

    Star() {
        this.x = random(-width, width);
        this.y = random(-height, height);
        this.z = random(width);
        this.pz = z;
    }

    void update() {
        this.z = this.z - 4;

        if (this.z < 1) {
            this.x = random(-width, width);
            this.y = random(-height, height);
            this.z = width; 
            this.pz = z;
        }
    }

    void draw() {
        float sx = map(x / z, 0, 1, 0, width);
        float sy = map(y / z, 0, 1, 0, height);
        float radius = map(z, 0, width, 8, 0);
        float px = map(x / pz, 0, 1, 0, width);
        float py = map(y / pz, 0, 1, 0, height);

        this.pz = z;

        pushMatrix();
            fill(219, 233, 255);
            noStroke();
            stroke(219, 233, 255);
            line(px, py, sx, sy);
        popMatrix();
    }
}