public class Ufo extends Sprite {
    int ufoWidth = 60;
    int ufoHeight = 25;
    Boolean shipHit = false;

    private GameManager gameManager;
    private PVector[] shapeVectors;
    private PShape ufo;
    private Ship ship;
    private ArrayList<Laser> lasers = new ArrayList<Laser>();
    private int timerShoot = 0;

    Ufo(float x, float y, Ship ship) {
        super(x, y, 20);

        this.ship = ship;
        this.velocity = PVector.random2D();
        this.velocity.setMag(2.5);
        this.gameManager = gameManager;

        this.shapeVectors = this.generateShapeVectors();
    }

    @Override
    void update() {
        super.update();
        this.ufo = this.generateShape(this.shapeVectors);

        if (this.ship != null) {
            if (this.timerShoot == 0) {
                this.timerShoot = millis() + floor(random(10000, 15000));
            }

            this.checkTimerShoot();
        }

        for (int i = this.lasers.size() - 1; i >= 0; i--) {
            Laser laser = this.lasers.get(i);

            if (laser.hitsShip()) {
                this.lasers.remove(laser);
                this.shipHit = true;
            }

            if (laser.isOffScreen) {
                this.lasers.remove(laser);
            }
        }
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            translate(this.position.x, this.position.y);
            shape(this.ufo, 0, 0);
        popMatrix();
        popStyle();

        for (int i = this.lasers.size() - 1; i >= 0; i--) {
            Laser laser = this.lasers.get(i);
            laser.update();
            laser.draw();
        }
    }

    Boolean hit(float x, float y, float radius) {
        if ((x + radius > this.position.x && x + radius < this.position.x + this.ufoWidth) &&
            (y + radius > this.position.y && y + radius < this.position.y + this.ufoHeight)) {
                return true;
        }

        return false;
    }

    Boolean hitsShip() {
        return this.shipHit;
    }

    private PVector[] generateShapeVectors() {
        PVector[] vectors = new PVector[12];

        float height1 = this.ufoHeight / 4;
        float height2 = this.ufoHeight / 8 * 5;
        float width1 = this.ufoWidth / 3;
        float width2 = this.ufoWidth * 0.66;
        float width3 = this.ufoWidth / 10 * 6;
        float width4 = this.ufoWidth / 10 * 4;

        vectors[0] = new PVector(0, height2);
        vectors[1] = new PVector(width1, this.ufoHeight);
        vectors[2] = new PVector(width2, this.ufoHeight);
        vectors[3] = new PVector(this.ufoWidth, height2);
        vectors[4] = new PVector(0, height2);
        vectors[5] = new PVector(width1, height1);
        vectors[6] = new PVector(width2, height1);
        vectors[7] = new PVector(this.ufoWidth, height2);
        vectors[8] = new PVector(width2, height1);
        vectors[9] = new PVector(width3, 0);
        vectors[10] = new PVector(width4, 0);
        vectors[11] = new PVector(width1, height1);

        return vectors;
    }

    private PShape generateShape(PVector[] vectors) {
        PShape shape = createShape();

        shape.beginShape();
        shape.noFill();
        shape.strokeWeight(1.3);

        for (int i = 0 ; i < vectors.length; i++) {
            float alpha = random(170, 255);
            shape.stroke(219, 233, 255, alpha);
            shape.vertex(vectors[i].x, vectors[i].y);
        }

        shape.endShape();

        return shape;
    }

    private void checkTimerShoot() {
        if (millis() > this.timerShoot) {
            this.timerShoot = 0;

            PVector target = PVector.sub(this.ship.position, this.position);
            PVector middle = new PVector(this.ufoWidth / 2, this.ufoHeight / 2);
            PVector position = PVector.add(this.position, middle);

            this.lasers.add(new Laser(position, target.heading(), this.ship));
        }
    }
}