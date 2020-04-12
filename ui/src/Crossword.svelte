<script>
export let dimension;

import {Crossword} from './crossword'
import {Solver} from './solver'

$: crossword = new Crossword(dimension)
let solver = new Solver(onSolution)

function toggleCell(i, j) {
    crossword.toggle(i, j)
    crossword = crossword
}

function onSolution(result) {
    crossword.setLetters(result.data)
    crossword = crossword
}

function solve() {
    let result = solver.solve(crossword.words)
}
</script>

<div>
    <table class="crossword">
        <tbody>
            {#each crossword.grid as row, i}
                <tr>
                    {#each row as cell, j}
                    <td on:click={() => toggleCell(i, j)} class={cell.filled ? 'filled' : ''} title={cell.index}>
                        { cell.filled ? '' : cell.value }
                        <div class="number">{ cell.number ? cell.number : '' }</div>
                    </td>
                    {/each}
                </tr>
            {/each}
        </tbody>
    </table>

    <button on:click={solve}>solve</button>
</div>

<style>
.number {
    position: absolute;
    font-size: 8pt;
    top: 2px;
    left: 2px;
    color: #888;
}

table.crossword {
    border-collapse: collapse;
}

.crossword td {
    border: 1px solid black;
    height: 30px;
    width: 30px;
    border-spacing: 0;
    text-align: center;
    position: relative;
}

.filled {
    background: black;
}
</style>