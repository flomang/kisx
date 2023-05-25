<script lang="ts" context="module">
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
    import Button, { Label } from "@smui/button";
    import ImageList, {
        Item,
        Image,
        Supporting,
        Label as ImageLabel,
    } from "@smui/image-list";
    import IconButton, { Icon } from "@smui/icon-button";
    import Dialog, { Header, Title, Content, Actions } from "@smui/dialog";
    import LayoutGrid, { Cell } from "@smui/layout-grid";

    export let lots: Card[] = [];
    let open = false;
    let response = "";
    let selectedLot: Card;

    function handleOpen(lot: Card) {
        selectedLot = lot;
        open = true;
    }
</script>

<Dialog
    bind:open
    scrimClickAction=""
    escapeKeyAction=""
    aria-labelledby="mandatory-title"
    aria-describedby="mandatory-content"
    fullscreen
>
    {#if selectedLot != null}
        <Header>
            <Title id="over-fullscreen-title">{selectedLot.title}</Title>
            <IconButton action="close" class="material-icons">close</IconButton>
        </Header>
        <Content id="mandatory-content">
            <LayoutGrid>
                <Cell span={9}>
                    <img
                        class="image"
                        src={selectedLot.imageUrl}
                        alt="Image {selectedLot.id}"
                    />
                </Cell>

                <Cell span={3}>
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
            <Button
                on:click={() =>
                    (response = "Wrong answer. Thrown in the lake.")}
            >
                <Label>Sell it!</Label>
            </Button>
        </Actions>
    {/if}
</Dialog>

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
                            <Icon class="material-icons">favorite_border</Icon>
                            <Icon class="material-icons">share</Icon>
                            <Icon class="material-icons">more_vert</Icon>
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

    .actions {
        display: flex;
        justify-content: right;
        width: 100%;
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
</style>
