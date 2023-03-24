<script lang="ts">
    //import { gql } from "@apollo/client";
    import { gql } from "@apollo/client/core";
    import client from "../../lib/apollo";

    let username: string | undefined;
    interface SigninProps {
        // add props here
        //onSignin?: () => void;
    }

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

    const SIGNIN_MUTATION = gql`
        mutation SigninMutation($email: String!, $password: String!) {
            signin(params: {email: $email, password: $password}) {
                user {
                    token
                    username
                }
            }
        }
    `;

    const handleSignin = async () => {
        const { data } = await client.mutate<SigninResult>({
            mutation: SIGNIN_MUTATION,
            variables: { email, password },
        });
        //console.log("Object: %o", data?.signin.user.token);

        let token = data?.signin.user.token;
        if (token != undefined) {
            username = data?.signin.user.username;
            localStorage.setItem("token", token);
        }
    };
</script>

<form on:submit|preventDefault={handleSignin}>
    <input type="email" bind:value={email} required />
    <input type="password" bind:value={password} required />
    <button type="submit">Sign In</button>
    <div>{username}</div>
</form>
