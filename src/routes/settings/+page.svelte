<script lang="ts">
  import { settings } from '$lib/stores/settings';
  
  let activeTab = $state('General');
  let tabs = ['General', 'Display', 'Games', 'System'];
  
  let syncStatus = $state<'idle' | 'syncing' | 'done'>('idle');
  
  async function handleCloudSync() {
    syncStatus = 'syncing';
    // Mocking cloud sync delay
    await new Promise(resolve => setTimeout(resolve, 2000));
    syncStatus = 'done';
    setTimeout(() => syncStatus = 'idle', 3000);
  }
</script>

<div class="h-full flex pt-8 pb-12 animate-in fade-in duration-500">
  <!-- Side Menu -->
  <div class="w-64 border-r border-gaming-border px-8 flex flex-col gap-2">
    <h2 class="text-3xl font-bold tracking-tight mb-8">Settings</h2>
    
    {#each tabs as tab}
      <button 
        class="text-left py-3 px-4 rounded-xl font-semibold transition-all duration-200
               {activeTab === tab ? 'bg-gaming-surface text-white' : 'text-gaming-muted hover:bg-white/5 hover:text-white'}"
        onclick={() => activeTab = tab}
      >
        {tab}
      </button>
    {/each}
  </div>
  
  <!-- Content Area -->
  <div class="flex-1 px-12 overflow-y-auto custom-scrollbar">
    {#if activeTab === 'General'}
      <div class="max-w-2xl flex flex-col gap-8">
        <section>
          <h3 class="text-xl font-bold mb-4 text-gaming-accent">Startup Behavior</h3>
          <div class="glass-panel p-6 rounded-2xl flex items-center justify-between">
            <div>
              <div class="font-bold text-lg">Start with Windows</div>
              <div class="text-gaming-muted text-sm">Automatically launch Gaming Hub when your PC boots.</div>
            </div>
            <button 
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none {$settings.startWithWindows ? 'bg-gaming-accent' : 'bg-gaming-border'}"
              onclick={() => settings.updateSetting('startWithWindows', !$settings.startWithWindows)}
            >
              <span class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {$settings.startWithWindows ? 'translate-x-6' : 'translate-x-1'}"></span>
            </button>
          </div>
        </section>
        
        <section>
          <h3 class="text-xl font-bold mb-4 text-gaming-accent">Exit Behavior</h3>
          <div class="glass-panel p-6 rounded-2xl flex items-center justify-between">
            <div>
              <div class="font-bold text-lg">Hold ESC Duration</div>
              <div class="text-gaming-muted text-sm">Time required to hold Escape before quitting.</div>
            </div>
            <select 
              class="bg-gaming-surface border border-gaming-border rounded-lg px-4 py-2 text-white focus:outline-none focus:border-gaming-accent"
              value={$settings.exitHoldDuration}
              onchange={(e) => settings.updateSetting('exitHoldDuration', parseInt(e.currentTarget.value))}
            >
              <option value={3000}>3 Seconds</option>
              <option value={5000}>5 Seconds</option>
              <option value={10000}>10 Seconds</option>
            </select>
          </div>
        </section>
      </div>
      
    {:else if activeTab === 'Display'}
      <div class="max-w-2xl flex flex-col gap-8">
        <section>
          <h3 class="text-xl font-bold mb-4 text-gaming-accent">Theme Customization</h3>
          <div class="grid grid-cols-3 gap-4">
            {#each ['dark', 'midnight', 'oled'] as themeName}
              <button 
                class="glass-panel p-4 rounded-xl border-2 transition-all capitalize
                       {$settings.theme === themeName ? 'border-gaming-accent shadow-[0_0_15px_rgba(59,130,246,0.3)]' : 'border-transparent hover:border-gaming-border'}"
                onclick={() => settings.updateSetting('theme', themeName as any)}
              >
                {themeName}
              </button>
            {/each}
          </div>
        </section>
      </div>
      
    {:else if activeTab === 'System'}
      <div class="max-w-2xl flex flex-col gap-8">
        <section>
          <h3 class="text-xl font-bold mb-4 text-gaming-accent">Cloud Sync</h3>
          <div class="glass-panel p-6 rounded-2xl flex flex-col gap-4">
            <div>
              <div class="font-bold text-lg">Backup Configuration</div>
              <div class="text-gaming-muted text-sm">Save your game library and preferences to the cloud.</div>
            </div>
            <button 
              class="bg-gaming-surface border border-gaming-border hover:border-gaming-accent px-6 py-3 rounded-xl font-bold transition-all w-fit flex items-center gap-2"
              onclick={handleCloudSync}
              disabled={syncStatus === 'syncing'}
            >
              {#if syncStatus === 'idle'}
                Sync Now
              {:else if syncStatus === 'syncing'}
                <svg class="animate-spin h-5 w-5 text-gaming-accent" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Syncing...
              {:else}
                <span class="text-green-400">Sync Complete!</span>
              {/if}
            </button>
          </div>
        </section>
        
        <section>
          <h3 class="text-xl font-bold mb-4 text-gaming-accent">About</h3>
          <div class="glass-panel p-6 rounded-2xl flex items-center justify-between">
            <div>
              <div class="font-bold text-lg">Gaming Hub</div>
              <div class="text-gaming-muted text-sm">Version 1.0.0</div>
            </div>
            <button class="bg-gaming-accent text-white px-6 py-2 rounded-full font-bold hover:bg-blue-600 transition-colors">
              Check for Updates
            </button>
          </div>
        </section>
      </div>
    {/if}
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>
