public class Lifes extends Overlay {
    private ArrayList<Ship> ships = new ArrayList<Ship>();

    void draw() {
        push();

        for (Ship ship : this.ships) {
            ship.draw();
        }

        pop();
    }

    void setLifeCount(int count) {
        int y = 50;
        int x = 50;

        this.ships.clear();

        for (int counter = 0; counter < count; counter++) {
            this.ships.add(new Ship(new PVector(x, y), true));

            x += 40;
        }
    }
}