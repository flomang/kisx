<script lang="ts">
    import {
        useForm,
        validators,
        HintGroup,
        Hint,
        email as emailFunc,
        required,
    } from "svelte-use-form";
    import { ApolloError, gql } from "@apollo/client/core";
    import client, { addToken, removeToken } from "../../lib/apollo";
    import { goto } from "$app/navigation";

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

    const handleSignin = async () => {
        try {
            const { data } = await client.mutate<SigninResult>({
                mutation: SIGNIN_MUTATION,
                variables: { email, password },
            });

            const { token, username } = data?.signin.user ?? {};
            if (token) {
                addToken(token)
                message = username ?? message;
                goto("/home");
            }
        } catch (error: any) {
            if (
                error instanceof ApolloError &&
                error.message.includes("Unauthorized") || 
                error.message.includes("not found")
            ) {
                message = "invalid email / password";
            } else {
                console.error(
                    "encountered unexpected error from signin request:",
                    error
                );
                alert(error.message);
            }
        }
    };
</script>

<form use:form on:submit|preventDefault={handleSignin}>
    <h1>Login</h1>

    <input type="email" name="email" use:validators={[required, emailFunc]}  bind:value={email} required  />
    <HintGroup for="email">
        <Hint on="required">This is a mandatory field</Hint>
        <Hint on="email" hideWhenRequired>Email is not valid</Hint>
    </HintGroup>

    <input type="password" name="password" use:validators={[required]}  bind:value={password} required />
    <Hint for="password" on="required">This is a mandatory field</Hint>

    <button disabled={!$form.valid}>Login</button>
</form>
<div>{message}</div>

<style>
    :global(.touched:invalid) {
        border-color: red;
        outline-color: red;
    }
</style>
