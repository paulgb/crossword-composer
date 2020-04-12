
export class Solver {
    async setup() {
        let result = await fetch('/words.txt')
        let text = await result.text()
        this.wordlist = text.split('\n')
    }

    constructor(onSolution) {
        this.worker = null
        this.wordlist = null
        this.onSolution = onSolution
        this.setup()
    }

    terminate() {
        if (this.worker) {
            this.worker.terminate()
        }
    }

    messageReceived(message) {
        this.onSolution(message)
    }

    solve(grid) {
        this.terminate()

        this.worker = new Worker('/build/worker.js')

        this.worker.onmessage = this.messageReceived.bind(this)

        this.worker.postMessage({
            wordlist: this.wordlist,
            grid
        })
    }
}