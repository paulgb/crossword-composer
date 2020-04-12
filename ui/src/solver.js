
class RunStatus {
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

export class Solver {
    async setup() {
        let result = await fetch('/words.txt')
        let text = await result.text()
        this.wordlist = text.split('\n')
    }

    constructor(onSolution) {
        this.worker = null
        this.status = new RunStatus()
        this.wordlist = null
        this.onSolution = onSolution
        this.setup()
    }

    terminate() {
        if (this.worker) {
            this.worker.terminate()
            this.status.end()
        }
    }

    messageReceived(message) {
        this.status.end()
        this.onSolution(message)
    }

    getStatus() {
        return `${this.status.getStatus()} (${this.status.getTime()})`
    }

    solve(grid) {
        this.terminate()
        this.status.start()
        this.worker = new Worker('/build/worker.js')

        this.worker.onmessage = this.messageReceived.bind(this)

        this.worker.postMessage({
            wordlist: this.wordlist,
            grid
        })
    }
}