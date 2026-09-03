<script lang="ts">
  import { onMount } from 'svelte';
  
  let mediaFiles = $state<any[]>([]);
  
  onMount(() => {
    // Media files will be populated dynamically from user's captures directory
    mediaFiles = [];
  });
</script>

<div class="h-full px-12 pt-8 pb-12 flex flex-col animate-in fade-in duration-500">
  <div class="flex items-center justify-between mb-8 flex-none">
    <h2 class="text-4xl font-bold tracking-tight">Media Gallery</h2>
    <div class="text-gaming-muted">Captures & Recordings</div>
  </div>
  
  <div class="flex-1 overflow-y-auto pr-4 custom-scrollbar min-h-0">
    {#if mediaFiles.length === 0}
      <div class="h-full flex flex-col items-center justify-center text-gaming-muted gap-4">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="opacity-50"><rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>
        <p class="text-xl">No media found.</p>
        <p class="text-sm">Screenshots and gameplay clips will appear here.</p>
      </div>
    {:else}
      <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 pb-20">
        {#each mediaFiles as media}
          <div class="group relative aspect-video rounded-xl overflow-hidden glass-panel cursor-pointer hover:scale-[1.02] transition-transform shadow-lg">
            <div 
              class="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-110"
              style="background-image: url('{media.url}');"
            ></div>
            <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            
            {#if media.type === 'video'}
              <div class="absolute top-3 right-3 bg-black/60 rounded-full p-2 backdrop-blur-md">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="white" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 3 19 12 5 21 5 3"/></svg>
              </div>
            {/if}
            
            <div class="absolute bottom-0 left-0 p-4 opacity-0 group-hover:opacity-100 transition-opacity duration-300 w-full translate-y-2 group-hover:translate-y-0">
              <div class="text-sm font-bold truncate">{media.game}</div>
              <div class="text-xs text-gaming-muted">{media.date}</div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
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
