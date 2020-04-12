<script>
import Crossword from './Crossword.svelte'

$: dimension = 5;
</script>

<div>
	<label>Height/width: <input bind:value={dimension} /></label>
	<Crossword dimension={parseInt(dimension)} />
</div>