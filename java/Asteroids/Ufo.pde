public class Ufo extends Sprite {
    private final static int UFO_WIDTH = 60;
    private final static int UFO_HEIHT = 25;
    private final static int UFO_VELOCITY = 2;
    private final static int CHANGE_HEADING_FREQUENCY = 6000;
    private final static int VARIABILITY_IN_HEADING = 3000;
    private final static int VARIABILITY_IN_SHOOTING = 1000;

    private PShape shape;
    private ArrayList<Laser> lasers = new ArrayList<Laser>();

    // Helps to define actions based on a time frequency
    private Interval changeHeadingInterval;
    private Interval shootInterval;

    Ufo(PVector position, int shootIntervalFrequency) {
        super(
            position,
            (UFO_WIDTH + UFO_HEIHT) / 2,
            PVector.random2D(),
            0);

        this.velocity.setMag(UFO_VELOCITY);
        this.shape = this.generateShape(this.generateVertices());

        this.changeHeadingInterval = new Interval(
            int(random(
                CHANGE_HEADING_FREQUENCY - VARIABILITY_IN_HEADING,
                CHANGE_HEADING_FREQUENCY + VARIABILITY_IN_HEADING
            ))
        );

        if (shootIntervalFrequency > 0) {
            this.shootInterval = new Interval(
                int(random(
                    shootIntervalFrequency - VARIABILITY_IN_SHOOTING,
                    shootIntervalFrequency + VARIABILITY_IN_SHOOTING
                ))
            );
        }
    }

    Boolean update(PVector shipPosition) {
        if (this.changeHeadingInterval.isElapsed()) {
            this.velocity = PVector.random2D();
            this.velocity.setMag(UFO_VELOCITY);
        }

        if (shipPosition != null) {
            if (this.shootInterval != null && this.shootInterval.isElapsed()) {
                PVector target = PVector.sub(shipPosition, this.position);

                this.lasers.add(
                    new Laser(this.position.copy(), target.heading())
                );
            }

            for (int laserIndex = this.lasers.size() - 1; laserIndex >= 0 ; laserIndex--) {
                Laser laser = this.lasers.get(laserIndex);

                laser.update();

                if (laser.isOffScreen) {
                    this.lasers.remove(laserIndex);
                }
            }
        }

        super.update();

        return true;
    }

    void draw() {
        push();

        translate(
            this.position.x - this.UFO_WIDTH / 2,
            this.position.y - this.UFO_HEIHT / 2
        );

        shape(this.shape, 0, 0);

        pop();

        for (Laser laser : this.lasers) {
            laser.draw();
        }
    }

    void pause() {
        this.changeHeadingInterval.pause();
        this.shootInterval.pause();
    }

    void unpause() {
        this.changeHeadingInterval.unpause();
        this.shootInterval.unpause();
    }

     Boolean lasersHit(Sprite sprite) {
        for (int laserIndex = this.lasers.size() - 1; laserIndex >= 0 ; laserIndex--) {
            Laser laser = this.lasers.get(laserIndex);

            if (laser.collideWith(sprite)) {
                this.lasers.remove(laserIndex);
                return true;
            }
        }

        return false;
    }

    private ArrayList<PVector> generateVertices() {
        ArrayList<PVector> vertices = new ArrayList<PVector>();

        float height1 = UFO_HEIHT / 4;
        float height2 = (UFO_HEIHT / 8) * 5;
        float width1 = UFO_WIDTH / 3;
        float width2 = UFO_WIDTH * 0.66;
        float width3 = (UFO_WIDTH / 10) * 6;
        float width4 = (UFO_WIDTH / 10) * 4;

        vertices.add(new PVector(7, height2));
        vertices.add(new PVector(width1, UFO_HEIHT - 1));
        vertices.add(new PVector(width2, UFO_HEIHT - 1));
        vertices.add(new PVector(UFO_WIDTH - 7, height2));
        vertices.add(new PVector(7, height2));
        vertices.add(new PVector(width1, height1));
        vertices.add(new PVector(width2, height1));
        vertices.add(new PVector(UFO_WIDTH - 7, height2));
        vertices.add(new PVector(width2, height1));
        vertices.add(new PVector(width3, 1));
        vertices.add(new PVector(width4, 1));
        vertices.add(new PVector(width1, height1));

        return vertices;
    }

    private PShape generateShape(ArrayList<PVector> vertices) {
        PShape shape = createShape();

        shape.beginShape();

        shape.noFill();
        shape.stroke(Colors.EDGE);
        shape.strokeWeight(1.3);

        for (PVector vertex : vertices) {
            shape.vertex(vertex.x, vertex.y);
        }

        shape.endShape(CLOSE);

        return shape;
    }
}