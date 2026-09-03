<script lang="ts">
  import { games, type Game } from '$lib/stores/games';
  import GameCard from '$lib/components/GameCard.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  
  let searchQuery = $state('');
  let selectedCategory = $state('All');
  let isScanning = $state(false);
  
  async function handleManualAdd() {
    try {
      const selectedPath = await open({
        multiple: false,
        directory: false,
        filters: [{
          name: 'Executables',
          extensions: ['exe', 'app', 'sh', 'x86_64']
        }]
      });
      
      if (selectedPath && typeof selectedPath === 'string') {
        // Extract basic name from path
        const fileName = selectedPath.split(/[/\\]/).pop() || 'Unknown Game';
        const gameName = fileName.replace(/\.(exe|app|sh|x86_64)$/i, '');
        
        if (!$games.some(g => g.executable === selectedPath)) {
          await games.addGame({
            id: Math.random().toString(36).substring(2, 9),
            name: gameName,
            executable: selectedPath,
            installed: true,
            totalPlaytimeSeconds: 0,
            categories: ['Manual']
          });
        }
      }
    } catch (e) {
      console.error('Failed to open dialog:', e);
    }
  }
  
  async function handleScan() {
    isScanning = true;
    try {
      const scannedGames = await invoke<any[]>('scan_games');
      for (const sg of scannedGames) {
        // Prevent duplicate addition based on executable path
        if (!$games.some(g => g.executable === sg.executable)) {
          await games.addGame({
            id: Math.random().toString(36).substring(2, 9),
            name: sg.name,
            executable: sg.executable,
            cover: sg.cover || undefined,
            installed: true,
            totalPlaytimeSeconds: 0,
            categories: ['Discovered']
          });
        }
      }
    } catch (e) {
      console.error('Failed to scan games:', e);
    }
    isScanning = false;
  }
  
  // Available categories based on library content
  let categories = $derived(['All', 'Favorites', ...new Set($games.flatMap(g => g.categories || []))]);
  
  let filteredGames = $derived($games.filter(g => {
    const matchesSearch = g.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesCategory = selectedCategory === 'All' 
                            || (selectedCategory === 'Favorites' && g.favorite)
                            || (g.categories && g.categories.includes(selectedCategory));
    return matchesSearch && matchesCategory;
  }));
</script>

<div class="h-full px-12 pt-8 pb-12 flex flex-col animate-in fade-in duration-500">
  <div class="flex items-center justify-between mb-8 flex-none">
    <h2 class="text-4xl font-bold tracking-tight">Game Library</h2>
    
    <div class="flex items-center gap-4">
      <div class="relative">
        <input 
          type="text" 
          placeholder="Search games..." 
          bind:value={searchQuery}
          class="bg-gaming-surface border border-gaming-border rounded-full py-2 px-6 pl-10 text-gaming-text focus:outline-none focus:border-gaming-accent transition-colors w-64"
        />
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="absolute left-4 top-1/2 -translate-y-1/2 text-gaming-muted"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
      </div>
      
      <button 
        class="glass-panel text-white px-6 py-2 rounded-full font-bold hover:bg-white/10 transition-colors"
        onclick={handleManualAdd}
      >
        Add Manual
      </button>
      
      <button 
        class="bg-gaming-accent text-white px-6 py-2 rounded-full font-bold hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/20 flex items-center gap-2"
        onclick={handleScan}
        disabled={isScanning}
      >
        {#if isScanning}
          <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Scanning...
        {:else}
          Scan PC
        {/if}
      </button>
    </div>
  </div>
  
  <div class="flex gap-4 mb-8 overflow-x-auto pb-2 flex-none hide-scrollbar">
    {#each categories as category}
      <button 
        class="px-6 py-2 rounded-full font-semibold transition-all border 
               {selectedCategory === category 
                 ? 'bg-gaming-text text-gaming-background border-transparent shadow-[0_0_15px_rgba(255,255,255,0.2)]' 
                 : 'glass-panel text-gaming-muted hover:text-white'}"
        onclick={() => selectedCategory = category}
      >
        {category}
      </button>
    {/each}
  </div>
  
  <div class="flex-1 overflow-y-auto pr-4 custom-scrollbar min-h-0">
    {#if filteredGames.length === 0}
      <div class="h-full flex flex-col items-center justify-center text-gaming-muted gap-4">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="opacity-50"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 3v18"/><path d="M17 3v18"/><path d="M3 7h18"/><path d="M3 17h18"/></svg>
        <p class="text-xl">No games found.</p>
        {#if $games.length === 0}
          <p class="text-sm">Click "Scan PC" or add games manually to populate your library.</p>
        {/if}
      </div>
    {:else}
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-8 gap-6 pb-20">
        {#each filteredGames as game}
          <div class="w-full">
            <GameCard {game} />
          </div>
        {/each}
      </div>
    {/if}
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
  
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(24, 24, 27, 0.5);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.4);
  }
</style>
