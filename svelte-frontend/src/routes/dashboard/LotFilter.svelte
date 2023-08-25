<script lang="ts">
    import Textfield from "@smui/textfield";
    import { Icon as CommonIcon } from "@smui/common";
    import HelperText from "@smui/textfield/helper-text";
    import Select, { Option } from "@smui/select";
    import Button, { Label } from "@smui/button";
    import { filters as stored_filters, Filter } from "./stores";
    import { onDestroy } from "svelte";

    export let onFilter: (filters: Filter[]) => void;

    let keyword = "";
    let category: string | undefined;
    let condition: string | undefined;
    let status: string | undefined;
    let clicked = 0;
    // TODO populate these from the server
    let statusOptions = ["drafted", "actively listed", "sold", "deleted"];
    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];
    let filters: Filter[];
    const unsubscribeFn = stored_filters.subscribe((value) => {
        filters = value;
    });

    onDestroy(unsubscribeFn);

    $: enabled = keyword != "" || condition != undefined || category != undefined || status != undefined; 

    const handleFilterCollection = async () => {
        let kw = keyword.trim() == "" ? undefined : keyword.trim();

        if (kw != undefined) {
            filters.push(new Filter(filters.length, "keyword", kw));
        }
        if (category != undefined) {
            filters.push(new Filter(filters.length, "category", category));
        }
        if (condition != undefined) {
            filters.push(new Filter(filters.length, "condition", condition));
        }
        if (status != undefined) {
            filters.push(new Filter(filters.length, "status", status));
        }

        stored_filters.set(filters);
        onFilter(filters);

        // reset values
        keyword = "";
        category = undefined;
        condition = undefined;
        status = undefined;

        return true;
    };
</script>

<div class="content">
    <div class="select-container">
        <div class="input-type">
            <Select style="width: 100%;" bind:value={category}>
                <svelte:fragment slot="label">
                    <!-- <CommonIcon
                        class="material-icons"
                        style="font-size: 1em; line-height: normal; vertical-align: top;"
                        >category</CommonIcon
                    > Category -->
                    Category
                </svelte:fragment>
                {#each types.sort() as ty}
                    <Option value={ty}>{ty}</Option>
                {/each}
            </Select>
        </div>
        <div class="input-condition">
            <Select style="width: 100%;" bind:value={condition}>
                <svelte:fragment slot="label">
                    <!-- <CommonIcon
                        class="material-icons"
                        style="font-size: 1em; line-height: normal; vertical-align: top;"
                        >grade</CommonIcon
                    > Condition -->
                    Condition
                </svelte:fragment>
                {#each conditions as cond}
                    <Option value={cond}>{cond}</Option>
                {/each}
            </Select>
        </div>
    </div>
    <div class="select-container">
            <Select style="width: 100%;" bind:value={status}>
                <svelte:fragment slot="label">
                    <!-- <CommonIcon
                        class="material-icons"
                        style="font-size: 1em; line-height: normal; vertical-align: top;"
                        >grade</CommonIcon
                    > Status -->
                    Status
                </svelte:fragment>
                {#each statusOptions.sort() as ty}
                    <Option value={ty}>{ty}</Option>
                {/each}
            </Select>
    </div>
    <div class="input-container">
        <Textfield
            variant="outlined"
            style="width: 100%;"
            class="input-container"
            bind:value={keyword}
        >
            <svelte:fragment slot="label">
                <CommonIcon
                    class="material-icons"
                    style="font-size: 1em; line-height: normal; vertical-align: top;"
                    >search</CommonIcon
                > keyword 
            </svelte:fragment>
            <HelperText slot="helper" persistent={true}
                >title, set number</HelperText
            >
        </Textfield>
    </div>
    <div class="button-container">
        <Button
            variant="raised"
            style="width: 100%; height: 100%;"
            disabled={!enabled}
            on:click={handleFilterCollection}
        >
            <Label>Filter</Label>
        </Button>
    </div>
</div>

<style>
    .content {
        height: 100%;
        width: 100%;
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
</style>
