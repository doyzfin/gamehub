<script lang="ts">
  export let game: any;
  export let selected: boolean = false;
</script>

<button 
  class="relative aspect-[3/4] w-48 rounded-2xl overflow-hidden glass-panel flex-shrink-0 transition-all duration-300 text-left
         {selected ? 'focused-item z-10' : 'hover:scale-[1.02] opacity-80 hover:opacity-100 z-0'}"
>
  {#if game.cover}
    <div 
      class="absolute inset-0 bg-cover bg-center"
      style="background-image: url('{game.cover}');"
    ></div>
  {:else}
    <div class="absolute inset-0 bg-gaming-border flex items-center justify-center p-4 text-center text-xl font-bold">
      {game.name}
    </div>
  {/if}
  
  <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/30 to-transparent"></div>
  
  <div class="absolute bottom-0 left-0 w-full p-4">
    <div class="text-sm font-semibold truncate mb-1">{game.name}</div>
    {#if game.totalPlaytimeSeconds !== undefined}
      <div class="text-xs text-gaming-muted">
        {Math.floor(game.totalPlaytimeSeconds / 3600)}h {Math.floor((game.totalPlaytimeSeconds % 3600) / 60)}m
      </div>
    {/if}
  </div>
</button>
