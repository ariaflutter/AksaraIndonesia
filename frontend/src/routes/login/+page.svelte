<script lang="ts">
  // --- SCRIPT SECTION ---
  // This is where our TypeScript logic lives.
  import { authToken } from '$lib/stores';

  let nip = '';
  let password = '';
  let errorMessage = '';
  let isLoading = false;

  // This is the function that will be called when the form is submitted.
  async function handleLogin() {
    isLoading = true;
    errorMessage = ''; // Clear any previous errors

    try {
      const response = await fetch('http://127.0.0.1:3000/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ nip, password }),
      });

      if (!response.ok) {
        // If the server returns an error (like 401 Unauthorized),
        // we'll throw an error to be caught by the catch block.
        throw new Error('NIP atau Password salah.');
      }

      // If the login is successful, the server returns our token.
      const data = await response.json();
      const token = data.token;

        // --- CHANGE THIS PART ---
      // Instead of directly setting localStorage, we set the value of our store.
      // The store itself will handle saving it to localStorage.
      authToken.set(data.token);
      // ----------------------

      // Redirect the user to the homepage after successful login.
      window.location.href = '/dashboard';

    } catch (error) {
      // If anything goes wrong, display the error message.
      if (error instanceof Error) {
        errorMessage = error.message;
      } else {
        errorMessage = 'Terjadi kesalahan yang tidak diketahui.';
      }
    } finally {
      // This block runs whether the try succeeded or failed.
      isLoading = false;
    }
  }
</script>

<!-- HTML SECTION -->
<!-- This is the visible part of our component. -->
<main>
  <h1>Login Aksara</h1>

  <!-- We use on:submit|preventDefault to call our handleLogin function -->
  <!-- without causing a full page reload. -->
  <form on:submit|preventDefault={handleLogin}>
    <div class="form-group">
      <label for="nip">NIP</label>
      <!-- `bind:value` creates a two-way binding. If the user types in the
           input, the `nip` variable is updated. -->
      <input type="text" id="nip" bind:value={nip} required />
    </div>

    <div class="form-group">
      <label for="password">Password</label>
      <input type="password" id="password" bind:value={password} required />
    </div>

    <!-- We show an error message if one exists. -->
    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}

    <!-- The button is disabled while the form is submitting. -->
    <button type="submit" disabled={isLoading}>
      {#if isLoading}
        Loading...
      {:else}
        Login
      {/if}
    </button>
  </form>
</main>

<!-- STYLE SECTION -->
<!-- These styles are "scoped" and will only apply to this component. -->
<style>
  main {
    max-width: 400px;
    margin: 5rem auto;
    padding: 2rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }
  .form-group {
    margin-bottom: 1rem;
  }
  label {
    display: block;
    margin-bottom: 0.5rem;
  }
  input {
    width: 100%;
    padding: 0.5rem;
    font-size: 1rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  button {
    width: 100%;
    padding: 0.75rem;
    font-size: 1rem;
    border: none;
    border-radius: 4px;
    background-color: #007bff;
    color: white;
    cursor: pointer;
  }
  button:disabled {
    background-color: #a0cfff;
    cursor: not-allowed;
  }
  .error {
    color: red;
    text-align: center;
    margin-bottom: 1rem;
  }
</style>