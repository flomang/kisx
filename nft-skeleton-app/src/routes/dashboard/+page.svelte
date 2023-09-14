<script lang="ts">
    import { ApolloError, gql } from "@apollo/client/core";
    import client from "../../lib/apollo";
    // import Textfield from "@smui/textfield";
    // import Button, { Label } from "@smui/button";
    // import { Icon as CommonIcon } from "@smui/common";
    // import { Icon } from "@smui/icon-button";
    // import HelperText from "@smui/textfield/helper-text";
    // import Paper, { Title, Subtitle, Content } from "@smui/paper";

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
            helperText =
                "Password reset link sent. Please check your email in a few minutes.";
        } catch (error: any) {
            isError = true;
            //message = error.message;
            //console.log(JSON.stringify(error));
            if (error.message === "Validation Errors" && error.graphQLErrors) {
                const validationErrors =
                    error.graphQLErrors[0].extensions.errors;
                validationErrors.forEach((e: any) => {
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


Dashboard

<style>
 
</style>
