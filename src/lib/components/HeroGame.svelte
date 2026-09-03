  <script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  export let game: any = null;

  async function launchGame() {
    if (!game) return;
    try {
      // In a real scenario, this uses game.executable and game.workingDirectory
      await invoke('launch_game', { executable: game.executable || 'calc.exe', workingDirectory: null });
    } catch (e) {
      console.error('Failed to launch:', e);
    }
  }
</script>

{#if game}
  <div class="relative w-full h-[60vh] rounded-3xl overflow-hidden mb-8 shadow-2xl glass-panel transition-all duration-500 group">
    <div class="absolute inset-0 z-0">
      <!-- Main Cover Image -->
      <div class="absolute inset-0 bg-cover bg-center transition-all duration-1000 blur-sm scale-105 opacity-40" style="background-image: url('{game.background || game.cover}');"></div>
      
      <!-- Gradients for blending -->
      <div class="absolute inset-0 bg-linear-to-t from-gaming-background via-gaming-background/80 to-transparent"></div>
      <div class="absolute inset-0 bg-linear-to-r from-gaming-background via-gaming-background/50 to-transparent"></div>
    </div>  
    
    <!-- Content -->
    <div class="absolute bottom-0 left-0 p-12 w-full max-w-3xl flex flex-col items-start gap-4 z-10">
      <h2 class="text-5xl font-bold tracking-tight text-white drop-shadow-lg">{game.name}</h2>
      
      <p class="text-xl text-gaming-muted">
        {game.description || 'Continue your journey'}
      </p>
      
      <div class="flex items-center gap-6 mt-4">
        <button 
          class="bg-white text-black px-10 py-4 rounded-full font-bold text-lg hover:scale-105 transition-transform duration-200 shadow-[0_0_20px_rgba(255,255,255,0.3)] cursor-pointer"
          on:click={launchGame}
        >
          PLAY GAME
        </button>
        
        <div class="text-gaming-muted text-sm flex flex-col">
          <span>Last played: {game.lastPlayedAt ? new Date(game.lastPlayedAt).toLocaleDateString() : 'Never'}</span>
          <span>Playtime: {Math.floor((game.totalPlaytimeSeconds || 0) / 3600)}h {Math.floor(((game.totalPlaytimeSeconds || 0) % 3600) / 60)}m</span>
        </div>
      </div>
    </div>
  </div>
{:else}
  <div class="w-full h-[60vh] rounded-3xl glass-panel flex items-center justify-center text-gaming-muted mb-8">
    No game selected
  </div>
{/if}
