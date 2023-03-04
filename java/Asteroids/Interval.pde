public class Interval {
    private int intervalFrequency;
    private int timeReference;
    private int timePaused = 0;

    Interval(int intervalFrequency) {
        this.intervalFrequency = intervalFrequency;
        this.timeReference = millis();

    }

    Boolean isElapsed() {
        if (this.timePaused == 0 && this.intervalFrequency > 0) {
            int now = millis();

            if (this.timeReference + this.intervalFrequency < now) {
                this.timeReference = now;
                return true;
            }
        }

        return false;
    }

    void pause() {
        this.timePaused = millis();
    }

    void unpause() {
        int now = millis();
        int timeLeft =
            this.timeReference + this.intervalFrequency - this.timePaused;

        this.timeReference = now + timeLeft - this.intervalFrequency;
        this.timePaused = 0;
    }
}
