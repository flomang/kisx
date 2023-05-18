import type { PageLoad } from './$types';
import { browser } from '$app/environment';
import { ApolloError, gql } from "@apollo/client/core";
import client from "../../lib/apollo";
import { redirect } from '@sveltejs/kit';


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


// export const load = (async ({ }) => {
//     let currentUser = "";

//     if (browser) {
//         try {
//             const { data } = await client.query<GetCurrentUserResult>({
//                 query: CURRENT_USER,
//             });

//             const { token, username } = data?.getCurrentUser.user ?? {};
//             if (token) {
//                 localStorage.setItem("token", token);
//                 currentUser = username ?? currentUser;
//             }

//             return {
//                 username: currentUser,
//             };

//         } catch (error: any) {
//             //throw redirect(307, '/login');
//             //if (
//             //    error instanceof ApolloError &&
//             //    error.message.includes("Unauthorized")
//             //) {
//             //    throw redirect(307, '/signin');
//             //} else {
//             //    console.error(
//             //        "encountered unexpected error from signin request:",
//             //        error
//             //    );
//             //    alert(error.message);
//             //}
//         }
//     }
// }) satisfies PageLoad;