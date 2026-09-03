<script lang="ts">
  import HeroGame from '$lib/components/HeroGame.svelte';
  import GameCard from '$lib/components/GameCard.svelte';
  import SystemMonitor from '$lib/components/SystemMonitor.svelte';
  import PowerMenu from '$lib/components/PowerMenu.svelte';
  
  import { games } from '$lib/stores/games';
  
  let selectedGameIndex = $state(0);
  
  function handleKeydown(e: KeyboardEvent) {
    if ($games.length === 0) return;
    if (e.key === 'ArrowRight') {
      selectedGameIndex = Math.min(selectedGameIndex + 1, $games.length - 1);
    } else if (e.key === 'ArrowLeft') {
      selectedGameIndex = Math.max(selectedGameIndex - 1, 0);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="px-12 pb-12 h-full flex flex-col pt-8 animate-in fade-in duration-500">
  <!-- Hero Section -->
  <div class="flex-none">
    <HeroGame game={$games[selectedGameIndex]} />
  </div>
  
  <!-- Content Grid -->
  <div class="flex-1 grid grid-cols-12 gap-8 min-h-0">
    
    <!-- Games List -->
    <div class="col-span-8 flex flex-col h-full overflow-hidden">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-xl font-bold tracking-wide">Your Games</h3>
        <a href="/library" class="text-gaming-accent hover:text-white transition-colors">View All</a>
      </div>
      
      <div class="flex gap-6 overflow-x-auto pb-4 pt-2 px-2 -mx-2 hide-scrollbar">
        {#each $games as game, i}
          <GameCard 
            {game} 
            selected={i === selectedGameIndex} 
            on:click={() => selectedGameIndex = i}
          />
        {/each}
        
        <!-- Add Game button -->
        <button class="aspect-[3/4] w-48 rounded-2xl glass-panel flex-shrink-0 flex flex-col items-center justify-center gap-4 hover:bg-white/5 transition-colors border-dashed border-2 border-gaming-border hover:border-gaming-accent text-gaming-muted hover:text-white group">
          <div class="w-12 h-12 rounded-full border-2 border-current flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">
            +
          </div>
          <span class="font-bold">Add Game</span>
        </button>
      </div>
    </div>
    
    <!-- System Monitor & Power -->
    <div class="col-span-4 grid grid-rows-2 gap-8 h-full">
      <SystemMonitor />
      <PowerMenu />
    </div>
    
  </div>
</div>

<style>
  .hide-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .hide-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>
