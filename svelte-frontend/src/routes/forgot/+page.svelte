<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    import Textfield from "@smui/textfield";
    import Button, { Label } from "@smui/button";
    import { Icon as CommonIcon } from "@smui/common";
    import { Icon } from "@smui/icon-button";
    import HelperText from "@smui/textfield/helper-text";
    import Paper, { Title, Subtitle, Content } from "@smui/paper";

    let email = "";
    let sent = false;
    let helperText = "";
    let isError = false;

    interface ForgotPasswordResult {
        forgotPassword: boolean;
    }

    const FORGOT_MUTATION = gql`
        mutation ForgotMutation($email: String!) {
            forgotPassword(params: { email: $email })
        }
    `;

    // determine if the email and password are valid
    function is_valid(email: string): boolean {
        const emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isEmailValid: boolean = emailRegex.test(email);

        return isEmailValid;
    }

    const handleInputEmail = (event: Event) => {
        helperText = "";
        isError = false;
        sent = false;
    };

    const handleForgot = async () => {
        try {
            const { data } = await client.mutate<ForgotPasswordResult>({
                mutation: FORGOT_MUTATION,
                variables: {
                    email,
                },
            });

            //console.log(data?.forgotPassword);
            sent = data?.forgotPassword ?? false;
            helperText = "Password reset link sent. Please check your email in a few minutes."
        } catch (error: any) {
            isError = true;
            //message = error.message;
            //console.log(JSON.stringify(error));
            if (error.message === "Validation Errors" && error.graphQLErrors) {
                const validationErrors =
                    error.graphQLErrors[0].extensions.errors;
                validationErrors.forEach((e) => {
                    const { key, message } = e;
                    console.log(key, message);
                    if (key == "email") {
                        helperText = message;
                    }
                });
            }
        }
    };
</script>

<div class="container">
    <Paper color="secondary" variant="clear" style="width: 100%;">
        <Title><b>Forgot Password</b></Title>
        <Subtitle
            >Enter your email address you registered with and we will send you a
            link to reset your password.</Subtitle
        >
        <Content>
            <div class="input-container">
                <Textfield
                    variant="filled"
                    style="width: 100%;"
                    class="input-container"
                    bind:value={email}
                    on:input={handleInputEmail}
                    invalid={isError}
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
                        persistent={helperText != ""}
                        slot="helper">{helperText}</HelperText
                    >
                </Textfield>
            </div>
            <div class="button-container">
                <Button
                    on:click={handleForgot}
                    variant="raised"
                    style="width: 100%; height: 100%;"
                    disabled={!is_valid(email) || sent}
                >
                    {#if sent}
                        <Label>Sent!</Label>
                        <Icon class="material-icons" on>send</Icon>
                    {:else}
                        <Label>Reset Password</Label>
                    {/if}
                </Button>
            </div>
            <div class="links-container">
                <a href="/login">Return to Login</a>
            </div></Content
        >
    </Paper>
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

    .input-container {
        width: 100%;
        padding-top: 15px;
    }

    .button-container {
        display: flex;
        justify-content: center;
        width: 100%;
        height: 45px;
        padding-bottom: 15px;
    }

    .links-container {
        width: 100%;
        display: flex;
        justify-content: space-between;
    }

    a {
        font-size: 0.9em;
        color: #40b3ff;
        text-decoration: none;
    }
</style>
