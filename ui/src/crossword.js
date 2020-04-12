class Cell {
    constructor() {
        this.filled = false
        this.value = ''
        this.number = null
        this.downWord = null
        this.acrossWord = null
    }
}

export class Crossword {
    constructor(dimension) {
        this.dimension = dimension
        this.grid = new Array(dimension).fill(0).map(() =>
            new Array(dimension).fill(null).map(() => new Cell())
        );
        this.words = null;

        this.generateNumbers()
    }

    setLetters(values) {
        for (let i = 0; i < this.dimension; i++) {
            for (let j = 0; j < this.dimension; j++) {
                let cell = this.grid[i][j]
                if (cell.index !== null) {
                    cell.value = values[cell.index]
                } else {
                    cell.value = ''
                }
            }
        }
    }

    generateNumbers() {
        let words = []
        let lastNumberedCell = 0
        let nextIndex = 0
        for (let i = 0; i < this.dimension; i++) {
            for (let j = 0; j < this.dimension; j++) {
                let cell = this.grid[i][j]

                if (cell.filled) {
                    cell.value = ''
                    cell.number = null
                    cell.downWord = null
                    cell.acrossWord = null
                    continue
                }

                cell.index = nextIndex++

                // Is this cell in the first {row, column} of the grid?
                const firstRow = i === 0
                const firstCol = j === 0

                // Is the cell before this filled in if it exists?
                const prevRowFilled = firstRow || this.grid[i-1][j].filled
                const prevColFilled = firstCol || this.grid[i][j-1].filled

                // Is the cell in the last {row, column} of the grid?
                const lastRow = i === this.dimension - 1
                const lastCol = j === this.dimension - 1

                // Is the cell after this one filled in if it exists?
                const nextRowFilled = lastRow || this.grid[i+1][j].filled
                const nextColFilled = lastCol || this.grid[i][j+1].filled

                // Is this cell the start of an up/down word?
                const downWord = prevRowFilled && !nextRowFilled
                const acrossWord = prevColFilled && !nextColFilled

                if (downWord) {
                    cell.downWord = words.length
                    words.push([cell.index])
                } else if (!prevRowFilled) {
                    cell.downWord = this.grid[i-1][j].downWord
                    words[cell.downWord].push(cell.index)
                } else {
                    cell.downWord = null
                }

                if (acrossWord) {
                    cell.acrossWord = words.length
                    words.push([cell.index])
                } else if (!prevColFilled) {
                    cell.acrossWord = this.grid[i][j-1].acrossWord
                    words[cell.acrossWord].push(cell.index)
                } else {
                    cell.acrossWord = null
                }

                // Annotate the cell with a visual number.
                if (downWord || acrossWord) {
                    cell.number = ++lastNumberedCell
                } else {
                    cell.number = null
                }
            }
        }

        this.words = words;
    }

    toggle(i, j) {
        this.grid[i][j].filled = !this.grid[i][j].filled
        this.grid[this.dimension-1-i][this.dimension-1-j].filled = this.grid[i][j].filled

        this.generateNumbers()
    }
}