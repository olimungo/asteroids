public class Laser extends Sprite {
    Boolean isOffScreen = false;

    private ArrayList<Asteroid> asteroids;
    private ArrayList<Ufo> ufos;
    private Ship ship;

    Laser(PVector position, float angle) {
        super(position.x, position.y, 4);
        this.velocity = PVector.fromAngle(angle).mult(10);
    }

    Laser(PVector position, float angle, ArrayList<Asteroid> asteroids, ArrayList<Ufo> ufos) {
        super(position.x, position.y, 4);
        this.velocity = PVector.fromAngle(angle).mult(10);
        this.asteroids = asteroids;
        this.ufos = ufos;
    }

    Laser(PVector position, float angle, Ship ship) {
        super(position.x, position.y, 1);
        this.velocity = PVector.fromAngle(angle).mult(10);
        this.ship = ship;
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            stroke(255);
            strokeWeight(4);
            point(this.position.x, this.position.y);
        popMatrix();
        popStyle();
    }

    @Override
    Boolean checkEdges() {
        this.isOffScreen = super.checkEdges();
        
        return this.isOffScreen;
    }

    // Boolean hitsUfo() {
    //     for (Ufo ufo : this.ufos) {
    //         if (ufo.hit(this.position.x, this.position.y, this.radius)) {
    //             ufos.remove(ufo);

    //             return true;
    //         }
    //     }

    //     return false;
    // }

    // Boolean hitsShip() {
    //     if (this.ship.hit(this.position.x, this.position.y, this.radius)) {
    //         return true;
    //     }

    //     return false;
    // }
}