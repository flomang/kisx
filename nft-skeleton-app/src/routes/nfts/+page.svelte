<script>
	let title = "";
	let description = "";
	let price = "";
	let selectedOption = "set";

	const options = ["set", "box", "instruction"];

	function handleSubmit() {
		// Handle form submission here
		console.log("Title:", title);
		console.log("Description:", description);
		console.log("Price:", price);
		console.log("Selected Option:", selectedOption);
	}
</script>

<div class="container">
	<label class="label">
		<span>Title</span>
		<input class="input form-input rounded" type="text" placeholder="title" />
	</label>
	<label class="label">
		<span>Description</span>
		<textarea class="textarea form-input rounded" rows="4" placeholder="description" />
	</label>
	<label class="label">
		<span>Amount</span>
		<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] rounded">
			<div class="input-group-shim">ETH</div>
			<input type="text" placeholder="Amount" />
		</div>
	</label>
	<label class="label">
		<span>Type</span>
		<select class="select form-input rounded">
			<option value="1">set</option>
			<option value="2">instruction</option>
			<option value="3">part</option>
			<option value="4">moc</option>
			<option value="5">box</option>
			<option value="6">other</option>
		</select>
	</label>
	<label class="label">
		<span>Upload your images</span>
		<input class="input rounded" type="file" multiple />
	</label>
	<button type="button" class="btn variant-filled rounded">Sell It</button>
</div>

<style lang="postcss">
	.container{ 
		padding: 1rem;
		width: 30%;
	}
	input {
		color: black;
	}
	#message {
		color: black;
	}
	figure {
		@apply flex relative flex-col;
	}
	figure svg,
	.img-bg {
		@apply w-64 h-64 md:w-80 md:h-80;
	}
	.img-bg {
		@apply absolute z-[-1] rounded-full blur-[50px] transition-all;
		animation: pulse 5s cubic-bezier(0, 0, 0, 0.5) infinite,
			glow 5s linear infinite;
	}
	@keyframes glow {
		0% {
			@apply bg-primary-400/50;
		}
		33% {
			@apply bg-secondary-400/50;
		}
		66% {
			@apply bg-tertiary-400/50;
		}
		100% {
			@apply bg-primary-400/50;
		}
	}
	@keyframes pulse {
		50% {
			transform: scale(1.5);
		}
	}
</style>
