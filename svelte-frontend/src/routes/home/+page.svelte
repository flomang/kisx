<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../../lib/apollo";
    import type { PageData } from "./$types";

    export let data: PageData;

    const handleSignout = async () => {
        client.cache.reset();
        removeToken();
        goto("/signin");
    };
</script>

{#if data.username}
    <h1>Welcome Home</h1>
    <div>{data.username}</div>

    <form on:submit|preventDefault={handleSignout}>
        <button type="submit">Signout</button>
    </form>
{/if}

