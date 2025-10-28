<!-- src/routes/+layout.svelte -->

<div class="app-container">
  <header class="public-header">
    <a href="/" class="logo">Aksara</a>
    <nav>
      <a href="/">Home</a>
      <a href="/about">About</a>
      <a href="/login" class="login-button">Login</a>
    </nav>
  </header>

  <main class="public-main">
    <!-- All pages, public and protected, will be rendered inside this slot -->
    <slot />
  </main>

  <footer class="public-footer">
    <p>&copy; {new Date().getFullYear()} Aksara Project. All rights reserved.</p>
  </footer>
</div>

<style>
  /* Add some basic styling for the public parts of your site */
  .app-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  .public-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background-color: #fff;
    border-bottom: 1px solid #eaeaea;
  }
  .logo {
    font-weight: bold;
    font-size: 1.5rem;
    text-decoration: none;
    color: #333;
  }
  nav a {
    margin-left: 1.5rem;
    text-decoration: none;
    color: #555;
  }
  .login-button {
    border: 1px solid #007bff;
    padding: 0.5rem 1rem;
    border-radius: 5px;
    color: #007bff;
  }
  .public-main {
    flex-grow: 1; /* This makes the main content take up available space */
    padding: 2rem;
  }
  .public-footer {
    text-align: center;
    padding: 1.5rem;
    background-color: #f8f9fa;
    color: #6c757d;
    border-top: 1px solid #eaeaea;
  }
</style>