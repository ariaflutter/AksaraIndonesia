<!-- src/routes/+layout.svelte -->
<script>
  import "../app.css";
</script>

  <main class="public-main">
    <!-- All pages, public and protected, will be rendered inside this slot -->
    <slot />
  </main>
