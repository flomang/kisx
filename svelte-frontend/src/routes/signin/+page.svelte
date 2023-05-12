<script lang="ts">
    import {
        useForm,
        validators,
        email as emailFunc,
        required,
    } from "svelte-use-form";
    import { ApolloError, gql } from "@apollo/client/core";
    import client, { addToken, removeToken } from "../../lib/apollo";
    import { goto } from "$app/navigation";
    import Textfield from "@smui/textfield";
    import Button, { Label } from "@smui/button";
    import { Icon as CommonIcon } from "@smui/common";

    const form = useForm();
    let email = "";
    let password = "";
    let message = "";

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

    function is_valid(email: string, password: string): boolean {
        const emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isEmailValid: boolean = emailRegex.test(email);
        const isPasswordValid: boolean = password.length >= 10;

        return isEmailValid && isPasswordValid;
    }

    const handleEmail = (event: Event) => {
        message = "";
    };
    const handlePassword = (event: Event) => {
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
    <form>
        <h1>Login</h1>
        <div>
            <Textfield
                variant="outlined"
                bind:value={email}
                on:input={handleEmail}
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
        <div>
            <Textfield
                variant="outlined"
                bind:value={password}
                on:input={handlePassword}
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
        <div class="button-container">
            <Button on:click={handleSignin} variant="raised" disabled='{!is_valid(email, password)}'>
                <Label>Login</Label>
            </Button>
        </div>
        <div class="message-container">
            {message}
        </div>
    </form>
</div>

<style>
    :global(.touched:invalid) {
        border-color: red;
        outline-color: red;
    }

    div {
        padding: 10px;
    }

    h1 {
        text-align: center;
    }

    .button-container {
        display: flex;
        justify-content: center;
    }

    .message-container {
        display: flex;
        justify-content: center;
        color: red;
        height: 0px; /* Adjust the height as needed */
    }

    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }
</style>
