import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import client from "../lib/apollo";
import { gql } from "@apollo/client/core";

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

/** @type {import('./$types').LayoutLoad} */
export async function load({route }) {
    if (browser && route.id !== '/login') {
        try {
            let {data} =  await client.query<GetCurrentUserResult>({
                    query: CURRENT_USER,
            });

            return data?.getCurrentUser.user ?? {};
            
        } catch (error: any) {
            // user is not logged in 
            throw redirect(307, '/login');
        }
    }
}