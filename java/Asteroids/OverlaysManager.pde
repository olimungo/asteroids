public class OverlaysManager {
    Help help ;
    Hub hub;

    OverlaysManager() {
        this.help = new Help();
        this.hub = new Hub();
    }

    void draw() {
        // this.help.draw();
        this.hub.draw();
    }
}