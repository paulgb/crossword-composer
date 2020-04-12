import init, * as cw from '../../pkg/crossword';

export class Solver {
    async setup() {
        let result = await fetch('/words.txt')
        let text = await result.text()
        let arr = text.split('\n')

        await init()

        console.log('here1')
        this.solver = cw.Solver.new(arr)
        console.log('here2')
    }

    constructor() {
        this.solver = null
        this.setup()
    }

    solve(grid) {
        if (!this.solver) {
            console.log('asked to solve before ready.')
            return null
        }

        return this.solver.solve(grid)
    }
}