<!-- src/lib/components/blocks/LoginForm.svelte -->
<script lang="ts">
	// Svelte 5 Imports
	let { class: className, ...restProps } = $props();

	// Component Imports
	import * as Card from "$lib/components/ui/card/index.js";
	import { FieldGroup, Field, FieldLabel, FieldDescription } from "$lib/components/ui/field/index.js";
	import { Input } from "$lib/components/ui/input/index.js";
	import { Button } from "$lib/components/ui/button/index.js";
	import { cn } from "$lib/utils.js";
	import { authToken } from '$lib/stores';
	 import { Toast } from 'flowbite-svelte';
    import { ExclamationCircleOutline } from 'flowbite-svelte-icons'; // Optional icon for the toast

    // --- LOGIC AND STATE (SVELTE 5 RUNES SYNTAX) ---
    let nip = $state('');
    let password = $state('');
    let errorMessage = $state('');
    let isLoading = $state(false);

    async function handleLogin() {
        isLoading = true;
        errorMessage = '';
        try {
            const response = await fetch('http://127.0.0.1:3000/api/auth/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ nip, password }),
            });
            if (!response.ok) {
                throw new Error('NIP atau Password salah.');
            }
            const data = await response.json();
            authToken.set(data.token);
            window.location.href = '/dashboard';
        } catch (error) {
            errorMessage = error instanceof Error ? error.message : 'Terjadi kesalahan.';
        } finally {
            isLoading = false;
        }
    }
</script>

<!-- The outer div is just for layout -->
<div class={cn("flex flex-col gap-6", className)} {...restProps}>
	<Card.Root class="overflow-hidden p-0">
		<Card.Content class="grid p-0 md:grid-cols-2">

            <!-- The `onsubmit` handler goes on the <form> tag -->
			<form class="p-6 md:p-8" onsubmit={handleLogin}>
				<FieldGroup>
					<div class="flex flex-col items-center gap-2 text-center">
						<h1 class="text-2xl font-bold">Selamat Datang</h1>
						<p class="text-muted-foreground text-balance">
							Login menggunakan akun Aksara anda
						</p>
					</div>
					<Field>
						<FieldLabel for="nip-input">NIP</FieldLabel>
						<Input id="nip-input" type="text" placeholder="199001012020121001" required bind:value={nip}/>
					</Field>
					<Field>
						<div class="flex items-center">
							<FieldLabel for="password-input">Password</FieldLabel>
							<a href="##" class="ml-auto text-sm underline-offset-2 hover:underline">
								Lupa Password?
							</a>
						</div>
						<Input placeholder="••••••••" id="password-input" type="password" required bind:value={password} />
					</Field>
					<Field>
						{#if errorMessage}
							<p class="text-center text-sm text-red-500">{errorMessage}</p>
						{/if}
						<Button type="submit" class="w-full" disabled={isLoading}>
							{#if isLoading} Loading... {:else} Login {/if}
						</Button>
					</Field>
				</FieldGroup>
			</form>
			<div class="bg-muted relative hidden md:block">
				<img
					src="/placeholder.svg"
					alt="placeholder"
					class="absolute inset-0 h-full w-full object-cover dark:brightness-[0.2] dark:grayscale"
				/>
			</div>
		</Card.Content>
	</Card.Root>
	<FieldDescription class="px-6 text-center">
		created by ariaflutter for Direktorat Jenderal Pemasyarakatan
	</FieldDescription>
</div>