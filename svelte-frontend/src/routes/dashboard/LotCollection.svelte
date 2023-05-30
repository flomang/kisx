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
        Image as ImageItem,
        Supporting,
        Label as ImageLabel,
    } from "@smui/image-list";
    import IconButton, { Icon as ButtonIcon } from "@smui/icon-button";
    import Dialog, { Header, Title, Content, Actions } from "@smui/dialog";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Textfield from "@smui/textfield";
    import SegmentedButton, {
        Segment,
        Icon,
        Label,
    } from "@smui/segmented-button";
    import { Svg } from "@smui/common";
    import { mdiDelete, mdiTextBoxEdit } from "@mdi/js";
    import Select, { Option } from "@smui/select";
    import { Icon as CommonIcon } from "@smui/common";
    import Dropzone from "svelte-file-dropzone/Dropzone.svelte";

    let files = {
        accepted: [],
        rejected: [],
    };

    function handleFilesSelect(e) {
        console.log(e.detail);
        const { acceptedFiles, fileRejections } = e.detail;
        files.accepted = [...files.accepted, ...acceptedFiles];
        files.rejected = [...files.rejected, ...fileRejections];

        for (let i = 0; i < acceptedFiles.length; i++) {
            const reader = new FileReader();
            reader.addEventListener("load", function () {
                let newImage = new Image();
                newImage.setAttribute("src", reader.result);
                images = [...images, newImage];
            });

            reader.readAsDataURL(acceptedFiles[i]);
        }
    }

    let actions = [
        {
            name: "Delete",
            selected: false,
            icon: mdiDelete,
            action: handleConfirm,
        },
        {
            name: "Edit",
            selected: false,
            icon: mdiTextBoxEdit,
            action: handleToggleEdit,
        },
    ];

    export let lots: Card[] = [];
    let open = false;
    let confirm = false;
    let selectedLot: Card;
    let editableLot: Card;
    let editable = false;
    let images: HTMLImageElement[] = [];
    let thumbnail: HTMLImageElement;

    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];

    interface DeleteLotResult {
        deleteLot: number;
    }

    const DELETE_LOT_MUTATION = gql`
        mutation RemoveLot($lotID: String!) {
            deleteLot(lotId: $lotID)
        }
    `;

    function blurAllElements(): void {
        const activeElement = document.activeElement as HTMLElement;
        if (activeElement !== null) {
            activeElement.blur();
        }
    }

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

    function handleToggleEdit() {
        editable = !editable;
        if (!editable) {
            editableLot = { ...selectedLot };
        }
    }

    function handleCancelEdit() {
        actions[1].selected = false;
        actions = actions;
        editable = false;
        editableLot = { ...selectedLot };
    }

    function handleSaveEdit() {
        actions[1].selected = false;
        actions = actions;
        editable = false;
    }

    function handleConfirm() {
        confirm = true;
    }

    function handleConfirmNo() {
        confirm = false;
    }

    function handleClose() {
        actions[1].selected = false;
        open = false;
        editable = false;
    }

    function handleOpen(lot: Card) {
        selectedLot = { ...lot };
        // shallow copy lot
        editableLot = { ...lot };
        // editableLot = lot;
        open = true;

        let image = new Image();
        image.setAttribute("src", editableLot.imageUrl);
        thumbnail = image;
        images = [image];
    }

    function handleSelectThumbnail(image: HTMLImageElement) {
        thumbnail = image;
    }
</script>

