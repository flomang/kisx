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
    import ImageList, {
        Item,
        Image,
        Supporting,
        Label as ImageLabel,
    } from "@smui/image-list";
    import IconButton, { Icon } from "@smui/icon-button";
    import Dialog, { Header, Title, Content, Actions } from "@smui/dialog";

    let tag = "";
    let category = "";
    let condition = "";
    let description = "";
    let images = [{ imageUrl: "", isThumbnail: true}];
    let meta_data = { quantity: 1 };
    let types = ["box", "set", "instructions", "minifig", "part", "custom"];
    let conditions = ["sealed", "complete", "used", "missing pieces", "other"];

    interface LotResult {
        lot: {
            id: string;
            category: string;
            condition: string;
            tag: string;
            description: string;
            meta_data: string;
        },
        images: {
            id: string;
            imageUrl: string;
            isThumbnail: boolean;
            createdAt: string;
            updatedAt: string;
        }
    }

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

    const handleAddLot = async () => {
        try {
            const { data } = await client.mutate<LotResult>({
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

            console.log(data);
            // const { data } = data?.lot ?? {};
            // if (token) {
            //     success = true;
            //     //    addToken(token);
            //     //    goto("/home");
            // }
        } catch (error: any) {
            //message = error.message;
            console.log(JSON.stringify(error));
            // if (error.message === "Validation Errors" && error.graphQLErrors) {
            //     const validationErrors =
            //         error.graphQLErrors[0].extensions.errors;
            //     validationErrors.forEach((e: any) => {
            //         const { key, message } = e;
            //         console.log(key, message);
            //         if (key == "username") {
            //             usernameError = message;
            //         } else if (key == "email") {
            //             emailError = message;
            //         } else if (key == "password") {
            //             passwordError = message;
            //         }
            //     });
            // }
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

    let pins = [
        {
            id: 1,
            imageUrl: "75192.webp",
            title: "Millennium Falcon",
            setID: "75192",
        },
        {
            id: 4,
            imageUrl: "75345.webp",
            title: "501st Clone Troopers Battle Pack",
            setID: "75345",
        },
        {
            id: 2,
            imageUrl: "75337.webp",
            title: "AT-TE™ Walker",
            setID: "75337",
        },
        {
            id: 3,
            imageUrl: "40557.webp",
            title: "Defense of Hoth",
            setID: "40557",
        },
        {
            id: 5,
            imageUrl: "10305.webp",
            title: "Lion Knight's Castle",
            setID: "10305",
        },
        {
            id: 6,
            imageUrl: "21325.webp",
            title: "Medieval Blacksmith",
            setID: "21325",
        },
        {
            id: 7,
            imageUrl: "6086.png",
            title: "Black Knight's Castle",
            setID: "6086",
        },
        {
            id: 8,
            imageUrl: "castle-forest.jpeg",
            title: "Castle in the Forest",
            setID: "910001-1",
        },
        { id: 9, imageUrl: "fig1.jpeg", title: "Space Police" },
        { id: 10, imageUrl: "fig2.jpeg", title: "Bee Keeper" },
        {
            id: 11,
            imageUrl: "6987.webp",
            title: "Blacktron Message Intercept Base",
            setID: "6987",
        },
        {
            id: 12,
            imageUrl: "bonsai-moc.png",
            title: "Master Ogway's Bonsai Tree",
        },
        // Add more pin objects as needed
    ];
    let response;
    let open = false;
</script>

<Dialog
    bind:open
    scrimClickAction=""
    escapeKeyAction=""
    aria-labelledby="mandatory-title"
    aria-describedby="mandatory-content"
    fullscreen
>
    <!-- <Title id="mandatory-title">Webpage Troll</Title> -->
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

<LayoutGrid>
    <Cell span={9}>
        <ImageList class="my-image-list-masonry" masonry>
            {#each pins as pin (pin.id)}
                <Item on:click={() => (open = true)}>
                    <Image src={pin.imageUrl} alt="Image {pin.id}" />
                    <Supporting>
                        <ImageLabel>
                            <b>{pin.title}</b>
                            {#if pin.setID}
                                <div class="label">
                                    {pin.setID}
                                </div>
                            {/if}
                            <div class="actions">
                                <div class="right">
                                    <Icon class="material-icons"
                                        >favorite_border</Icon
                                    >
                                    <Icon class="material-icons">share</Icon>
                                    <Icon class="material-icons">more_vert</Icon
                                    >
                                </div>
                            </div>
                        </ImageLabel>
                    </Supporting>
                </Item>
            {/each}
        </ImageList>
    </Cell>

    <Cell span={3}>
        <div class="sell-it">
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

    /* :global(img) {
        opacity: 0.9;
        transition: all 0.2s;
    } */
    /* :global(img):hover {
        opacity: 1;
        transform: scale(1.04);
    } */

    .pin-container {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 20px;
    }

    .pin {
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 8px;
        background-color: white;
        overflow: hidden;
        transition: all 0.2s;
    }

    .pin:hover {
        opacity: 1;
        transform: scale(1.04);
    }

    .pin img {
        width: 100%;
        height: 100%;
        object-fit: scale-down;
    }
</style>
