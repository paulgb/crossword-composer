import { RunStatus } from '../status';

export class ParallelSolver {
  async setup() {
    let result = await fetch('/words.txt');
    let text = await result.text();
    this.wordlist = text.split('\n');

    this.workerBlob = await fetch('/build/worker.js').then((response) => response.blob());
    this.wasmResponse = await fetch('/build/crossword_bg.wasm').then((response) =>
      response.arrayBuffer()
    );
  }

  constructor(onSolution) {
    this.workers = [];
    this.numThreads = navigator.hardwareConcurrency;
    this.status = new RunStatus();
    this.wordlist = null;
    this.onSolution = onSolution;
    this.setup();
  }

  terminate() {
    this.workers.forEach((worker) => {
      worker.terminate();
    });
    this.status.end();
  }

  messageReceived(message) {
    this.terminate();
    this.onSolution(message);
  }

  getStatus() {
    if (this.status.started) {
      return `${this.status.getStatus()} (${this.status.getTime()})`;
    } else {
      return 'Click Start to begin filling.';
    }
  }

  solve(grid) {
    this.terminate();
    this.status.start();

    this.workers = [];
    for (let i = 0; i < this.numThreads; i++) {
      const worker = new Worker(URL.createObjectURL(this.workerBlob));
      worker.onmessage = this.messageReceived.bind(this);
      this.workers.push(worker);

      worker.postMessage({
        wasm: this.wasmResponse,
        wordlist: this.wordlist,
        grid,
        id: i,
        numThreads: this.numThreads,
      });
    }
  }
}