{#if editableLot != null}
    <Dialog
        bind:open
        aria-labelledby="over-fullscreen-title"
        aria-describedby="over-fullscreen-content"
        fullscreen
        on:SMUIDialog:closed={handleClose}
    >
        <Header>
            <Title id="over-fullscreen-title">
                {editableLot.title}
            </Title>
            <IconButton action="close" class="material-icons">close</IconButton>
        </Header>
        <Content id="over-fullscreen-content">
            <LayoutGrid>
                <Cell span={9}>
                    <div class="lot-thumb">
                        <img
                            id="thumbnail"
                            src={thumbnail.src}
                            alt="Image {editableLot.id}"
                        />
                    </div>
                    <div class="lot-images">
                        {#each images as image}
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            <img
                                on:click={() => handleSelectThumbnail(image)}
                                class="lot-image"
                                src={image.src}
                                alt="Image {editableLot.id}"
                            />
                        {/each}
                     
                    </div>
                </Cell>

                <Cell span={3}>
                    <div class="detail">
                        <div class="action-bar">
                            <SegmentedButton
                                segments={actions}
                                let:segment
                                key={(segment) => segment.name}
                            >
                                <Segment
                                    {segment}
                                    selected={segment.selected}
                                    on:click$preventDefault={(event) => {
                                        actions = actions;
                                        if (segment.name != "Delete") {
                                            segment.selected =
                                                !segment.selected;
                                        }
                                        segment.action();
                                        blurAllElements();
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
                        <div class="lot-input">
                            <Textfield
                                variant="outlined"
                                style="width: 100%;"
                                disabled={!editable}
                                bind:value={editableLot.title}
                                ><svelte:fragment slot="label">
                                    <CommonIcon
                                        class="material-icons"
                                        style="font-size: 1em; line-height: normal; vertical-align: top;"
                                        >title</CommonIcon
                                    > Title
                                </svelte:fragment>
                            </Textfield>
                        </div>
                        {#if editableLot.setID}
                            <div class="lot-input">
                                <Textfield
                                    variant="outlined"
                                    style="width: 100%;"
                                    disabled={!editable}
                                    bind:value={editableLot.setID}
                                    ><svelte:fragment slot="label">
                                        <CommonIcon
                                            class="material-icons"
                                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                                            >numbers</CommonIcon
                                        > Set Number
                                    </svelte:fragment>
                                </Textfield>
                            </div>
                        {/if}
                        <div class="lot-input">
                            <Select
                                style="width: 100%;"
                                bind:value={editableLot.category}
                                variant="outlined"
                                disabled={!editable}
                            >
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
                        <div class="lot-input">
                            <Select
                                style="width: 100%;"
                                bind:value={editableLot.condition}
                                variant="outlined"
                                disabled={!editable}
                            >
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
                        <div class="lot-description">
                            <Textfield
                                textarea
                                disabled={!editable}
                                style="width: 100%; height: 100%;"
                                bind:value={editableLot.description}
                            >
                                <svelte:fragment slot="label">
                                    <CommonIcon
                                        class="material-icons"
                                        style="font-size: 1em; line-height: normal; vertical-align: top;"
                                        >subject</CommonIcon
                                    > Description
                                </svelte:fragment>
                            </Textfield>
                        </div>
                        {#if editable}
                            <div class="lot-dropzone">
                                <Dropzone
                                    on:drop={handleFilesSelect}
                                    accept={["image/*"]}
                                    containerClasses="lot-dropzone"
                                    inputElement
                                />
                            </div>

                            <div class="lot-description-edit-action">
                                <Button on:click={handleCancelEdit}>
                                    <Label>Cancel</Label>
                                </Button>
                                <Button on:click={handleSaveEdit}>
                                    <Label>Save</Label>
                                </Button>
                            </div>
                        {/if}
                    </div>
                </Cell>
            </LayoutGrid>
        </Content>

        <Actions />
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
            <Button on:click={handleConfirmNo}>
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
            <ImageItem src={lot.imageUrl} alt="Image {lot.id}" />
            <Supporting>
                <ImageLabel>
                    <b>{lot.title}</b>
                    {#if lot.setID}
                        <div class="label">
                            {lot.setID}
                        </div>
                    {/if}
                    <div class="value">
                        <img src="eth-symbol-virgil.svg" alt="" />3000.0
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

    .lot-thumb {
        height: 500px;
        width: 700px;
    }

    #thumbnail {
        height: 100%;
        width: 100%;
        object-fit: contain;
    }

    .detail {
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .lot-input {
        margin-top: 10px;
        width: 100%;
        display: flex;
    }

    .lot-description {
        margin-top: 10px;
        width: 100%;
        display: flex;
        flex: 1;
    }

    .lot-dropzone {
        width: 100%;
        height: 50%;
        align-items: center;
        text-align: center;
        margin-top: 10px;
    }

    .lot-description-edit-action {
        display: flex;
        flex-direction: row;
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

    .value {
        display: flex;
        background-color: green;
        float: right;
        padding-inline: 5px;
        margin-bottom: 10px;
        border-radius: 5px;
        color: #fff;
        font-weight: bold;

        box-shadow: 0 1px 1px 0 rgba(0, 0, 0, 0.14),
            0 2px 1px -1px rgba(0, 0, 0, 0.12), 0 1px 3px 0 rgba(0, 0, 0, 0.2);
    }

    .value img {
        margin-top: 5px;
        margin-right: 5px;
        width: 17px;
        height: 17px;
        filter: invert(100%);
    }

    .lot-images {
        display: flex;
        flex-direction: row;
        overflow-wrap: normal;
        overflow-x: auto;
        margin-bottom: 5px;
    }

   

    .lot-image {
        /* margin: 10px; Adjust as needed */
        object-fit: contain;

        margin-right: 10px;
        height: 100px;
        width: 100px;
    }
</style>
