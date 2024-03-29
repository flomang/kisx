<script lang="ts">
    import { gql } from "@apollo/client/core";
    import client, { addToken } from "../../lib/apollo";
    import { goto } from "$app/navigation";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import type { ToastSettings } from "@skeletonlabs/skeleton";
    import Icon from "@iconify/svelte";

    const toastStore = getToastStore();

    let email = "";
    let password = "";
    let message = "";
    let remember = false;
    let show = false;

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
                goto("/dashboard");
            }
        } catch (error: any) {
            message = error.message;
            const t: ToastSettings = {
                message: error.message,
                background: "variant-filled-error",
                autohide: true,
            };
            toastStore.trigger(t);
        }
    };
</script>

<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
        <svg
            class="fill-token mx-auto h-20 w-auto -scale-x-[100%]"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 200 200"
        >
            <path
                fill-rule="evenodd"
                d="M98.77 50.95c25.1 0 46.54 8.7 61.86 23a41.34 41.34 0 0 0 5.19-1.93c4.35-2.02 10.06-6.17 17.13-12.43-1.15 10.91-2.38 18.93-3.7 24.04-.7 2.75-1.8 6.08-3.3 10a80.04 80.04 0 0 1 8.42 23.33c6.04 30.3-4.3 43.7-28.33 51.18.18.9.32 1.87.42 2.9.86 8.87-3.62 23.19-9 23.19-3.54 0-5.84-4.93-8.3-12.13-.78 8.34-4.58 17.9-8.98 17.9-4.73 0-7.25-8.84-10.93-20.13a214 214 0 0 1-.64 2.93l-.16.71-.16.71-.17.71c-1.84 7.58-4.46 15.07-8.5 15.07-5.06 0-2.29-15.9-10.8-22.63-43.14 2.36-79.43-13.6-79.43-59.62 0-8.48 2-16.76 5.69-24.45a93.72 93.72 0 0 1-1.77-3.68c-2.87-6.32-6.3-15.88-10.31-28.7 10.26 7.66 18.12 12.22 23.6 13.68.5.14 1.02.26 1.57.36 14.36-14.44 35.88-24.01 60.6-24.01Zm-9.99 62.3c-14.57 0-26.39 11.45-26.39 25.58 0 14.14 11.82 25.6 26.39 25.6s26.39-11.46 26.39-25.6c0-13.99-11.58-25.35-25.95-25.58Zm37.45 31.95c-4.4 0-6.73 9.4-6.73 13.62 0 3.3 1.1 5.12 2.9 5.45 4.39.4 3.05-5.97 5.23-5.97 1.06 0 2.2 1.35 3.34 2.73l.34.42c1.25 1.52 2.5 2.93 3.64 2.49 2.7-1.61 1.67-5.12.74-7.88-3.3-6.96-5.05-10.86-9.46-10.86Zm-36.85-28.45c12.57 0 22.76 9.78 22.76 21.85 0 12.07-10.2 21.85-22.76 21.85-.77 0-1.53-.04-2.29-.11 11.5-1.1 20.46-10.42 20.46-21.74 0-11.32-8.97-20.63-20.46-21.74.76-.07 1.52-.1 2.3-.1Zm65.54-5c-10.04 0-18.18 10.06-18.18 22.47 0 12.4 8.14 22.47 18.18 22.47s18.18-10.06 18.18-22.47c0-12.41-8.14-22.48-18.18-22.48Zm.6 3.62c8.38 0 15.16 8.4 15.16 18.74 0 10.35-6.78 18.74-15.16 18.74-.77 0-1.54-.07-2.28-.21 7.3-1.36 12.89-9.14 12.89-18.53 0-9.4-5.6-17.17-12.89-18.53.74-.14 1.5-.2 2.28-.2Zm3.34-72.27.12.07c.58.38.75 1.16.37 1.74l-2.99 4.6c-.35.55-1.05.73-1.61.44l-.12-.07a1.26 1.26 0 0 1-.37-1.74l2.98-4.6a1.26 1.26 0 0 1 1.62-.44ZM39.66 42l.08.1 2.76 3.93a1.26 1.26 0 0 1-2.06 1.45l-2.76-3.94A1.26 1.26 0 0 1 39.66 42Zm63.28-42 2.85 24.13 10.62-11.94.28 29.72-2.1-.47a77.8 77.8 0 0 0-16.72-2.04c-4.96 0-9.61.67-13.96 2l-2.34.73L83.5 4.96l9.72 18.37L102.94 0Zm-1.87 13.39-7.5 17.96-7.3-13.8-1.03 19.93.22-.06a51.56 51.56 0 0 1 12.1-1.45h.31c4.58 0 9.58.54 15 1.61l.35.07-.15-16.54-9.79 11-2.21-18.72Zm38.86 19.23c.67.2 1.05.89.86 1.56l-.38 1.32c-.17.62-.8 1-1.42.89l-.13-.03a1.26 1.26 0 0 1-.86-1.56l.38-1.32c.19-.66.88-1.05 1.55-.86ZM63.95 31.1l.05.12.7 2.17a1.26 1.26 0 0 1-2.34.9l-.04-.12-.71-2.17a1.26 1.26 0 0 1 2.34-.9Z"
            />
        </svg>
        <h2
            class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-white-900"
        >
            Login to your account
        </h2>
    </div>

    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <form class="space-y-6">
            <div>
                <label
                    for="email"
                    class="block text-sm font-medium leading-6 text-white-900"
                    >Email address</label
                >
                <div class="mt-2">
                    <input
                        type="email"
                        bind:value={email}
                        on:input={handleInput}
                        autocomplete="email"
                        required
                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                </div>
            </div>

            <div>
                <div class="flex items-center justify-between">
                    <label
                        for="password"
                        class="block text-sm font-medium leading-6 text-white-900"
                        >Password</label
                    >
                </div>
                <div class="mt-2 relative">
                    {#if show}
                        <input
                            type="text"
                            bind:value={password}
                            on:input={handleInput}
                            autocomplete="current-password"
                            required
                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                        <div
                            class="absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5"
                        >
                            <button on:click={() => (show = !show)}>
                                <Icon
                                    icon="majesticons:eye-off-line"
                                    color="gray"
                                    width="30"
                                    height="30"
                                /></button
                            >
                        </div>
                    {:else}
                        <input
                            type="password"
                            bind:value={password}
                            on:input={handleInput}
                            autocomplete="current-password"
                            required
                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                        <div
                            class="absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5"
                        >
                            <button on:click={() => (show = !show)}>
                                <Icon
                                    icon="majesticons:eye-line"
                                    color="gray"
                                    width="30"
                                    height="30"
                                /></button
                            >
                        </div>
                    {/if}
                </div>
            </div>
            <div>
                <button
                    on:click={handleSignin}
                    disabled={!is_valid(email, password)}
                    class="disabled:cursor-not-allowed disabled:opacity-50 flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >Login</button
                >
            </div>
        </form>

        <div class="flex items-center justify-between">
            <div class="mt-10 text-center text-sm text-white-500">
                <a
                    href="/forgot"
                    class="font-semibold text-indigo-600 hover:text-indigo-500"
                    >Forgot password</a
                >
            </div>
            <div class="mt-10 text-center text-sm text-white-500">
                <a
                    href="/signup"
                    class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500"
                >
                    Not a member?</a
                >
            </div>
        </div>
    </div>
</div>

<style>
    a {
        color: #40b3ff;
        text-decoration: none;
    }
</style>
