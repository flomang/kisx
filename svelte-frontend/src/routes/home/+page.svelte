<script lang="ts">
    import { goto } from "$app/navigation";
    import client, { removeToken } from "../../lib/apollo";
    import type { PageData } from "./$types";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Textfield from "@smui/textfield";
    import FormField from "@smui/form-field";
    import Checkbox from "@smui/checkbox";
    import { Icon as CommonIcon } from "@smui/common";
    import HelperText from "@smui/textfield/helper-text";
    import Select, { Option } from "@smui/select";
    import CharacterCounter from "@smui/textfield/character-counter";
    import Button, { Label } from "@smui/button";

    export let data: PageData;
    let name = "";
    let lotType = "";
    let condition = "";
    let description = "";
    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];

    const handleSignout = async () => {
        client.cache.reset();
        removeToken();
        goto("/login");
    };
</script>

<LayoutGrid>
    <Cell span={6}>
        <div class="left-menu">
            <h1>Welcome Home</h1>
        </div>
    </Cell>
    <Cell span={6}>
        <div class="right-menu">
            <div class="username">{data.username}</div>
            <form on:submit|preventDefault={handleSignout}>
                <button type="submit">Signout</button>
            </form>
        </div>
    </Cell>
    <Cell span={12}>
        <div class="sell-it">
            <h1>Mint Something</h1>
            <div class="select-container">
                <div class="input-type">
                    <Select style="width: 100%;" bind:value={lotType}>
                        <svelte:fragment slot="label">
                            <CommonIcon
                                class="material-icons"
                                style="font-size: 1em; line-height: normal; vertical-align: top;"
                                >category</CommonIcon
                            > Category
                        </svelte:fragment>
                        {#each types.sort() as ty}
                            <Option value={ty}>{ty}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="input-condition">
                    <Select style="width: 100%;" bind:value={condition}>
                        <svelte:fragment slot="label">
                            <CommonIcon
                                class="material-icons"
                                style="font-size: 1em; line-height: normal; vertical-align: top;"
                                >grade</CommonIcon
                            > Condition
                        </svelte:fragment>
                        {#each conditions as cond}
                            <Option value={cond}>{cond}</Option>
                        {/each}
                    </Select>
                </div>
            </div>
            <div class="input-container">
                <Textfield
                    variant="outlined"
                    style="width: 100%;"
                    class="input-container"
                    bind:value={name}
                >
                    <svelte:fragment slot="label">
                        <CommonIcon
                            class="material-icons"
                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                            >numbers</CommonIcon
                        > Lot Tag
                    </svelte:fragment>
                    <HelperText slot="helper" persistent={true}
                        >title, part name, set numer, etc</HelperText
                    >
                </Textfield>
            </div>

            <div class="input-container">
                <Textfield
                    textarea
                    input$maxlength={500}
                    style="width: 100%;"
                    bind:value={description}
                >
                    <svelte:fragment slot="label">
                        <CommonIcon
                            class="material-icons"
                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                            >subject</CommonIcon
                        > Description
                    </svelte:fragment>
                    <CharacterCounter slot="internalCounter"
                        >0 / 100</CharacterCounter
                    >
                </Textfield>
            </div>
            <div class="button-container">
                <Button variant="raised" style="width: 100%; height: 100%;">
                    <Label>Mint it!</Label>
                </Button>
            </div>
        </div>
    </Cell>
</LayoutGrid>

<style>
    .left-menu {
        height: 20px;
        display: flex;
        justify-content: left;
        align-items: center;
    }
    .right-menu {
        height: 20px;
        display: flex;
        justify-content: right;
    }
    .username {
        padding-inline: 10px;
        padding-block: 2px;
    }

    .select-container {
        display: flex;
        padding-block: 10px;
    }

    .input-type {
        padding-right: 3%;
        width: 47%;
    }
    .input-condition {
        width: 47%;
        padding-left: 3%;
    }
    .input-container {
        width: 100%;
        padding-block: 4px;
    }

    .button-container {
        display: flex;
        justify-content: center;
        width: 100%;
        padding-block: 16px;
        height: 45px;
    }
    .sell-it {
        height: 100px;
        width: 500px;
        margin: auto;
        /* background-color: green; */
    }
</style>
