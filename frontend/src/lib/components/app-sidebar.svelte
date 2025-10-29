<!-- src/lib/components/app-sidebar.svelte -->
<script lang="ts">
	import NavMain from "./nav-main.svelte";
	import NavProjects from "./nav-projects.svelte";
	import NavUser from "./nav-user.svelte";
	import TeamSwitcher from "./team-switcher.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import type { ComponentProps } from "svelte";

    // --- THIS IS THE FIX ---
    // We define our custom props in an interface.
    interface CustomProps {
        user: any;
        teams: any[];
        navMain: any[];
        projects: any[];
    }
    
    // We combine our custom props with the original Sidebar.Root props.
	let {
        user,
        teams,
        navMain,
        projects,
		ref = $bindable(null),
		collapsible = "icon",
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & CustomProps = $props(); // <-- And merge them here
</script>

<!-- The HTML part of this file remains the same. -->
<!-- It will now correctly receive the props. -->
<Sidebar.Root {collapsible} {...restProps}>
	<Sidebar.Header>
		<TeamSwitcher {teams} />
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain items={navMain} />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser {user} />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>