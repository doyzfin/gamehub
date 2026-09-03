<script lang="ts">
  import '../app.css';
  import Navigation from '$lib/components/Navigation.svelte';
  import ExitOverlay from '$lib/components/ExitOverlay.svelte';
  
  import { onMount, onDestroy } from 'svelte';
  import { games } from '$lib/stores/games';
  import { settings } from '$lib/stores/settings';
  import { gamepadManager } from '$lib/utils/gamepad';
  
  onMount(() => {
    games.init();
    settings.init();
    gamepadManager.start();
  });

  onDestroy(() => {
    gamepadManager.stop();
  });
</script>

<div 
  class="h-screen w-screen bg-gaming-background text-gaming-text overflow-hidden relative transition-colors duration-500"
  data-theme={$settings.theme}
>
  <!-- Background Ambient Glow -->
  <div class="absolute inset-0 pointer-events-none z-0 overflow-hidden">
    <div class="absolute -top-1/4 -right-1/4 w-250 h-250 bg-gaming-accent opacity-10 blur-[150px] rounded-full"></div>
  </div>
  
  <Navigation />
  
  <main class="w-full h-full pt-20">
    <slot />
  </main>
  
  <ExitOverlay />
</div>
