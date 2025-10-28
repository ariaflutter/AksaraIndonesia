<!-- src/routes/(protected)/+layout.svelte -->
<script lang="ts">
  import { authToken } from '$lib/stores';
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';

  // onMount runs after the component has been added to the DOM.
  onMount(() => {
    // We check for the token when the layout mounts.
    // The `get` function from svelte/store is not available here, so we check directly.
    const token = localStorage.getItem('authToken');
    
    if (!token) {
      // If there's no token, redirect to the login page.
      window.location.href = '/login';
    }
  });

  function handleLogout() {
    // To log out, we simply set the authToken store to null.
    // The store will automatically remove it from localStorage.
    authToken.set(null);
    window.location.href = '/login';
  }
</script>

{#if $authToken}
  <!-- Only show the content if the authToken store has a value. -->
  <!-- This prevents a "flash" of the protected page before the redirect happens. -->
  <header>
    <nav>
      <a href="/dashboard">Dashboard</a>
      <a href="/bapas">Manajemen Bapas</a>
      <!-- Add other links here -->
    </nav>
    <button on:click={handleLogout}>Logout</button>
  </header>

  <main>
    <!-- The <slot /> is where SvelteKit will render the actual page content -->
    <!-- (like our new dashboard page). -->
    <slot />
  </main>
{/if}

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
  }
  nav a {
    margin-right: 1rem;
    text-decoration: none;
    color: #007bff;
  }
  button {
    background-color: #dc3545;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }
</style>