<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../lib/apollo";
    import type { PageData } from "./$types";
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

<nav class={$page.route.id != "/login" ? "mdc-theme--primary-bg" : ""}>
    <!-- only show if logged in -->
    {#if $page.route.id != "/login"}
        <div class="left-menu">
            <a
                class="my-primary mdc-elevation-transition rounded flexy-boy {$page
                    .route.id === '/home'
                    ? 'selected'
                    : ''}"
                class:mdc-elevation--z4={$page.route.id == "/home"}
                href="/home">kisx</a
            >
        </div>
        <div class="left-menu">
            <a
                class="my-primary mdc-elevation-transition rounded flexy-boy {$page
                    .route.id === '/about'
                    ? 'selected'
                    : ''}"
                class:mdc-elevation--z4={$page.route.id == "/about"}
                href="/about">about</a
            >
        </div>
        <div class="left-menu">
            <a
                class="my-primary mdc-elevation-transition rounded flexy-boy {$page
                    .route.id === '/dashboard'
                    ? 'selected'
                    : ''}"
                class:mdc-elevation--z4={$page.route.id == "/dashboard"}
                href="/dashboard">dashboard</a
            >
        </div>
        <div class="left-menu">
            <a
                class="my-primary mdc-elevation-transition rounded flexy-boy {$page
                    .route.id === '/news'
                    ? 'selected'
                    : ''}"
                class:mdc-elevation--z4={$page.route.id == "/news"}
                href="/news">news</a
            >
        </div>

        <div class="search-container">
                <Textfield
                    withTrailingIcon={search.length > 0}
                    bind:value={search}
                    on:keydown={handleKeyDown}
                    label="Set Number, Minifigure, Name, Description"
                    class="search-text-field"
                    variant="outlined"
                >
                    <Icon class="material-icons" slot="leadingIcon">search</Icon
                    >
                    <svelte:fragment slot="trailingIcon">
                        {#if search.length > 0}
                            <Icon class="material-icons" slot="trailingIcon"
                                >keyboard_return</Icon
                            >
                        {/if}
                    </svelte:fragment>
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
        align-items: center;
        /* justify-content: space-between; */
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
        flex: 1;
        height: 65%;
        display: flex;
    }

    a {
        font-size: 1.3em;
        color: #fff;
        text-decoration: none;
    }

    /* * :global(.shaped-outlined) {
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
    } */

    .selected {
        background-color: #fa8072;
    }
    .flexy-boy {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 10px;
    }

    .rounded {
        border-radius: 4px;
    }
</style>
