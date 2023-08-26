<script lang="ts">
    import { gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    import { onMount } from "svelte";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import LotCollection from "./LotCollection.svelte";
    import AddLot from "./AddLot.svelte";
    import type { Card, LotResult } from "./LotCollection.svelte";
    import LotFilter from "./LotFilter.svelte";
    import CollectionStats from "./CollectionStats.svelte";
    import Accordion, { Panel, Header, Content } from "@smui-extra/accordion";
    import IconButton, { Icon } from "@smui/icon-button";
    import Chip, { Set, TrailingAction, Text } from "@smui/chips";
    import { filters, Filter } from "./stores";

    let cards: Card[] = [];
    let collectionStats = true;
    let filterPanelOpen = false;
    let addPanelOpen = false;

    interface SearchResult {
        // array of lot results
        getLots: LotResult[];
    }

    const QUERY_LOTS = gql`
        query GetLots(
            $categories: [String]!
            $conditions: [String]!
            $terms: [String]!
            $page: Int!
            $limit: Int!
            $statuses: [String]!
        ) {
            getLots(
                params: {
                    categories: $categories
                    conditions: $conditions
                    terms: $terms
                    page: $page
                    limit: $limit
                    statuses: $statuses
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
                    createdAt
                    updatedAt
                    status
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

    async function handleFilterCollection(filters: Filter[], page: number = 1, limit: number = 10) {
        try {
            let categories: String[] = [];
            let conditions: String[] = [];
            let keywords: String[] = [];
            let statuses: String[] = [];

            if (filters.length > 0) {
                categories = filters
                    .filter((f: Filter) => f.type == "category")
                    .map((f: Filter) => f.value);
                conditions = filters
                    .filter((f: Filter) => f.type == "condition")
                    .map((f: Filter) => f.value);
                keywords = filters
                    .filter((f: Filter) => f.type == "keyword")
                    .map((f: Filter) => f.value);
                statuses = filters
                    .filter((f: Filter) => f.type == "status")
                    .map((f: Filter) => f.value);

                // console.log("conditions", conditions);
                // console.log("keywords", keywords);
                // console.log("statuses", statuses);
                // console.log("categories", categories);
            }
            console.log("filtering collection", filters);

            const { data } = await client.query<SearchResult>({
                query: QUERY_LOTS,
                variables: {
                    categories,
                    conditions,
                    terms: keywords,
                    page,
                    limit,
                    statuses,
                },
            });

            let user_lots = data.getLots;

            // map lots to cards
            cards = user_lots.map((lot) => {
                let image = lot.images.find((image: { isThumbnail: any }) => {
                    if (image.isThumbnail) {
                        return true;
                    }
                });

                return {
                    id: lot.lot.id,
                    imageUrl: image ? image.imageUrl : "stock-image.png",
                    title: lot.lot.title,
                    setID: lot.lot.externalId,
                    category: lot.lot.category,
                    condition: lot.lot.condition,
                    description: lot.lot.description,
                    meta_data: lot.lot.meta_data,
                };
            });
        } catch (error: any) {
            console.log(JSON.stringify(error));
        }
    }

    function handleDeleteChip(chip: string) {
        // remove chip from filters
        let new_filters = $filters.filter((f: Filter) => f.toString() != chip);
        handleFilterCollection(new_filters);
    }

    onMount(async () => {
        handleFilterCollection($filters);
    });
</script>

<LayoutGrid>
    <Cell span={9}>
        <Set chips={$filters} let:chip key={(chip) => chip.key} input>
            <Chip {chip}>
                <Text>{chip.toString()}</Text>
                <TrailingAction
                    icon$class="material-icons"
                    on:click={() => handleDeleteChip(chip.toString())}
                    >cancel</TrailingAction
                >
            </Chip>
        </Set>
        <LotCollection lots={cards} />
    </Cell>

    <Cell span={3}>
        <Accordion multiple>
            <Panel bind:open={collectionStats}>
                <Header
                    ><b>Collection Stats</b>
                    <IconButton slot="icon" toggle pressed={collectionStats}>
                        <Icon class="material-icons" on>expand_less</Icon>
                        <Icon class="material-icons">expand_more</Icon>
                    </IconButton>
                </Header>
                <Content>
                    <CollectionStats total_lots={cards.length} />
                </Content>
            </Panel>
            <Panel bind:open={filterPanelOpen}>
                <Header
                    ><b>Filter Collection</b>
                    <IconButton slot="icon" toggle pressed={filterPanelOpen}>
                        <Icon class="material-icons" on>expand_less</Icon>
                        <Icon class="material-icons">expand_more</Icon>
                    </IconButton>
                </Header>
                <Content>
                    <LotFilter onFilter={handleFilterCollection} />
                </Content>
            </Panel>
            <Panel bind:open={addPanelOpen}>
                <Header
                    ><b>Add Something</b>
                    <IconButton slot="icon" toggle pressed={addPanelOpen}>
                        <Icon class="material-icons" on>expand_less</Icon>
                        <Icon class="material-icons">expand_more</Icon>
                    </IconButton>
                </Header>
                <Content>
                    <AddLot bind:cards />
                </Content>
            </Panel>
        </Accordion>
    </Cell>
</LayoutGrid>

<style>
</style>
