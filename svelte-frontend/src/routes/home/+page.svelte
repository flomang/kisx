<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client from "../../lib/apollo";

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
                localStorage.setItem("token", token);
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
</script>

<form on:submit|preventDefault={handleSignin}>
    <input type="email" bind:value={email} required />
    <input type="password" bind:value={password} required />
    <button type="submit">Sign In</button>
    <div>{message}</div>
</form>
