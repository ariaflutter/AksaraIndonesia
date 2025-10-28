<script lang="ts">
  import { onMount } from 'svelte';
  import { authToken } from '$lib/stores';

  // Define a type for our Bapas object to get TypeScript benefits
  type Bapas = {
    id: number;
    nama_bapas: string;
    kota: string;
    alamat: string | null;
    nomor_telepon_bapas: string | null;
    email: string | null;
    kanwil: string | null;
  };

  // State for the list of Bapas offices
  let bapasList: Bapas[] = [];
  let isLoading = true;
  let errorMessage = '';

  // State for the "Create New Bapas" form
  let newBapas: Partial<Bapas> = {
    nama_bapas: '',
    kota: '',
    kanwil: '',
    alamat: '',
    email: '',
    nomor_telepon_bapas: ''
  };

    let isModalOpen = false;
  // Use `Partial<Bapas>` to allow for an initially empty object
  let editingBapas: Partial<Bapas> = {}; 


  // --- Data Fetching ---
  async function fetchBapas() {
    isLoading = true;
    const token = $authToken;
    if (!token) return;

    try {
      const response = await fetch('http://127.0.0.1:3000/api/bapas', {
        headers: { Authorization: `Bearer ${token}` }
      });
      if (!response.ok) throw new Error('Gagal mengambil data Bapas.');
      bapasList = await response.json();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Unknown error';
    } finally {
      isLoading = false;
    }
  }

  // --- Form Submission ---
  async function handleCreateBapas() {
    const token = $authToken;
    if (!token) return;

    try {
      const response = await fetch('http://127.0.0.1:3000/api/bapas', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(newBapas)
      });

      if (!response.ok) throw new Error('Gagal membuat Bapas baru.');

      // Success! Reset the form and refresh the list.
      newBapas = { nama_bapas: '', kota: '', kanwil: '' };
      await fetchBapas(); // Refresh the list to show the new entry

    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Unknown error';
    }
  }

  async function handleDelete(id: number) {
    // A simple browser confirmation dialog is good practice
    if (!confirm('Apakah Anda yakin ingin menghapus Bapas ini?')) {
      return;
    }

    const token = $authToken;
    if (!token) return;

    try {
      const response = await fetch(`http://127.0.0.1:3000/api/bapas/${id}`, {
        method: 'DELETE',
        headers: { Authorization: `Bearer ${token}` }
      });

      if (!response.ok) {
        throw new Error('Gagal menghapus Bapas.');
      }

      // If successful, remove the item from our local list to update the UI instantly.
      bapasList = bapasList.filter(bapas => bapas.id !== id);

    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Unknown error';
    }
  }

  // --- ADD NEW HANDLERS FOR THE EDIT MODAL ---
  function openEditModal(bapasToEdit: Bapas) {
    // Create a *copy* of the object to edit, so we don't change the table row directly.
    editingBapas = { ...bapasToEdit };
    isModalOpen = true;
  }

  function closeEditModal() {
    isModalOpen = false;
    editingBapas = {}; // Clear the object
  }

  async function handleUpdate() {
    if (!editingBapas.id) return;

    const token = $authToken;
    if (!token) return;
    
    try {
      const response = await fetch(`http://127.0.0.1:3000/api/bapas/${editingBapas.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(editingBapas)
      });

      if (!response.ok) {
        throw new Error('Gagal mengupdate Bapas.');
      }

      const updatedBapas = await response.json();

      // Find the index of the old item and replace it with the updated one.
      const index = bapasList.findIndex(b => b.id === updatedBapas.id);
      if (index !== -1) {
        bapasList[index] = updatedBapas;
      }

      closeEditModal(); // Close the modal on success

    } catch (error) {
      // You could show an error inside the modal here
      errorMessage = error instanceof Error ? error.message : 'Unknown error';
    }
  }

  // onMount is a lifecycle function that runs when the component is first created.
  // We'll fetch the data here.
  onMount(fetchBapas);

</script>

<h2>Manajemen Bapas</h2>

<!-- Form for creating a new Bapas -->
<div class="form-container">
  <h3>Buat Bapas Baru</h3>
  <form on:submit|preventDefault={handleCreateBapas}>
    <input type="text" placeholder="Nama Bapas" bind:value={newBapas.nama_bapas} required />
    <input type="text" placeholder="Kota" bind:value={newBapas.kota} required />
    <input type="text" placeholder="Kanwil" bind:value={newBapas.kanwil} required />
    <button type="submit">Tambah Bapas</button>
  </form>
</div>


<!-- Table to display the list of Bapas -->
<div class="table-container">
  {#if isLoading}
    <p>Loading data Bapas...</p>
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
        {#each bapasList as bapas}
          <tr>
            <td>{bapas.id}</td>
            <td>{bapas.nama_bapas}</td>
            <td>{bapas.kota}</td>
            <td>{bapas.kanwil}</td>
            <td>
                <button class="edit" on:click={() => openEditModal(bapas)}>Edit</button>
              <button class="delete" on:click={() => handleDelete(bapas.id)}>Delete</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
{#if isModalOpen}
  <div class="modal-overlay" on:click={closeEditModal}>
    <div class="modal-content" on:click|stopPropagation>
      <h3>Edit Bapas</h3>
      <form on:submit|preventDefault={handleUpdate}>
        <div class="form-group">
          <label for="edit-nama">Nama Bapas</label>
          <input id="edit-nama" type="text" bind:value={editingBapas.nama_bapas} required />
        </div>
        <div class="form-group">
          <label for="edit-kota">Kota</label>
          <input id="edit-kota" type="text" bind:value={editingBapas.kota} required />
        </div>
        <div class="form-group">
          <label for="edit-kanwil">Kanwil</label>
          <input id="edit-kanwil" type="text" bind:value={editingBapas.kanwil} required />
        </div>
        <!-- Add other fields like alamat, email etc. here if you want them to be editable -->
        <div class="modal-actions">
          <button type="button" class="cancel" on:click={closeEditModal}>Batal</button>
          <button type="submit" class="save">Simpan Perubahan</button>
        </div>
      </form>
    </div>
  </div>
{/if}


<style>
  h2 {
    margin-bottom: 2rem;
  }
  .form-container, .table-container {
    padding: 1.5rem;
    border: 1px solid #eee;
    border-radius: 8px;
    margin-bottom: 2rem;
  }
  input {
    padding: 0.5rem;
    margin-right: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  th {
    background-color: #f2f2f2;
  }
  .edit { background-color: #ffc107; }
  .delete { background-color: #dc3545; color: white; }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .modal-content {
    background-color: white;
    padding: 2rem;
    border-radius: 8px;
    width: 90%;
    max-width: 500px;
  }
  .modal-content h3 {
    margin-top: 0;
  }
  .modal-actions {
    margin-top: 1.5rem;
    display: flex;
    justify-content: flex-end;
  }
  .modal-actions button {
    margin-left: 0.5rem;
  }
  .cancel {
    background-color: #6c757d;
  }
  .save {
    background-color: #28a745;
  }
</style>