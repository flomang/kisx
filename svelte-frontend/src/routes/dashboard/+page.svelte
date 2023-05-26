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

    let cards: Card[] = [];
    let searchCategory = "something";
    let searchCondition = "something";
    let term = "term";
    let page = 1;
    let limit = 10;
    let collectionStats = true;
    let filterPanelOpen = false;
    let addPanelOpen = false;

    interface SearchResult {
        // array of lot results
        getLots: LotResult[];
    }

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
                    title
                    externalId
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

    onMount(async () => {
        try {
            const { data } = await client.query<SearchResult>({
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
    });
</script>

<LayoutGrid>
    <Cell span={9}>
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
                    <CollectionStats total_lots={cards.length}/>
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
                    <LotFilter />
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
