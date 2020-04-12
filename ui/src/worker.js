import init, * as cw from '../../pkg/crossword';

self.onmessage = ({data}) => {
    init('crossword_bg.wasm').then(() => {
        let solver = cw.Solver.new(data.wordlist)
        let result = solver.solve(data.grid)
        postMessage(result)
    })
}
