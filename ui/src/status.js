export class RunStatus {
    constructor() {
        this.started = null
        this.ended = null
    }

    start() {
        this.started = new Date()
        this.ended = null
    }

    end() {
        this.ended = new Date()
    }

    getRunning() {
        return this.started && !this.ended
    }

    getStatus() {
        return this.started ? (this.ended ? 'Finished' : 'Running') : 'Not Started';
    }

    getTime() {
        if (!this.started) {
            return '(not started)'
        }
        let end = this.ended || new Date()
        let s = end.valueOf() - this.started.valueOf()

        if (s > 1000) {
            return `${Math.round(s / 1000)}s`
        } else {
            return `${s}ms`
        }
    }
}