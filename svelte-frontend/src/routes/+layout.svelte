<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../lib/apollo";
    import type { PageData } from "./$types";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Textfield from "@smui/textfield";
    import HelperText from "@smui/textfield/helper-text";
    import Icon from "@smui/textfield/icon";
    import { page } from "$app/stores";
    import MenuSurface from "@smui/menu-surface";
    import { Item } from "@smui/list";
    import Button, { Label } from "@smui/button";

    export let data: PageData;
    let surface: MenuSurface;

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
        <div class="left-menu">
            <a href="/home">Home</a>
        </div>

        <div class="search-container">
            <Textfield
                withTrailingIcon={search.length > 0}
                bind:value={search}
                on:keydown={handleKeyDown}
                label="Search"
                class="shaped-outlined"
                variant="outlined"
            >
                <Icon class="material-icons" slot="leadingIcon">search</Icon>
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

        <div class="right-menu">
            <Button variant="raised" on:click={() => surface.setOpen(true)}
                >{data.username}</Button
            >
            <MenuSurface bind:this={surface} anchorCorner="BOTTOM_LEFT">
                <div class="profile-menu">
                    <Item on:SMUI:action={handleSignout}>Logout</Item>
                </div>
            </MenuSurface>
        </div>
    {/if}
</nav>

<slot />

<style>
    nav {
        height: 60px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .right-menu,
    .left-menu {
        display: flex;
        align-items: center;
        margin: 0 10px;
    }

    .profile-menu {
        display: flex;
        flex-direction: column;
        align-items: flex-end;
    }

    .search-container {
        height: 70%;
        flex: 1;
        margin: 10px 10px;
    }

    a {
        font-size: 1.3em;
        color: #40b3ff;
        text-decoration: none;
    }

    * :global(.shaped-outlined) {
        height: 100%;
        width: 70%;
    }
    * :global(.mdc-notched-outline .mdc-notched-outline__leading) {
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
