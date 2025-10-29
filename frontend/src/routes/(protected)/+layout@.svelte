<!-- src/routes/(protected)/+layout.svelte -->
<script lang="ts">
  // --- SVELTEKIT & LAYOUT IMPORTS ---
  import { onMount } from 'svelte';
  import { authToken } from '$lib/stores';
  import { page } from '$app/stores';
  
  // --- SHADCN BLOCK IMPORTS ---
  import AppSidebar from "$lib/components/app-sidebar.svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  
  // --- ICON IMPORTS ---
  import { BotIcon, SquareTerminalIcon, Settings2Icon, Building, UserCheck } from 'lucide-svelte';

  // --- DATA FETCHING AND STATE ---
  let userProfile: any = null;

  // Define the navigation structure that `app-sidebar` expects.
  const navMain = [
    { title: "Dashboard", url: "/dashboard", icon: SquareTerminalIcon, items: [] },
    {
      title: "Manajemen",
      url: "#",
      icon: BotIcon,
      items: [
        { title: "Bapas", url: "/bapas" },
        { title: "Pengguna", url: "/users" },
      ]
    },
    { title: "Klien", url: "/klien", icon: UserCheck, items: [] },
    { title: "Pengaturan", url: "/settings", icon: Settings2Icon, items: [] },
  ];

  // Placeholder data for components we don't use yet.
  const placeholderTeams = [{ name: "Aksara Indonesia", logo: null, plan: "v.1.4.0 - ariaflutter" }];
  const placeholderProjects: any[] = [];

  // Fetch the real user data on mount.
  onMount(async () => {
    const token = localStorage.getItem('authToken');
    if (!token) {
      window.location.href = '/login';
      return;
    }
    
    try {
      const response = await fetch('http://127.0.0.1:3000/api/auth/me', {
        headers: { 'Authorization': `Bearer ${token}` }
      });
      if (response.ok) {
        const apiUser = await response.json();
        // Adapt the API response to the shape the `NavUser` component expects
        userProfile = {
            name: apiUser.nama,
            nip: apiUser.nip || 'N/A',
            avatar: '/missing_avatar.svg' // A placeholder
        };
      } else {
        authToken.set(null);
        window.location.href = '/login';
      }
    } catch (e) {
      console.error("Failed to fetch user profile", e);
    }
  });

  // Function for the logout button, which might be in one of the child components
  function handleLogout() {
      authToken.set(null);
      window.location.href = '/login';
  }
</script>

{#if $authToken && userProfile}
  <!-- This is your full layout structure, now living in the correct file -->
  <Sidebar.Provider>
    
    <!-- Pass the real data to the AppSidebar component -->
    <AppSidebar 
        user={userProfile} 
        navMain={navMain}
        teams={placeholderTeams}
        projects={placeholderProjects}
    />

    <Sidebar.Inset>
      <header
        class="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex h-16 shrink-0 items-center justify-between gap-2 transition-[width,height] ease-linear px-4"
      >
        <div class="flex items-center gap-2">
            <Sidebar.Trigger class="-ml-1" />
            <Separator orientation="vertical" class="mr-2 data-[orientation=vertical]:h-4" />
            
            <!-- Dynamic Breadcrumbs -->
            <Breadcrumb.Root>
                <Breadcrumb.List>
                <Breadcrumb.Item>
                    <Breadcrumb.Link href="/dashboard">Home</Breadcrumb.Link>
                </Breadcrumb.Item>
                <Breadcrumb.Separator />
                <Breadcrumb.Item>
                    <Breadcrumb.Page class="capitalize">{ $page.url.pathname.split('/').pop() }</Breadcrumb.Page>
                </Breadcrumb.Item>
                </Breadcrumb.List>
            </Breadcrumb.Root>
        </div>
        
      </header>
      <main class="flex-1 p-4 pt-0">
        <!-- This is where your page content (/dashboard, /bapas, etc.) will render -->
        <slot />
      </main>
    </Sidebar.Inset>
  </Sidebar.Provider>
{:else}
  <!-- Show a loading screen while checking the session -->
  <div class="flex h-screen w-full items-center justify-center">
    <p>Loading session...</p>
  </div>
{/if}