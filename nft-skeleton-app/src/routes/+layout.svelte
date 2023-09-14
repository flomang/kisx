<script lang="ts">
	import "../app.postcss";
	import { AppShell, AppBar } from "@skeletonlabs/skeleton";
	import { Avatar } from "@skeletonlabs/skeleton";
	import { page } from "$app/stores";
	import { Toast, initializeStores } from '@skeletonlabs/skeleton';

	initializeStores();
	let ignore = ["/login", "/signup", "/forgot"];
</script>

<!-- App Shell -->
<Toast />
<AppShell>
	<svelte:fragment slot="header">
		{#if $page.route.id && !ignore.includes($page.route.id)}
			<!-- App Bar -->
			<AppBar>
				<svelte:fragment slot="lead">
					<strong class="text-xl uppercase">kisx</strong>
				</svelte:fragment>
				<svelte:fragment slot="trail">
					<!-- <a
					class="btn btn-sm variant-ghost-surface"
					href="https://discord.gg/EXqV7W8MtY"
					target="_blank"
					rel="noreferrer"
				>
					Discord
				</a>
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://twitter.com/SkeletonUI"
					target="_blank"
					rel="noreferrer"
				>
					Twitter
				</a>
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://github.com/skeletonlabs/skeleton"
					target="_blank"
					rel="noreferrer"
				>
					GitHub
				</a> -->
					<!-- <Avatar src="https://i.pravatar.cc/" /> -->
					<Avatar src="invalid-image.jpg" initials="ML" />
				</svelte:fragment>
			</AppBar>
		{/if}
		<!-- Page Route Content -->
	</svelte:fragment>

	<!-- Page Route Content -->
	<slot />
</AppShell>
