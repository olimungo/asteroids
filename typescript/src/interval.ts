export default class Interval {
    intervalFrequency: number;
    timeReference: number;
    timePaused: number = 0;

    constructor(intervalFrequency: number) {
        this.intervalFrequency = intervalFrequency;
        this.timeReference = Date.now();
    }

    isElapsed(): boolean {
        if (!this.timePaused && this.intervalFrequency > 0) {
            const now = Date.now();

            if (this.timeReference + this.intervalFrequency < now) {
                this.timeReference = now;
                return true;
            }
        }

        return false;
    }

    pause() {
        this.timePaused = Date.now();
    }

    unpause() {
        const now = Date.now();
        const timeLeft =
            this.timeReference + this.intervalFrequency - this.timePaused;

        this.timeReference = now + timeLeft - this.intervalFrequency;
        this.timePaused = 0;
    }
}
