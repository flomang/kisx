import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client/core';

const cache = new InMemoryCache();

let headers: Record<string, string> = {};

if (typeof localStorage !== 'undefined') {
  const token = localStorage.getItem('token');
  if (token) {
    headers.Token = token;
  }
}

const link = new HttpLink({
  uri: 'http://localhost:9000',
  headers,
});

const client = new ApolloClient({
  cache,
  link,
});

export const removeToken = () => {
  localStorage.removeItem("token");
  const newLink = new HttpLink({
    uri: 'http://localhost:9000',
    headers: {},
  });

  client.setLink(newLink);
  client.cache.reset();
}

export const addToken = (token: string) => {
  localStorage.setItem("token", token);

  const newLink = new HttpLink({
    uri: 'http://localhost:9000',
    headers: {
      Token: token,
    },
  });

  client.setLink(newLink);
  client.cache.reset();
}

export default client;
