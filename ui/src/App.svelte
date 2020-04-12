<script>
import Crossword from './Crossword.svelte'

$: dimension = 6;
</script>

<div>
	<label>Dimensions: <input bind:value={dimension} /></label>
	<Crossword dimension={parseInt(dimension)} />
</div>