<!-- src/routes/(public)/+page.svelte -->

<div class="hero">
  <h1>Selamat Datang di Aksara</h1>
  <p>Aplikasi Kendali dan Supervisi Klien Pemasyarakatan</p>
  <a href="/login" class="cta-button">Masuk ke Aplikasi</a>
</div>

<style>
  .hero {
    text-align: center;
    padding: 4rem 1rem;
  }
  h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }
  p {
    font-size: 1.25rem;
    color: #555;
    margin-bottom: 2rem;
  }
  .cta-button {
    background-color: #007bff;
    color: white;
    padding: 1rem 2rem;
    text-decoration: none;
    border-radius: 5px;
    font-size: 1.1rem;
  }
</style>