<script lang="ts">
    import { gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    import { onMount } from "svelte";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import LotCollection from './LotCollection.svelte'
    import AddLot from "./AddLot.svelte";
    import type { Card, LotResult } from './LotCollection.svelte'

    let cards: Card[] = [];
    let searchCategory = "something";
    let searchCondition = "something";
    let term = "term";
    let page = 1;
    let limit = 10;

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
        <AddLot bind:cards={cards} /> 
    </Cell>
</LayoutGrid>

<style>
</style>
