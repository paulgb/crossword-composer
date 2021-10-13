<script>
  import { Crossword } from "./crossword"
  import { ParallelSolver } from "./parallel/main"

  const allowedDimensions = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
  $: dimension = 8

  $: crossword = new Crossword(dimension, gridChanged)
  let solver = new ParallelSolver(onSolution)

  let hideLetters = false

  // Status message to display.
  let status = ""

  // Whether the filler is currently running.
  let autoRun = false

  // If the board has changed since the last successful run.
  let dirty = true

  // Called when the grid is changed, either because the dimensions change,
  // or because a cell is toggled.
  function gridChanged() {
    if (autoRun) {
      solver.solve(crossword.words)
    }
    dirty = true
    hideLetters = false
  }

  // Called when the "start" button is clicked.
  function enableAutoRun(e) {
    e.preventDefault()
    solver.solve(crossword.words)
    autoRun = true
    hideLetters = false
  }

  // Called when the "stop" button is clicked.
  function disableAutoRun(e) {
    e.preventDefault()
    solver.terminate()
    autoRun = false
  }

  // Called when the user toggles a cell between filled and unfilled by clicking.
  function toggleCell(i, j) {
    crossword.toggle(i, j);
    crossword = crossword;
  }

  // Called when a new solution is received from the background worker.
  function onSolution(result) {
    crossword.setLetters(result.data)
    crossword = crossword
    dirty = false
  }

  // Status message update loop.
  setInterval(() => {
    status = solver.getStatus()
  }, 100)
</script>

<style>
  .controls {
    display: flex;
  }

  .controls p, .controls label {
    margin: 10px;
  }

  button {
    cursor: pointer;
    border: none;
    color: white;
    padding: 10px 15px;
  }

  .red {
    background: #752726;
  }

  .green {
    background: #26752b;
  }

  .number {
    position: absolute;
    font-size: 8pt;
    top: 2px;
    left: 2px;
    color: #88c;
    font-weight: normal;
  }

  .letter {
    padding-top: 9px;
  }

  table.crossword {
    border-collapse: collapse;
    margin: 40px auto;
  }

  .crossword td {
    border: 1px solid black;
    height: 36px;
    width: 36px;
    border-spacing: 0;
    text-align: center;
    font-weight: bold;
    color: #333;
    background: #fff;
    cursor: default;
  }

  .cell {
    position: relative;
    height: 36px;
  }

  .crossword.dirty td {
    color: #999;
  }

  .crossword td.filled {
    background: #333;
  }
</style>

<div>
  <table class={`crossword ${dirty ? 'dirty' : ''}`}>
    <tbody>
      {#each crossword.grid as row, i}
        <tr>
          {#each row as cell, j}
            <td
              on:click={() => toggleCell(i, j)}
              class={cell.filled ? 'filled' : ''}
              title={cell.index}>
              <div class="cell">
                <div class="letter">{(hideLetters || cell.filled) ? '' : (cell.value || '').toUpperCase()}</div>
                <div class="number">{cell.number ? cell.number : ''}</div>
              </div>
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

  <div class="controls">
    <div>
      {#if autoRun}
        <button on:click={disableAutoRun} class="red">Stop</button>
      {:else}
        <button on:click={enableAutoRun} class="green">Start</button>
      {/if}
    </div>
    <div style="flex: 1;"><p>{status}</p></div>
    <div>
      <label>
        Hide Letters: <input bind:checked={hideLetters} type="checkbox" />
      </label>
    </div>
    <div>
      <label>
        Size:
        <select bind:value={dimension}>
          {#each allowedDimensions as ad}
            <option value={ad}>{ad} x {ad}</option>
          {/each}
        </select>
      </label>
    </div>
  </div>
</div>
