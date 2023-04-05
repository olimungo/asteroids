public class Interval {
    private int interval;
    private int timeReference = 0;
    private int timePaused = 0;

    void set(int interval) {
        this.interval = interval;
        this.timePaused = 0;
        this.timeReference = millis();
    }

    Boolean isElapsed() {
        if (this.timePaused == 0 && this.interval > 0) {
            int now = millis();

            if (this.timeReference + this.interval < now) {
                this.timeReference = now;
                return true;
            }
        }

        return false;
    }

    void cancel(){
        this.interval = 0;
    }

    void pause() {
        this.timePaused = millis();
    }

    void unpause() {
        int now = millis();
        int timeLeft =
            this.timeReference + this.interval - this.timePaused;

        this.timeReference = now + timeLeft - this.interval;
        this.timePaused = 0;
    }
}
