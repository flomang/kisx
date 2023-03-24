//import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client';
import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client/core';

const cache = new InMemoryCache();

const link = new HttpLink({
  uri: 'http://localhost:9000',
});

const client = new ApolloClient({
  cache,
  link,
});

export default client;
