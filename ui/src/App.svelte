<script>
import Crossword from './Crossword.svelte'

const allowedDimensions = [
	3,
	4,
	5,
	6,
	7,
	8,
	9,
	10,
	11,
	12
]

$: dimension = 5;
</script>

<div class="main">
	<Crossword dimension={parseInt(dimension)} />
	<label>Size: <select bind:value={dimension}>
	{#each allowedDimensions as ad}
	<option value={ad}>{ad} x {ad}</option>
	{/each}
	</select>
	</label>
</div>

<style>
.main {
	width: 600px;
	margin: auto;
}
</style>