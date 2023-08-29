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
        status: string;
        price: number;
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
            status: string;
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
    import { mdiDelete, mdiTextBoxEdit, mdiImage, mdiTrashCan } from "@mdi/js";
    import Select, { Option } from "@smui/select";
    import { Icon as CommonIcon } from "@smui/common";
    import Dropzone from "svelte-file-dropzone/Dropzone.svelte";

    interface Files {
        accepted: File[];
        rejected: File[];
    }

    let files: Files = {
        accepted: [],
        rejected: [],
    };

    function handleFilesSelect(e: any) {
        const { acceptedFiles, fileRejections } = e.detail;
        files.accepted = [...files.accepted, ...acceptedFiles];
        files.rejected = [...files.rejected, ...fileRejections];

        for (let i = 0; i < acceptedFiles.length; i++) {
            const reader = new FileReader();
            reader.addEventListener("load", function () {
                let newImage = new Image();
                newImage.setAttribute("src", reader.result as string);
                images = [...images, newImage];
            });

            reader.readAsDataURL(acceptedFiles[i]);
        }
    }

    let actions: Action[] = [
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

    interface Action {
        name: string;
        selected: boolean;
        icon: string;
        action: () => void;
    }

    // for adding images
    // only available when editing
    let addAction = {
        name: "Add",
        selected: false,
        icon: mdiImage,
        action: () => {},
    };

    export let lots: Card[] = [];
    let open = false;
    let confirm = false;
    let selectedLot: Card;
    let editableLot: Card;
    let editable = false;
    let images: HTMLImageElement[] = [];
    let thumbnail: HTMLImageElement;

    // TODO read these from the server
    const statuses = ["drafted", "for sale"];
    const types = ["box", "set", "instructions", "minifig", "part", "custom"];
    const conditions = [
        "sealed",
        "complete",
        "used",
        "missing pieces",
        "other",
    ];
    const noneditable = ["sold", "pending sale"];

    interface DeleteLotResult {
        deleteLot: number;
    }

    const DELETE_LOT_MUTATION = gql`
        mutation RemoveLot($lotID: String!) {
            deleteLot(lotId: $lotID)
        }
    `;

    const UPDATE_LOT_MUTATION = gql`
        mutation UpdateLot(
            $lotID: String!
            $category: String
            $condition: String
            $status: String
            $title: String
            $externalId: String
            $description: String
            $deleteImageIDs: JSON
        ) {
            updateLot(
                params: {
                    lotId: $lotID
                    category: $category
                    condition: $condition
                    title: $title
                    externalId: $externalId
                    description: $description
                    status: $status
                    deletedImageIds: $deleteImageIDs
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
                    status
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
            handleCancelEdit();
        } else {
            actions.push(addAction);
        }
    }

    function handleCancelEdit() {
        editableLot = { ...selectedLot };
        exitEdit();
    }

    function exitEdit() {
        editable = false;
        // remove add image button
        actions = actions.filter((action) => action.name != "Add");
        // unselect edit button
        actions[1].selected = false;
    }

    async function handleSaveEdit() {
        selectedLot = { ...editableLot };
        exitEdit();

        try {
            const { data } = await client.mutate<LotResult>({
                mutation: UPDATE_LOT_MUTATION,
                variables: {
                    lotID: editableLot.id,
                    category: editableLot.category,
                    condition: editableLot.condition,
                    status: editableLot.status,
                    title: editableLot.title,
                    externalId: editableLot.setID,
                    description: editableLot.description,
                    deleteImageIDs: [],
                },
            });

            console.log(JSON.stringify(data));
        } catch (error: any) {
            console.log(JSON.stringify(error));
        }
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

    function handleDeleteImage(Image: HTMLImageElement) {
        images = images.filter((img) => img.src != Image.src);
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
                <Cell span={7}>
                    <div class="image-showcase">
                        <div class="lot-thumb">
                            <img
                                id="thumbnail"
                                src={thumbnail.src}
                                alt="Image {editableLot.id}"
                            />
                        </div>
                        <div class="lot-images">
                            {#each images as image}
                                <div class="image-container">
                                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                                    <img
                                        on:click={() =>
                                            handleSelectThumbnail(image)}
                                        class="lot-image"
                                        src={image.src}
                                        alt="Image {editableLot.id}"
                                    />
                                    {#if editable}
                                        <div class="delete-button">
                                            <IconButton
                                                class="material-icons"
                                                on:click={() =>
                                                    handleDeleteImage(image)}
                                                size="button"
                                            >
                                                <Icon
                                                    component={Svg}
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path
                                                        fill="currentColor"
                                                        d={mdiTrashCan}
                                                    />
                                                </Icon>
                                            </IconButton>
                                        </div>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </div>
                </Cell>

                <Cell span={5}>
                    <div class="detail">
                        <div class="action-bar">
                            {#if !noneditable.includes(selectedLot.status)}
                                <SegmentedButton
                                    segments={actions}
                                    let:segment
                                    key={(segment) => segment.name}
                                >
                                    <Segment
                                        {segment}
                                        selected={segment.selected}
                                        on:click={(event) => {
                                            actions = actions;
                                            if (
                                                segment.name != "Delete" &&
                                                segment.name != "Add"
                                            ) {
                                                segment.selected =
                                                    !segment.selected;
                                            }
                                            if (segment.action != null)
                                                segment.action();

                                            blurAllElements();
                                        }}
                                    >
                                        {#if segment.name == "Add"}
                                            <Dropzone
                                                on:drop={handleFilesSelect}
                                                accept={["image/*"]}
                                                containerClasses="add-dropzone"
                                                disableDefaultStyles={true}
                                                inputElement
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
                                            </Dropzone>
                                        {:else}
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
                                        {/if}
                                    </Segment>
                                </SegmentedButton>
                            {/if}
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
                        <div class="lot-input">
                            {#if !noneditable.includes(editableLot.status)}
                                <Select
                                    style="width: 100%;"
                                    bind:value={editableLot.status}
                                    variant="outlined"
                                    disabled={!editable}
                                >
                                    <svelte:fragment slot="label">
                                        <CommonIcon
                                            class="material-icons"
                                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                                            >priority_high</CommonIcon
                                        > Status
                                    </svelte:fragment>
                                    {#each statuses.sort() as ty}
                                        <Option value={ty}>{ty}</Option>
                                    {/each}
                                </Select>
                            {:else}
                                <Textfield
                                    variant="outlined"
                                    style="width: 100%;"
                                    disabled={!editable}
                                    bind:value={editableLot.status}
                                    ><svelte:fragment slot="label">
                                        <CommonIcon
                                            class="material-icons"
                                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                                            >priority_high</CommonIcon
                                        > Status
                                    </svelte:fragment>
                                </Textfield>
                            {/if}
                        </div>
                        {#if editableLot.status == "for sale"}
                            <div class="lot-input">
                                <Textfield
                                    variant="outlined"
                                    style="width: 100%;"
                                    disabled={!editable}
                                    bind:value={editableLot.price}
                                    ><svelte:fragment slot="label">
                                       ETH Price
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
                    <div class={`status ${lot.status.replace(" ", "_")}`}>
                        {lot.status}
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
        height: 100%;
        width: 100%;
    }

    #thumbnail {
        background-color: #fff;
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

    .for_sale {
        display: flex;
        background-color: red;
        float: right;
        padding-inline: 5px;
        margin-bottom: 10px;
        border-radius: 5px;
        color: #fff;
        font-weight: bold;

        box-shadow: 0 1px 1px 0 rgba(0, 0, 0, 0.14),
            0 2px 1px -1px rgba(0, 0, 0, 0.12), 0 1px 3px 0 rgba(0, 0, 0, 0.2);
    }

    .status {
        display: flex;
        float: right;
        padding-inline: 5px;
        margin-bottom: 10px;
        border-radius: 5px;
        font-weight: bold;
        box-shadow: 0 1px 1px 0 rgba(0, 0, 0, 0.14),
            0 2px 1px -1px rgba(0, 0, 0, 0.12), 0 1px 3px 0 rgba(0, 0, 0, 0.2);
    }

    .drafted {
        background-color: white;
        color: black;
    }

    .deleted {
        background-color: black;
        color: #fff;
    }

    .pending_sale {
        background-color: orange;
        color: #fff;
    }

    .sold {
        color: red;
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
        height: 100px;
    }

    .lot-image {
        /* margin: 10px; Adjust as needed */
        object-fit: contain;
        margin-right: 10px;
        width: 100px;
    }

    .image-container {
        position: relative;
        /* display: inline-block; */
    }

    .delete-button {
        position: absolute;
        top: 0;
        background-color: gray;
        font-weight: bold;
        color: white;
        border: none;
        cursor: pointer;
    }
    /* :global(.edit-dropzone) {
        width: 70px;
        height: 70px;
        border: 2px dashed #ccc;
        border-radius: 3px;
        display: flex;
        justify-content: center;
        align-items: center;
    } */

    :global(.add-dropzone) {
        width: 100%;
        height: 100%;
        display: flex;
    }
</style>
