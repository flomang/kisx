<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client, { addToken } from "../../lib/apollo";
    import { goto } from "$app/navigation";
    import Textfield from "@smui/textfield";
    import Button, { Label } from "@smui/button";
    import FormField from "@smui/form-field";
    import Checkbox from "@smui/checkbox";
    import { Icon as CommonIcon } from "@smui/common";
    import Paper, { Title, Subtitle, Content } from "@smui/paper";

    let email = "";
    let password = "";
    let message = "";
    let remember = false;

    interface SigninResult {
        signin: {
            user: {
                token: string;
                username: string;
            };
        };
    }

    const SIGNIN_MUTATION = gql`
        mutation SigninMutation($email: String!, $password: String!) {
            signin(params: { email: $email, password: $password }) {
                user {
                    token
                    username
                }
            }
        }
    `;

    // determine if the email and password are valid
    function is_valid(email: string, password: string): boolean {
        const emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isEmailValid: boolean = emailRegex.test(email);
        const isPasswordValid: boolean = password.length >= 6;

        return isEmailValid && isPasswordValid;
    }

    const handleInput = (event: Event) => {
        message = "";
    };

    const handleSignin = async () => {
        try {
            const { data } = await client.mutate<SigninResult>({
                mutation: SIGNIN_MUTATION,
                variables: { email, password },
            });

            const { token, username } = data?.signin.user ?? {};
            if (token) {
                addToken(token);
                message = username ?? message;
                goto("/home");
            }
        } catch (error: any) {
            message = error.message;
        }
    };
</script>

<div class="container">
    <Paper color="secondary" variant="clear" style="width: 100%;">
        <Title><b>Login</b></Title>
        <Content>
            <div class="input-container">
                <Textfield
                    variant="filled"
                    style="width: 100%;"
                    class="input-container"
                    bind:value={email}
                    on:input={handleInput}
                >
                    <svelte:fragment slot="label">
                        <CommonIcon
                            class="material-icons"
                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                            >email</CommonIcon
                        > Email
                    </svelte:fragment>
                </Textfield>
            </div>
            <div class="input-container">
                <Textfield
                    variant="filled"
                    style="width: 100%;"
                    bind:value={password}
                    on:input={handleInput}
                    type="password"
                >
                    <svelte:fragment slot="label">
                        <CommonIcon
                            class="material-icons"
                            style="font-size: 1em; line-height: normal; vertical-align: top;"
                            >lock</CommonIcon
                        > Password
                    </svelte:fragment>
                </Textfield>
            </div>
            <div class="remember-container">
                <FormField>
                    <Checkbox bind:checked={remember} />
                    Remember me
                </FormField>
            </div>
            <div class="button-container">
                <Button
                    on:click={handleSignin}
                    variant="raised"
                    style="width: 100%; height: 100%;"
                    disabled={!is_valid(email, password)}
                >
                    <Label>Login</Label>
                </Button>
            </div>
            <div class="links-container">
                <div class="left">
                    <a href="/forgot">Forgot Password</a>
                </div>
                <div class="right">
                    <a href="/signup">Sign Up</a>
                </div>
            </div>
            <div class="message-container">
                {message}
            </div>
        </Content>
    </Paper>
</div>

<style>
    .container {
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
        padding-bottom: 12px; /* Adjust the value as needed */
    }

    .remember-container {
        width: 100%;
        align-items: center;
        margin-left: -10px;
    }

    .button-container {
        display: flex;
        justify-content: center;
        width: 100%;
        padding-block: 10px;
        height: 45px;
    }

    .links-container {
        width: 100%;
        padding: 0px;
        display: flex;
        justify-content: space-between;
    }

    .links-container .left {
        align-self: flex-start;
    }

    .links-container .right {
        align-self: flex-end;
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
