export default class Interval {
    interval: number;
    timeReference: number = 0;
    timePaused: number = 0;

    set(interval: number) {
        this.interval = interval;
        this.timePaused = 0;
        this.timeReference = Date.now();
    }

    isElapsed(): boolean {
        if (!this.timePaused && this.interval > 0) {
            const now = Date.now();

            if (this.timeReference + this.interval < now) {
                this.timeReference = now;
                return true;
            }
        }

        return false;
    }

    cancel() {
        this.interval = 0;
    }

    pause() {
        this.timePaused = Date.now();
    }

    unpause() {
        const now = Date.now();
        const timeLeft = this.timeReference + this.interval - this.timePaused;

        this.timeReference = now + timeLeft - this.interval;
        this.timePaused = 0;
    }
}
