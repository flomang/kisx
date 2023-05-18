<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../lib/apollo";
    import type { PageData } from "./$types";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Textfield from "@smui/textfield";
    import HelperText from "@smui/textfield/helper-text";
    import Icon from "@smui/textfield/icon";
    import { page } from "$app/stores";

    export let data: PageData;
    let search = "";

    const handleSignout = async () => {
        client.cache.reset();
        removeToken();
        goto("/login");
        data = {};
    };

    function doSearch() {
        alert("Search for " + search);
    }

    function handleKeyDown(event: CustomEvent | KeyboardEvent) {
        event = event as KeyboardEvent;
        if (event.key === "Enter") {
            doSearch();
        }
    }
</script>

<nav>
    <!-- only show if logged in -->
    {#if $page.route.id != "/login"}
        <LayoutGrid>
            <Cell span={4}>
                <div class="left-menu">
                    <a href="/home">Home</a>
                </div>
            </Cell>

            <Cell span={4}>
                <div class="search-container">
                    <Textfield
                        withTrailingIcon={search.length > 0}
                        bind:value={search}
                        on:keydown={handleKeyDown}
                        style="width: 100%;"
                        label="Search"
                        class="shaped-outlined"
                        variant="outlined"
                    >
                        <Icon class="material-icons" slot="leadingIcon"
                            >search</Icon
                        >
                        <svelte:fragment slot="trailingIcon">
                            {#if search.length > 0}
                                <Icon class="material-icons" slot="trailingIcon"
                                    >keyboard_return</Icon
                                >
                            {/if}
                        </svelte:fragment>

                        <HelperText slot="helper"
                            >Set Number, Minifigure, Name, Description</HelperText
                        >
                    </Textfield>
                </div>
            </Cell>
            <Cell span={4}>
                <div class="right-menu">
                    {#if data.username}
                        <div class="username">{data.username}</div>
                        <form on:submit|preventDefault={handleSignout}>
                            <button type="submit">Signout</button>
                        </form>
                    {:else}
                        <a href="/login">Login</a>
                    {/if}
                </div>
            </Cell>
        </LayoutGrid>
    {/if}
</nav>

<slot />

<style>
    .left-menu {
        display: flex;
        align-items: center;
        height: 20px;
    }

    .right-menu {
        height: 20px;
        display: flex;
        align-items: center;
        float: right;
    }

    .username {
        padding-inline: 10px;
        padding-block: 2px;
    }

    .search-container {
        width: 100%;
        margin-top: -10px;
    }

    a {
        font-size: 1.3em;
        color: #40b3ff;
        text-decoration: none;
    }

    *
        :global(
            .shaped-outlined .mdc-notched-outline .mdc-notched-outline__leading
        ) {
        border-radius: 28px 0 0 28px;
        width: 28px;
    }
    *
        :global(
            .shaped-outlined .mdc-notched-outline .mdc-notched-outline__trailing
        ) {
        border-radius: 0 28px 28px 0;
    }
    *
        :global(
            .shaped-outlined .mdc-notched-outline .mdc-notched-outline__notch
        ) {
        max-width: calc(100% - 28px * 2);
    }
    *
        :global(
            .shaped-outlined.mdc-text-field--with-leading-icon:not(
                    .mdc-text-field--label-floating
                )
                .mdc-floating-label
        ) {
        left: 16px;
    }
    * :global(.shaped-outlined + .mdc-text-field-helper-line) {
        padding-left: 32px;
        padding-right: 28px;
    }
</style>
