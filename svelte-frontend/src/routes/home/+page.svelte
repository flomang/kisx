<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../../lib/apollo";
    import type { PageData } from "./$types";
    import LayoutGrid, { Cell } from "@smui/layout-grid";

    export let data: PageData;

    const handleSignout = async () => {
        client.cache.reset();
        removeToken();
        goto("/login");
    };
</script>

<LayoutGrid>
    <Cell span={12}>
        <div class="stats-cell">
            <h1>Welcome Home</h1>
            <div>{data.username}</div>
            <form on:submit|preventDefault={handleSignout}>
                <button type="submit">Signout</button>
            </form>
        </div>
    </Cell>
    <Cell span={4}>
        <div class="for-sale">For Sale</div>
    </Cell>
    <Cell span={4}>
        <div class="sell-it">Sell Something</div>
    </Cell>
</LayoutGrid>

<style>
    .stats-cell {
        height: 100px;
    }
    .for-sale {
        height: 100px;
        background-color: red;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .sell-it {
        height: 100px;
        background-color: green;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
