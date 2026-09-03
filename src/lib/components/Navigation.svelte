<script lang="ts">
  import { page } from '$app/stores';
  import { onMount, onDestroy } from 'svelte';
  
  const navItems = [
    { name: 'Games', path: '/' },
    { name: 'Library', path: '/library' },
    { name: 'Media', path: '/media' },
    { name: 'Settings', path: '/settings' }
  ];
  
  let currentTime = $state('');
  let timer: number;
  
  onMount(() => {
    const updateTime = () => {
      currentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    };
    updateTime();
    timer = setInterval(updateTime, 60000); // update every minute
  });
  
  onDestroy(() => {
    if (timer) clearInterval(timer);
  });
</script>

<header class="fixed top-0 left-0 w-full h-20 px-12 flex items-center justify-between z-50 bg-gradient-to-b from-gaming-background to-transparent">
  <div class="flex items-center gap-4">
    <div class="w-10 h-10 rounded-full bg-gaming-surface border border-gaming-border flex items-center justify-center">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gaming-accent"><path d="M12 2v20"/><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
    </div>
    <h1 class="text-xl font-bold tracking-wider">GAMING HUB</h1>
  </div>
  
  <nav class="flex gap-8">
    {#each navItems as item}
      <a 
        href={item.path}
        class="text-lg uppercase tracking-widest transition-colors duration-200 py-2 border-b-2 
               {$page.url.pathname === item.path ? 'border-gaming-text text-gaming-text' : 'border-transparent text-gaming-muted hover:text-gaming-text'}"
      >
        {item.name}
      </a>
    {/each}
  </nav>
  
  <div class="flex items-center gap-6 text-gaming-muted">
    <span>{currentTime}</span>
    <div class="flex items-center gap-2">
      <div class="w-8 h-8 rounded-full bg-gaming-surface flex items-center justify-center">
        <!-- Settings icon / user icon -->
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      </div>
    </div>
  </div>
</header>
