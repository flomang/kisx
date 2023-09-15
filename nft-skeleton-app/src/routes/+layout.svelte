<script lang="ts">
	import "../app.postcss";
	import { AppShell, AppBar } from "@skeletonlabs/skeleton";
	import client, { removeToken } from "../lib/apollo";
	import { Avatar } from "@skeletonlabs/skeleton";
	import { page } from "$app/stores";
	import { Toast, initializeStores } from "@skeletonlabs/skeleton";
	import {
		computePosition,
		autoUpdate,
		offset,
		shift,
		flip,
		arrow,
	} from "@floating-ui/dom";
	import { popup } from "@skeletonlabs/skeleton";
	import type { PopupSettings } from "@skeletonlabs/skeleton";
	import { storePopup } from "@skeletonlabs/skeleton";
	import { goto } from "$app/navigation";
	import type { PageData } from "./$types";
	import Icon from "@iconify/svelte";

	export let data: PageData;

	// init toast store
	initializeStores();

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	let ignore = ["/login", "/signup", "/forgot"];

	const popupFeatured: PopupSettings = {
		// Represents the type of event that opens/closed the popup
		event: "click",
		// Matches the data-popup value on your popup element
		target: "popupFeatured",
		// Defines which side of your trigger the popup will appear
		placement: "bottom",
	};

	const handleSignout = async () => {
		client.cache.reset();
		removeToken();
		goto("/login");
		data = {};
	};
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
					<button
						type="button"
						class="btn-icon btn-icon-sm variant-filled"
						use:popup={popupFeatured}
					>
						<Icon
							icon="majesticons:user-line"
							color="gray"
							width="30"
							height="30"
						/></button
					>
				</svelte:fragment>
			</AppBar>
		{/if}
		<!-- Page Route Content -->
	</svelte:fragment>

	<!-- Page Route Content -->
	<slot />
</AppShell>

<div class="card p-4 max-w-sm" data-popup="popupFeatured">
	<div class="grid grid-cols-1 gap-2">
		<div>{data.username}</div>
		<button
			type="button"
			class="btn btn-sm variant-soft-primary"
			on:click={handleSignout}>logout</button
		>
	</div>
</div>
