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

    let username = "";
    let email = "";
    let password = "";
    let agree = false;
    let message = "";
    let usernameError = "";
    let emailError = "";
    let passwordError = "";

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
    function is_valid(email: string, password: string, username: string, agree: boolean): boolean {
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
                addToken(token);
                goto("/home");
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
    <h1>Sign Up</h1>
    <div class="input-container">
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
            <HelperText persistent={usernameError != ""} slot="helper"
                >{usernameError}</HelperText
            >
        </Textfield>
    </div>
    <div class="input-container">
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
            <HelperText class="helper-text" persistent={emailError != ""} slot="helper"
                >{emailError}</HelperText
            >
        </Textfield>
    </div>
    <div class="input-container">
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
            <HelperText slot="helper" persistent={passwordError != ""}>{passwordError}</HelperText>
        </Textfield>
    </div>
    <div class="remember-container">
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
        padding-bottom: 5px;
    }

    .remember-container {
        width: 100%;
        align-items: center;
        margin-left: -20px;
        margin-top: -10px;
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
        padding: 10px;
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
