<script lang="ts">
  import { onMount } from 'svelte';
  import { authToken } from '$lib/stores';

  // --- Types ---
  type Bapas = {
    id: number;
    nama_bapas: string;
    kota: string;
    alamat: string | null;
    nomor_telepon_bapas: string | null;
    email: string | null;
    kanwil: string | null;
  };

  // --- State ---
  let bapasList: Bapas[] = [];
  let isLoading = true;
  let errorMessage = '';

  let newBapas: Partial<Bapas> = {
    nama_bapas: '',
    kota: '',
    kanwil: '',
    alamat: '',
    email: '',
    nomor_telepon_bapas: ''
  };

  let isModalOpen = false;
  let editingBapas: Partial<Bapas> = {};

  // --- Fetch Data ---
  async function fetchBapas() {
    isLoading = true;
    const token = $authToken;
    if (!token) return;

    try {
      const res = await fetch('http://127.0.0.1:3000/api/bapas', {
        headers: { Authorization: `Bearer ${token}` }
      });
      if (!res.ok) throw new Error('Gagal mengambil data Bapas.');
      bapasList = await res.json();
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : 'Unknown error';
    } finally {
      isLoading = false;
    }
  }

  // --- Create ---
  async function handleCreateBapas() {
    const token = $authToken;
    if (!token) return;

    try {
      const res = await fetch('http://127.0.0.1:3000/api/bapas', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`
        },
        body: JSON.stringify(newBapas)
      });

      if (!res.ok) throw new Error('Gagal membuat Bapas baru.');

      newBapas = { nama_bapas: '', kota: '', kanwil: '' };
      await fetchBapas();
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : 'Unknown error';
    }
  }

  // --- Delete ---
  async function handleDelete(id: number) {
    if (!confirm('Apakah Anda yakin ingin menghapus Bapas ini?')) return;

    const token = $authToken;
    if (!token) return;

    try {
      const res = await fetch(`http://127.0.0.1:3000/api/bapas/${id}`, {
        method: 'DELETE',
        headers: { Authorization: `Bearer ${token}` }
      });
      if (!res.ok) throw new Error('Gagal menghapus Bapas.');

      bapasList = bapasList.filter(b => b.id !== id);
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : 'Unknown error';
    }
  }

  // --- Edit Modal ---
  function openEditModal(bapas: Bapas) {
    editingBapas = { ...bapas };
    isModalOpen = true;
  }

  function closeEditModal() {
    isModalOpen = false;
    editingBapas = {};
  }

  async function handleUpdate() {
    if (!editingBapas.id) return;
    const token = $authToken;
    if (!token) return;

    try {
      const res = await fetch(`http://127.0.0.1:3000/api/bapas/${editingBapas.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`
        },
        body: JSON.stringify(editingBapas)
      });

      if (!res.ok) throw new Error('Gagal mengupdate Bapas.');

      const updated = await res.json();
      const idx = bapasList.findIndex(b => b.id === updated.id);
      if (idx !== -1) bapasList[idx] = updated;

      closeEditModal();
    } catch (err) {
      errorMessage = err instanceof Error ? err.message : 'Unknown error';
    }
  }

  onMount(fetchBapas);
</script>

<!-- --- UI --- -->
<h2>Manajemen Bapas</h2>

<section class="form-container">
  <h3>Buat Bapas Baru</h3>
  <form on:submit|preventDefault={handleCreateBapas}>
    <input placeholder="Nama Bapas" bind:value={newBapas.nama_bapas} required />
    <input placeholder="Kota" bind:value={newBapas.kota} required />
    <input placeholder="Kanwil" bind:value={newBapas.kanwil} required />
    <button type="submit" class="btn-primary">Tambah</button>
  </form>
</section>

<section class="table-container">
  {#if isLoading}
    <p>Memuat data Bapas...</p>
  {:else if errorMessage}
    <p class="error">{errorMessage}</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>Nama Bapas</th>
          <th>Kota</th>
          <th>Kanwil</th>
          <th>Aksi</th>
        </tr>
      </thead>
      <tbody>
        {#each bapasList as b}
          <tr>
            <td>{b.id}</td>
            <td>{b.nama_bapas}</td>
            <td>{b.kota}</td>
            <td>{b.kanwil}</td>
            <td>
              <button class="btn-edit" on:click={() => openEditModal(b)}>Edit</button>
              <button class="btn-delete" on:click={() => handleDelete(b.id)}>Hapus</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</section>

{#if isModalOpen}
  <div class="modal-overlay" on:click={closeEditModal}>
    <div class="modal" on:click|stopPropagation>
      <h3>Edit Bapas</h3>
      <form on:submit|preventDefault={handleUpdate}>
        <label>
          Nama Bapas
          <input bind:value={editingBapas.nama_bapas} required />
        </label>
        <label>
          Kota
          <input bind:value={editingBapas.kota} required />
        </label>
        <label>
          Kanwil
          <input bind:value={editingBapas.kanwil} required />
        </label>

        <div class="modal-actions">
          <button type="button" class="btn-cancel" on:click={closeEditModal}>Batal</button>
          <button type="submit" class="btn-save">Simpan</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  /* --- General --- */
  h2 {
    font-size: 1.8rem;
    margin-bottom: 1.5rem;
  }

  h3 {
    margin-bottom: 1rem;
  }

  input {
    padding: 0.5rem 0.75rem;
    margin-right: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 6px;
  }

  button {
    cursor: pointer;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    transition: background-color 0.2s ease;
  }

  /* --- Buttons --- */
  .btn-primary {
    background: #007bff;
    color: #fff;
  }

  .btn-primary:hover {
    background: #0069d9;
  }

  .btn-edit {
    background: #ffc107;
    color: #222;
    margin-right: 0.25rem;
  }

  .btn-edit:hover {
    background: #e0a800;
  }

  .btn-delete {
    background: #dc3545;
    color: #fff;
  }

  .btn-delete:hover {
    background: #c82333;
  }

  .btn-cancel {
    background: #6c757d;
    color: #fff;
  }

  .btn-cancel:hover {
    background: #5a6268;
  }

  .btn-save {
    background: #28a745;
    color: #fff;
  }

  .btn-save:hover {
    background: #218838;
  }

  /* --- Containers --- */
  .form-container,
  .table-container {
    background: #fff;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
    margin-bottom: 2rem;
  }

  /* --- Table --- */
  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }

  th {
    background: #f8f9fa;
    font-weight: 600;
  }

  tr:hover td {
    background: #f9f9f9;
  }

  /* --- Modal --- */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background: #fff;
    border-radius: 10px;
    padding: 2rem;
    width: 90%;
    max-width: 480px;
  }

  .modal h3 {
    margin-top: 0;
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.75rem;
  }

  label input {
    width: 100%;
    margin-top: 0.25rem;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 1.5rem;
  }

  .error {
    color: #dc3545;
  }
</style>
