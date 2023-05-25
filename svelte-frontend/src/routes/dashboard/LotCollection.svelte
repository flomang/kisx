<script lang="ts" context="module">
    export interface Card {
        id: string;
        imageUrl: string;
        title: string;
        setID: string;
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

    export let lots: Card[] = [];
    let open = false;
    let response = "";
</script>


<Dialog
    bind:open
    scrimClickAction=""
    escapeKeyAction=""
    aria-labelledby="mandatory-title"
    aria-describedby="mandatory-content"
    fullscreen
>
    <Header>
        <Title id="over-fullscreen-title">Item Info</Title>
        <IconButton action="close" class="material-icons">close</IconButton>
    </Header>
    <Content id="mandatory-content">
        Before you continue on this page, you must answer my riddle of age. When
        Alice was six her brother was half, now Alice is 90, you do the math.
        <br /><br />
        How old is Alice's brother now?
    </Content>
    <Actions>
        <Button
            on:click={() => (response = "Wrong answer. Thrown in the lake.")}
        >
            <Label>Sell it!</Label>
        </Button>
    </Actions>
</Dialog>

<ImageList class="my-image-list-masonry" masonry>
    {#each lots as lot (lot.id)}
        <Item on:click={() => (open = true)}>
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
</style>
