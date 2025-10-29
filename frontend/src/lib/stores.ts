// src/lib/stores.ts

import { writable, type Writable } from 'svelte/store';
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

// --- ADD THE NEW USER STORE ---
// Define a type for our user profile for better TypeScript support.
// This should match the User struct from your Rust backend.
export type UserProfile = {
  id: number;
  nip: string;
  nama: string;
  role: 'Pegawai' | 'AdminBapas' | 'SuperAdmin';
  unit_kerja_id: number | null;
  // Add any other fields you want to use in the UI
};

// Create a writable store for the user profile.
// It starts as `null` because the user is not logged in initially.
export const user: Writable<UserProfile | null> = writable(null);