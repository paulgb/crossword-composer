<script>
import Crossword from './Crossword.svelte'

$: dimension = 5;
</script>

<div class="main">
	<Crossword dimension={parseInt(dimension)} />
	<label>Height/width: <input bind:value={dimension} /></label>
</div>

<style>
.main {
	width: 600px;
	margin: auto;
}
</style>