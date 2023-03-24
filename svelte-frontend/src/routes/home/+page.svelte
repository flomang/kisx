<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client, { addToken, removeToken } from "../../lib/apollo";

    // interface SigninProps {
    //     // add props here
    //     //onSignin?: () => void;
    // }

    interface SigninResult {
        signin: {
            user: {
                token: string;
                username: string;
            };
        };
    }

    let email = "";
    let password = "";
    let message = "";
    let currentUser = "";

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
            }
        } catch (error: any) {
            if (
                error instanceof ApolloError &&
                error.message.includes("Unauthorized")
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

    interface GetCurrentUserResult {
        getCurrentUser: {
            user: {
                token: string;
                username: string;
            };
        };
    }

    const CURRENT_USER = gql`
        query GetCurrentUser {
            getCurrentUser {
                user {
                    token
                    username
                }
            }
        }
    `;

    const handleGetCurrentUser = async () => {
        try {
            const { data } = await client.query<GetCurrentUserResult>({
                query: CURRENT_USER,
            });

            const { token, username } = data?.getCurrentUser.user ?? {};
            if (token) {
                localStorage.setItem("token", token);
                currentUser = username ?? currentUser;
            }
        } catch (error: any) {
            if (
                error instanceof ApolloError &&
                error.message.includes("Unauthorized")
            ) {
                currentUser = "please login";
            } else {
                console.error(
                    "encountered unexpected error from signin request:",
                    error
                );
                alert(error.message);
            }
        }
    };

    const handleSignout = async () => {
        client.cache.reset();
        removeToken();
    };
</script>

<form on:submit|preventDefault={handleSignin}>
    <input type="email" bind:value={email} required />
    <input type="password" bind:value={password} required />
    <button type="submit">Sign In</button>
    <div>{message}</div>
</form>

<form on:submit|preventDefault={handleGetCurrentUser}>
    <button type="submit">Current User</button>
    <div>{currentUser}</div>
</form>

<form on:submit|preventDefault={handleSignout}>
    <button type="submit">Signout</button>
</form>