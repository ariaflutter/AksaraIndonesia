// src/lib/stores.ts

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// This function gets the token from localStorage if we are in the browser.
function getInitialToken() {
  if (browser) {
    return localStorage.getItem('authToken');
  }
  return null;
}

// Create a "writable" store.
// A writable store is a variable that components can subscribe to.
// When its value changes, all subscribed components will automatically update.
// We initialize it with the token from localStorage.
export const authToken = writable<string | null>(getInitialToken());

// This part is crucial: we subscribe to changes in the store and update
// localStorage whenever the token changes (e.g., on login or logout).
if (browser) {
  authToken.subscribe(token => {
    if (token) {
      localStorage.setItem('authToken', token);
    } else {
      localStorage.removeItem('authToken');
    }
  });
}