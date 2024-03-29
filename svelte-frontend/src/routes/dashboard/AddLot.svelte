<script lang="ts">
    import { gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    import Textfield from "@smui/textfield";
    import { Icon as CommonIcon } from "@smui/common";
    import HelperText from "@smui/textfield/helper-text";
    import Select, { Option } from "@smui/select";
    import CharacterCounter from "@smui/textfield/character-counter";
    import Button, { Label } from "@smui/button";
    import type { Card, LotResult } from "./LotCollection.svelte";

    export let cards: Card[] = [];

    let title = "";
    let setNumber = "";
    let category = "";
    let condition = "";
    let description = "";
    let images = [{ imageUrl: "", isThumbnail: true }];
    let meta_data = { quantity: 1 };
    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];

    $: enabled = title != "" && category != undefined && condition != undefined && description != ""; 

    interface CreateLotResult {
        createLot: LotResult;
    }

    const CREATE_LOT_MUTATION = gql`
        mutation CreateLot(
            $category: String!
            $condition: String!
            $title: String!
            $externalId: String!
            $description: String!
            $images: JSON!
            $meta_data: JSON!
        ) {
            createLot(
                params: {
                    category: $category
                    condition: $condition
                    title: $title
                    externalId: $externalId
                    description: $description
                    images: $images
                    metaData: $meta_data
                }
            ) {
                lot {
                    id
                    category
                    condition
                    title
                    externalId
                    description
                    metaData
                }
                images {
                    id
                    imageUrl
                    isThumbnail
                    createdAt
                    updatedAt
                }
            }
        }
    `;

    const handleAddLot = async () => {
        try {
            const { data } = await client.mutate<CreateLotResult>({
                mutation: CREATE_LOT_MUTATION,
                variables: {
                    category,
                    condition,
                    title,
                    externalId: setNumber,
                    description,
                    images,
                    meta_data,
                },
            });

            let lot = data?.createLot.lot;
            if (lot != null) {
                let image = data?.createLot.images.find(
                    (image: { isThumbnail: any }) => {
                        if (image.isThumbnail) {
                            return true;
                        }
                    }
                );

                let card = {
                    id: lot.id,
                    imageUrl: image ? image.imageUrl : "stock-image.png",
                    title: lot.title,
                    setID: lot.externalId,
                    category: lot.category,
                    condition: lot.condition,
                    description: lot.description,
                    meta_data: lot.meta_data,
                };
                // append card to end of pins
                cards = [...cards, card];

                // clear values
                title = "";
                category = "";
                condition = "";
                setNumber = "";
                description = "";
                images = [{ imageUrl: "", isThumbnail: true }];
                meta_data = { quantity: 1 };
            }
        } catch (error: any) {
            console.log(JSON.stringify(error));
        }
    };

    function handleQuantityInput(event: any) {
        const newValue = parseInt(event.target.value, 10);
        if (newValue < 1) {
            meta_data.quantity = 1; // Set minimum value as 1
        } else {
            meta_data.quantity = newValue;
        }
    }
</script>

<div class="content">
    <div class="select-container">
        <div class="input-type">
            <Select style="width: 100%;" bind:value={category}>
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
            bind:value={title}
        >
            <svelte:fragment slot="label">
                <CommonIcon
                    class="material-icons"
                    style="font-size: 1em; line-height: normal; vertical-align: top;"
                    >title</CommonIcon
                > Title 
            </svelte:fragment>
            <HelperText slot="helper" persistent={true}
                >title of set or part</HelperText
            >
        </Textfield>
    </div>
    <div class="input-container">
        <Textfield
            variant="outlined"
            style="width: 100%;"
            class="input-container"
            bind:value={setNumber}
        >
            <svelte:fragment slot="label">
                <CommonIcon
                    class="material-icons"
                    style="font-size: 1em; line-height: normal; vertical-align: top;"
                    >numbers</CommonIcon
                > Set Number
            </svelte:fragment>
            <HelperText slot="helper" persistent={true}
                >set number or custom tag</HelperText
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
            <CharacterCounter slot="internalCounter">0 / 100</CharacterCounter>
        </Textfield>
    </div>
    
    <div class="input-container">
        <Textfield
            variant="outlined"
            style="width: 100%;"
            class="input-container"
            bind:value={images[0].imageUrl}
        >
            <svelte:fragment slot="label">
                <CommonIcon
                    class="material-icons"
                    style="font-size: 1em; line-height: normal; vertical-align: top;"
                    >image</CommonIcon
                > thumb url
            </svelte:fragment>
        </Textfield>
    </div>
    <div>
        <Textfield
            bind:value={meta_data.quantity}
            label="Quantity"
            type="number"
            on:input={handleQuantityInput}
        />
    </div>

    <div class="button-container">
        <Button
            variant="raised"
            style="width: 100%; height: 100%;"
            disabled={!enabled}
            on:click={handleAddLot}
        >
            <Label>Add it!</Label>
        </Button>
    </div>
</div>

<style>
    .content {
        width: 100%;
        height: 100%;
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
