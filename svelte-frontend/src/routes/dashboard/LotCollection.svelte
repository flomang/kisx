<script lang="ts" context="module">
    import { gql } from "@apollo/client/core";
    import client from "../../lib/apollo";

    export interface Card {
        id: string;
        imageUrl: string;
        title: string;
        setID: string;
        category: string;
        condition: string;
        description: string;
        meta_data: string;
    }

    export interface LotImage {
        id: string;
        imageUrl: string;
        isThumbnail: boolean;
        createdAt: string;
        updatedAt: string;
    }

    export interface LotResult {
        lot: {
            id: string;
            category: string;
            condition: string;
            title: string;
            externalId: string;
            description: string;
            meta_data: string;
        };
        images: LotImage[];
    }
</script>

<script lang="ts">
    import Button from "@smui/button";
    import ImageList, {
        Item,
        Image,
        Supporting,
        Label as ImageLabel,
    } from "@smui/image-list";
    import IconButton, { Icon as ButtonIcon } from "@smui/icon-button";
    import Dialog, { Header, Title, Content, Actions } from "@smui/dialog";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import SegmentedButton, {
        Segment,
        Icon,
        Label,
    } from "@smui/segmented-button";

    import { Svg } from '@smui/common';
    import {
        mdiDelete,
        mdiTextBoxEdit,
    } from "@mdi/js";

    let actions = [
        { name: "Delete", icon: mdiDelete, action: handleConfirm },
        { name: "Edit", icon: mdiTextBoxEdit, action: handleEdit },
    ];

    export let lots: Card[] = [];
    let open = false;
    let confirm = false;
    let selectedLot: Card;

    interface DeleteLotResult {
        deleteLot: number;
    }

    const DELETE_LOT_MUTATION = gql`
        mutation RemoveLot($lotID: String!) {
            deleteLot(lotId: $lotID)
        }
    `;

    const handleDeleteLot = async (lot_id: string) => {
        try {
            const { data } = await client.mutate<DeleteLotResult>({
                mutation: DELETE_LOT_MUTATION,
                variables: {
                    lotID: lot_id,
                },
            });

            if (data?.deleteLot == 1) {
                lots = lots.filter((lot) => lot.id != lot_id);
                confirm = false;
                open = false;
            }
        } catch (error: any) {
            console.log(JSON.stringify(error));
        }
    };

    function handleEdit() {
        alert("edit");
    }

    function handleConfirm() {
        confirm = true;
    }

    function handleOpen(lot: Card) {
        selectedLot = lot;
        open = true;
    }
</script>

{#if selectedLot != null}
    <Dialog
        bind:open
        aria-labelledby="over-fullscreen-title"
        aria-describedby="over-fullscreen-content"
        fullscreen
    >
        <Header>
            <Title id="over-fullscreen-title">{selectedLot.title}</Title>
            <IconButton action="close" class="material-icons">close</IconButton>
        </Header>
        <Content id="over-fullscreen-content">
            <LayoutGrid>
                <Cell span={9}>
                    <img
                        class="image"
                        src={selectedLot.imageUrl}
                        alt="Image {selectedLot.id}"
                    />
                </Cell>

                <Cell span={3}>
                    <div class="action-bar">
                        <SegmentedButton segments={actions} let:segment>
                            <Segment
                                {segment}
                                on:click$preventDefault={() => {
                                    segment.action();
                                }}
                            >
                                <Icon
                                    component={Svg}
                                    style="width: 1em; height: auto;"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        fill="currentColor"
                                        d={segment.icon}
                                    />
                                </Icon>
                                <Label>{segment.name}</Label>
                            </Segment>
                        </SegmentedButton>
                    </div>
                    {#if selectedLot.setID}
                        <div>
                            <b>Set ID:</b>
                            {selectedLot.setID}
                        </div>
                    {/if}
                    <div>
                        <b>Category:</b>
                        {selectedLot.category}
                    </div>
                    <div>
                        <b>Condition:</b>
                        {selectedLot.condition}
                    </div>
                    <div class="description">
                        {selectedLot.description}
                    </div>
               
                </Cell>
            </LayoutGrid>
        </Content>

        <Actions>
        </Actions>
    </Dialog>

    <Dialog
        bind:open={confirm}
        aria-labelledby="over-fullscreen-confirmation-title"
        aria-describedby="over-fullscreen-confirmation-content"
    >
        <!-- Title cannot contain leading whitespace due to mdc-typography-baseline-top() -->
        <Header>
            <Title id="over-fullscreen-confirmation-title">Confirm</Title>
        </Header>
        <Content id="over-fullscreen-confirmation-content">
            Are you sure you want to delete this?
        </Content>
        <Actions>
            <Button>
                <Label>No</Label>
            </Button>
            <Button on:click={() => handleDeleteLot(selectedLot.id)}>
                <Label>Yes</Label>
            </Button>
        </Actions>
    </Dialog>
{/if}

<ImageList class="my-image-list-masonry" masonry>
    {#each lots as lot (lot.id)}
        <Item on:click={() => handleOpen(lot)}>
            <Image src={lot.imageUrl} alt="Image {lot.id}" />
            <Supporting>
                <ImageLabel>
                    <b>{lot.title}</b>
                    {#if lot.setID}
                        <div class="label">
                            {lot.setID}
                        </div>
                    {/if}
                    <div class="actions">
                        <div class="right">
                            <ButtonIcon class="material-icons">favorite_border</ButtonIcon>
                            <ButtonIcon class="material-icons">share</ButtonIcon>
                            <ButtonIcon class="material-icons">more_vert</ButtonIcon>
                        </div>
                    </div>
                </ImageLabel>
            </Supporting>
        </Item>
    {/each}
</ImageList>

<style>
    .label {
        margin-top: -10px;
        font-size: 0.8em;
    }

    .right {
        float: right;
    }

    .image {
        width: 100%;
        height: 100%;
        background-color: #fff;
    }

    .description {
        margin-top: 10px;
    }

    .action-bar {
        display: flex;
        flex-wrap: wrap;
    }

    .action-bar > :global(*) {
        margin-right: 18px;
        margin-bottom: 18px;
    }

    .action-bar :global(svg:focus) {
        outline: 0;
    }
</style>
