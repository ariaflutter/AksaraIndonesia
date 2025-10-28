<!-- src/routes/(protected)/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { authToken } from '$lib/stores';

  let userProfile: any = null; // We'll store the user's profile here
  let errorMessage = '';

  onMount(async () => {
    const token = $authToken; // Get the current token value from the store

    if (!token) {
      // This should ideally not happen because the layout protects us, but it's a good safeguard.
      return;
    }

    try {
      const response = await fetch('http://127.0.0.1:3000/api/auth/me', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });

      if (!response.ok) {
        throw new Error('Gagal mengambil profil pengguna.');
      }

      userProfile = await response.json();
    } catch (error) {
      if (error instanceof Error) {
        errorMessage = error.message;
      }
    }
  });
</script>

<h2>Dashboard</h2>

{#if userProfile}
  <p>Selamat datang, <strong>{userProfile.nama}</strong>!</p>
  <p>NIP: {userProfile.nip}</p>
  <p>Role: {userProfile.role}</p>
{:else if errorMessage}
  <p class="error">{errorMessage}</p>
{:else}
  <p>Loading user profile...</p>
{/if}