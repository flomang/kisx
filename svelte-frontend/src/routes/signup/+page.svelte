<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client, { addToken } from "../../lib/apollo";
    import { goto } from "$app/navigation";
    import Textfield from "@smui/textfield";
    import Button, { Label } from "@smui/button";
    import FormField from "@smui/form-field";
    import Checkbox from "@smui/checkbox";
    import { Icon as CommonIcon } from "@smui/common";
    import HelperText from "@smui/textfield/helper-text";
    import Paper, { Title, Subtitle, Content } from "@smui/paper";
    import { Icon } from "@smui/icon-button";

    let username = "";
    let email = "";
    let password = "";
    let agree = false;
    let message = "";
    let usernameError = "";
    let emailError = "";
    let passwordError = "";
    let success = true;

    interface SignupResult {
        signup: {
            user: {
                token: string;
                username: string;
            };
        };
    }

    const SIGNUP_MUTATION = gql`
        mutation SignupMutation(
            $username: String!
            $email: String!
            $password: String!
        ) {
            signup(
                params: {
                    username: $username
                    email: $email
                    password: $password
                }
            ) {
                user {
                    token
                    username
                }
            }
        }
    `;

    // determine if the email and password are valid
    function is_valid(
        email: string,
        password: string,
        username: string,
        agree: boolean
    ): boolean {
        const emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isEmailValid: boolean = emailRegex.test(email);
        const isPasswordValid: boolean = password.length >= 8;
        const isUsernameValid: boolean = username.length >= 3;

        return isEmailValid && isPasswordValid && isUsernameValid && agree;
    }

    const handleInputUsername = (event: Event) => {
        usernameError = "";
    };
    const handleInputEmail = (event: Event) => {
        emailError = "";
    };
    const handleInputPassword = (event: Event) => {
        passwordError = "";
    };

    const handleSignup = async () => {
        try {
            const { data } = await client.mutate<SignupResult>({
                mutation: SIGNUP_MUTATION,
                variables: {
                    username,
                    email,
                    password,
                },
            });

            const { token } = data?.signup.user ?? {};
            if (token) {
                success = true;
                //    addToken(token);
                //    goto("/home");
            }
        } catch (error: any) {
            message = error.message;
            //console.log(JSON.stringify(error));
            if (error.message === "Validation Errors" && error.graphQLErrors) {
                const validationErrors =
                    error.graphQLErrors[0].extensions.errors;
                validationErrors.forEach((e) => {
                    const { key, message } = e;
                    console.log(key, message);
                    if (key == "username") {
                        usernameError = message;
                    } else if (key == "email") {
                        emailError = message;
                    } else if (key == "password") {
                        passwordError = message;
                    }
                });
            }
        }
    };
</script>

<div class="container">
    {#if !success}
        <Paper color="secondary" variant="clear" style="width: 100%;">
            <Title class="title"><b>Sign Up</b></Title>
            <Content>
                <div class="username-container">
                    <Textfield
                        variant="filled"
                        style="width: 100%;"
                        class="input-container"
                        bind:value={username}
                        on:input={handleInputUsername}
                        invalid={usernameError != ""}
                    >
                        <svelte:fragment slot="label">
                            <CommonIcon
                                class="material-icons"
                                style="font-size: 1em; line-height: normal; vertical-align: top;"
                                >face</CommonIcon
                            >
                            <span>Username</span>
                        </svelte:fragment>
                        <HelperText
                            persistent={usernameError != ""}
                            slot="helper">{usernameError}</HelperText
                        >
                    </Textfield>
                </div>
                <div class="email-container">
                    <Textfield
                        variant="filled"
                        style="width: 100%;"
                        class="input-container"
                        bind:value={email}
                        on:input={handleInputEmail}
                        invalid={emailError != ""}
                    >
                        <svelte:fragment slot="label">
                            <CommonIcon
                                class="material-icons"
                                style="font-size: 1em; line-height: normal; vertical-align: top;"
                                >email</CommonIcon
                            >
                            <span>Email</span>
                        </svelte:fragment>
                        <HelperText
                            class="helper-text"
                            persistent={emailError != ""}
                            slot="helper">{emailError}</HelperText
                        >
                    </Textfield>
                </div>
                <div class="password-container">
                    <Textfield
                        variant="filled"
                        style="width: 100%;"
                        bind:value={password}
                        on:input={handleInputPassword}
                        type="password"
                        invalid={passwordError != ""}
                    >
                        <svelte:fragment slot="label">
                            <CommonIcon
                                class="material-icons"
                                style="font-size: 1em; line-height: normal; vertical-align: top;"
                                >lock</CommonIcon
                            >
                            <span>Password</span>
                        </svelte:fragment>
                        <HelperText
                            slot="helper"
                            persistent={passwordError != ""}
                            >{passwordError}</HelperText
                        >
                    </Textfield>
                </div>
                <div class="agree-container">
                    <FormField>
                        <Checkbox bind:checked={agree} />
                        Agree to terms and conditions
                    </FormField>
                </div>
                <div class="button-container">
                    <Button
                        on:click={handleSignup}
                        variant="raised"
                        style="width: 100%; height: 100%;"
                        disabled={!is_valid(email, password, username, agree)}
                    >
                        <Label>Submit</Label>
                    </Button>
                </div>
                <div class="links-container">
                    <a href="/login">Already have an account?</a>
                </div>
            </Content>
        </Paper>
    {:else}
        <Paper color="secondary" variant="clear" style="width: 100%;">
            <Title class="title"><b> Thanks for signing up!</b></Title>
            <Content>
                <div class="content-container">
                    <Icon class="material-icons" on>send</Icon>
                    A verification link has been sent to your email address. Please
                    verify your email address before continuing.
                </div>
                <div class="links-container">
                    <a href="/login">Back to Login</a>
                </div>
            </Content>
        </Paper>
    {/if}
</div>

<style>
    .container {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        height: 100vh;
        margin: auto;
        width: 300px;
    }

    .username-container {
        width: 100%;
        padding-bottom: 6px;
    }
    .email-container {
        width: 100%;
        padding-bottom: 6px;
    }
    .password-container {
        width: 100%;
        padding-bottom: 6px;
    }

    .agree-container {
        width: 100%;
        align-items: center;
        margin-left: -10px;
        margin-top: -10px;
        padding-bottom: 8px;
    }

    .button-container {
        display: flex;
        justify-content: center;
        width: 100%;
        height: 45px;
        padding-bottom: 16px;
    }

    .content-container {
        padding-bottom: 16px;
    }

    .links-container {
        width: 100%;
        display: flex;
        justify-content: space-between;
    }

    .message-container {
        display: flex;
        justify-content: center;
        color: red;
        height: 0px;
    }

    a {
        font-size: 0.9em;
        color: #40b3ff;
        text-decoration: none;
    }
</style>
