<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    import { onMount } from "svelte";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Textfield from "@smui/textfield";
    import { Icon as CommonIcon } from "@smui/common";
    import HelperText from "@smui/textfield/helper-text";
    import Select, { Option } from "@smui/select";
    import CharacterCounter from "@smui/textfield/character-counter";
    import Button, { Label } from "@smui/button";
    import LotCollection from './LotCollection.svelte'
    import type { Card } from './LotCollection.svelte'

    let tag = "";
    let category = "";
    let condition = "";
    let description = "";
    let images = [{ imageUrl: "", isThumbnail: true }];
    let meta_data = { quantity: 1 };
    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];
    let pins: Card[] = [];

    let searchCategory = "something";
    let searchCondition = "something";
    let term = "term";
    let page = 1;
    let limit = 10;

    let response;
    let open = false;

    interface LotImage {
        id: string;
        imageUrl: string;
        isThumbnail: boolean;
        createdAt: string;
        updatedAt: string;
    }

    interface LotResult {
        lot: {
            id: string;
            category: string;
            condition: string;
            tag: string;
            description: string;
            meta_data: string;
        };
        images: LotImage[];
    }

    interface LotCreateResult {
        createLot: LotResult;
    }

    interface LotSearchResult {
        getLots: LotResult[];
    }

    onMount(async () => {
        try {
            const { data } = await client.query<LotSearchResult>({
                query: QUERY_LOTS,
                variables: {
                    category: searchCategory,
                    condition: searchCondition,
                    term,
                    page,
                    limit,
                },
            });

            let user_lots = data.getLots;
            // for each lot create a card
            let cards = user_lots.map((lot) => {
                let image = lot.images.find((image: { isThumbnail: any }) => {
                    if (image.isThumbnail) {
                        return true;
                    }
                });

                return {
                    id: lot.lot.id,
                    imageUrl: image ? image.imageUrl : "stock-image.png",
                    title: lot.lot.description,
                    setID: lot.lot.tag,
                };
            });
            pins = cards;
        } catch (error: any) {
            console.log(JSON.stringify(error));
        }
    });

    const CREATE_LOT_MUTATION = gql`
        mutation CreateLot(
            $category: String!
            $condition: String!
            $tag: String!
            $description: String!
            $images: JSON!
            $meta_data: JSON!
        ) {
            createLot(
                params: {
                    category: $category
                    condition: $condition
                    tag: $tag
                    description: $description
                    images: $images
                    metaData: $meta_data
                }
            ) {
                lot {
                    id
                    category
                    condition
                    tag
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

    const QUERY_LOTS = gql`
        query GetLots(
            $category: String!
            $condition: String!
            $term: String!
            $page: Int!
            $limit: Int!
        ) {
            getLots(
                params: {
                    category: $category
                    condition: $condition
                    term: $term
                    page: $page
                    limit: $limit
                }
            ) {
                lot {
                    id
                    category
                    condition
                    tag
                    description
                    metaData
                    createdAt
                    updatedAt
                }
                images {
                    id
                    lotId
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
            const { data } = await client.mutate<LotCreateResult>({
                mutation: CREATE_LOT_MUTATION,
                variables: {
                    category,
                    condition,
                    tag,
                    description,
                    images,
                    meta_data,
                },
            });

            let lot = data?.createLot.lot;
            let image = data?.createLot.images.find(
                (image: { isThumbnail: any }) => {
                    if (image.isThumbnail) {
                        return true;
                    }
                }
            );

            if (lot != null) {
                let card = {
                    id: lot.id,
                    imageUrl: image ? image.imageUrl : "stock-image.png",
                    title: lot.description,
                    setID: lot.tag,
                };
                // append card to end of pins
                pins = [...pins, card];

                // clear values
                category = "";
                condition = "";
                tag = "";
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

<LayoutGrid>
    <Cell span={9}>
        <LotCollection lots={pins} />
    </Cell>

    <Cell span={3}>
        <div class="sell-it">
            <Button on:click={() => (open = !open)}
                ><Label>Toggle Drawer</Label></Button
              >
            <h1>Add Something</h1>
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
                    bind:value={tag}
                >
                    <svelte:fragment slot="label">
                        <CommonIcon
                            class="material-icons"
                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                            >numbers</CommonIcon
                        > Lot Tag
                    </svelte:fragment>
                    <HelperText slot="helper" persistent={true}
                        >title, part name, set number, etc</HelperText
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
                    on:click={handleAddLot}
                >
                    <Label>Mint it!</Label>
                </Button>
            </div>
        </div>
    </Cell>
</LayoutGrid>

<style>
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
        width: 350px;
        float: right;
    }

    .label {
        margin-top: -10px;
        font-size: 0.8em;
    }

    .actions {
        display: flex;
        justify-content: right;
        width: 100%;
    }

    .right {
        float: right;
    }
</style>
