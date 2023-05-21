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
    let ignore = ["/login", "/signup", "/forgot"];
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

<nav class={!ignore.includes($page.route.id) ? "mdc-theme--primary-bg" : ""}>
    <!-- only show if logged in -->
    {#if $page.route.id != "/login"}
        <div class="left-menu">
            <a
                class={$page.route.id === "/home"
                    ? "selected mdc-elevation--z12"
                    : "selectable"}
                href="/home">KISX</a
            >
        </div>
        <div class="left-menu">
            <a
                class={$page.route.id === "/about"
                    ? "selected mdc-elevation--z12"
                    : "selectable"}
                href="/about">ABOUT</a
            >
        </div>
        <div class="left-menu">
            <a
                class={$page.route.id === "/dashboard"
                    ? "selected mdc-elevation--z12"
                    : "selectable"}
                href="/dashboard">DASHBOARD</a
            >
        </div>
        <div class="left-menu">
            <a
                class={$page.route.id === "/news"
                    ? "selected mdc-elevation--z12"
                    : "selectable"}
                href="/news">NEWS</a
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
                <Icon
                    color="secondary"
                    class="material-icons"
                    slot="leadingIcon">search</Icon
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

        {#if data.username}
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
    }

    a {
        font-size: 1.3em;
        color: #fff;
        text-decoration: none;
    }

    .selected {
        background-color: #da291c;
        color: #fff;
    }

    .selectable {
        color: #000;
    }

    .selected,
    .selectable {
        border-radius: 4px;
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 10px;
        font-size: 0.9em;
        font-weight: bold;
    }
</style>
