<script>
  export let dimension;

  import { Crossword } from "./crossword";
  import { Solver } from "./solver";

  $: crossword = new Crossword(dimension, gridChanged);
  let solver = new Solver(onSolution);
  let status = "";
  let autoRun = false;
  let dirty = true;

  function gridChanged() {
    if (autoRun) {
      solver.solve(crossword.words)
    }
    dirty = true
  }

  function changeRunFiller() {
    autoRun = !autoRun;

    if (autoRun) {
      solver.solve(crossword.words);
    } else {
      solver.terminate();
    }
  }

  function toggleCell(i, j) {
    crossword.toggle(i, j)
    crossword = crossword
  }

  function onSolution(result) {
    crossword.setLetters(result.data)
    crossword = crossword
    dirty = false
  }

  setInterval(() => {
    status = solver.getStatus()
  }, 100);
</script>

<style>
  .number {
    position: absolute;
    font-size: 8pt;
    top: 2px;
    left: 2px;
    color: #88c;
    font-weight: normal;
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
    position: relative;
    font-weight: bold;
    color: #333;
    background: #fff;
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
              {cell.filled ? '' : (cell.value || '').toUpperCase()}
              <div class="number">{cell.number ? cell.number : ''}</div>
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

  <label>
    Run filler
    <input on:change={changeRunFiller} type="checkbox" checked={autoRun} />
  </label>
  <div>{status}</div>
</div>
