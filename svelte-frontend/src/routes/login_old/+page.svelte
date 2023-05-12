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

    const form1 = useForm();
    const form2 = useForm();
    let loginEmail = "";
    let loginPassword = "";
    let loginError = "";
    let signupEmail = "";
    let signupUsername = "";
    let signupPassword = "";
    let signupError = "";
    let disableSignup = true;
    let nameError = "";
    let emailError = "";
    let passwordError = "";

    interface SigninResult {
        signin: {
            user: {
                token: string;
                username: string;
            };
        };
    }
    interface SignupResult {
        signup: {
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

    const handleLoginInput = (event: Event) => {
        loginError = "";
    };
    const handleSigninInput = (event: Event) => {
        signupError = "";
    };
    const handleToggleForm = (event: Event) => {
        disableSignup = !disableSignup;
    };

    const handleSignin = async () => {
        try {
            const { data } = await client.mutate<SigninResult>({
                mutation: SIGNIN_MUTATION,
                variables: { email: loginEmail, password: loginPassword },
            });

            const { token, username } = data?.signin.user ?? {};
            if (token) {
                addToken(token);
                loginError = username ?? loginError;
                goto("/home");
            }
        } catch (error: any) {
            if (
                error instanceof ApolloError &&
                error.message.includes("Unauthorized")
            ) {
                loginError = "invalid email / password";
            } else {
                console.error(
                    "encountered unexpected error from signin request:",
                    error
                );
                alert(error.message);
            }
        }
    };

    const handleSignup = async () => {
        try {
            const { data } = await client.mutate<SignupResult>({
                mutation: SIGNUP_MUTATION,
                variables: {
                    username: signupUsername,
                    email: signupEmail,
                    password: signupPassword,
                },
            });

            const { token, username } = data?.signup.user ?? {};
            if (token) {
                addToken(token);
                goto("/home");
            }
        } catch (error: any) {
            signupError = error.message;
            //console.log(JSON.stringify(error));

            if (error.message === "Validation Errors" && error.graphQLErrors) {
                const validationErrors = error.graphQLErrors[0].extensions.errors;
                // Process the validationErrors object according to your requirements
                // validationErrors.forEach(v => {
                //     console.log(v.key, v.message);
                // });
                //console.log(JSON.stringify(validationErrors));

                //for (v of validationErrors) { 
                validationErrors.forEach((e) => {
                    // Set variables based on the validation error
                    const { key, message } = e;
                    // Perform any necessary actions with the key and message
                    console.log(key, message);
                    if (key == "username") {
                        nameError = message;
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

<body>
    <div class="main">
        <input
            type="checkbox"
            id="chk"
            aria-hidden="true"
            on:input={handleToggleForm}
        />

        <div class="login">
            <form use:form1 on:submit|preventDefault={handleSignin}>
                <label for="chk" aria-hidden="true">Login</label>
                <input
                    type="email"
                    name="email"
                    placeholder="Email"
                    use:validators={[required, emailFunc]}
                    bind:value={loginEmail}
                    on:input={handleLoginInput}
                    required
                />

                <input
                    type="password"
                    name="password"
                    placeholder="Password"
                    use:validators={[required]}
                    bind:value={loginPassword}
                    on:input={handleLoginInput}
                    required
                />

                <button class={$form1.valid ? "enable" : ""}>Login</button>
                <span class="fade-in-span {loginError ? 'show' : ''}">
                    {loginError}
                </span>
            </form>
        </div>

        <div class="signup">
            <form use:form2 on:submit|preventDefault={handleSignup}>
                <label for="chk" aria-hidden="true">Sign up</label>
                <input
                    type="text"
                    name="username"
                    placeholder="User name"
                    disabled={disableSignup}
                    bind:value={signupUsername}
                    on:input={handleSigninInput}
                    use:validators={[required]}
                    required
                />
                <span class="fade-in-span {nameError ? 'show' : ''}">
                    {nameError}
                </span>
                <input
                    type="email"
                    name="email"
                    placeholder="Email"
                    use:validators={[required, emailFunc]}
                    bind:value={signupEmail}
                    on:input={handleSigninInput}
                    disabled={disableSignup}
                    required
                />
                <span class="fade-in-span {emailError ? 'show' : ''}">
                    {emailError}
                </span>
                <input
                    type="password"
                    name="password"
                    placeholder="Password"
                    use:validators={[required]}
                    bind:value={signupPassword}
                    on:input={handleSigninInput}
                    disabled={disableSignup}
                    required
                />
                <span class="fade-in-span {passwordError ? 'show' : ''}">
                    {passwordError}
                </span>

                <button
                    disabled={disableSignup}
                    class={$form2.valid ? "enable" : ""}>Sign up</button
                >
                <span class="fade-in-span {signupError ? 'show' : ''}">
                    {signupError}
                </span>
            </form>
        </div>
    </div>
</body>

<style>
    :global(.touched:invalid) {
        border-color: red;
        outline-color: red;
    }

    body {
        margin: 0;
        padding: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        font-family: "Jost", sans-serif;
        background: linear-gradient(to bottom, #0f0c29, #302b63, #24243e);
    }
    .main {
        width: 350px;
        height: 500px;
        background: red;
        overflow: hidden;
        background: url("https://doc-08-2c-docs.googleusercontent.com/docs/securesc/68c90smiglihng9534mvqmq1946dmis5/fo0picsp1nhiucmc0l25s29respgpr4j/1631524275000/03522360960922298374/03522360960922298374/1Sx0jhdpEpnNIydS4rnN4kHSJtU1EyWka?e=view&authuser=0&nonce=gcrocepgbb17m&user=03522360960922298374&hash=tfhgbs86ka6divo3llbvp93mg4csvb38")
            no-repeat center/ cover;
        border-radius: 10px;
        box-shadow: 1px 10px 25px #000;
    }
    #chk {
        display: none;
    }
    .login {
        position: relative;
        width: 100%;
        height: 100%;
    }
    label {
        color: #fff;
        font-size: 2.3em;
        justify-content: center;
        display: flex;
        margin: 60px;
        font-weight: bold;
        cursor: pointer;
        transition: 0.5s ease-in-out;
    }
    .fade-in-span {
        color: red;
        font-size: 0.75em;
        justify-content: center;
        display: inline-block;
        position: absolute;
        margin-top: -17px;
        margin-left: 60px;
        /* margin: 7px; */
        font-weight: bold;
        opacity: 0;
        transition: opacity 0.1s ease-in-out;
    }
    .fade-in-span.show {
        opacity: 1;
    }

    input {
        width: 60%;
        height: 20px;
        background: #e0dede;
        justify-content: center;
        display: flex;
        margin: 20px auto;
        padding: 10px;
        /* border: none; */
        outline: none;
        border-radius: 5px;
    }
    button {
        width: 60%;
        height: 40px;
        margin: 10px auto;
        justify-content: center;
        display: block;
        color: #ffffff63;
        background: #573b8a63;
        font-size: 1em;
        font-weight: bold;
        margin-top: 20px;
        outline: none;
        border: none;
        border-radius: 5px;
    }
    button.enable {
        background: #573b8a;
        color: #fff;
        cursor: pointer;
    }
    button:hover.enable {
        background: #6d44b8;
    }

    .signup {
        height: 560px;
        background: #eee;
        border-radius: 60% / 10%;
        transform: translateY(-160px);
        transition: 0.6s ease-in-out;
    }
    .signup label {
        color: #573b8a;
        transform: scale(0.6);
    }

    #chk:checked ~ .signup {
        transform: translateY(-500px);
    }
    #chk:checked ~ .signup label {
        transform: scale(1);
    }
    #chk:checked ~ .login label {
        transform: scale(0.6);
    }
</style>
