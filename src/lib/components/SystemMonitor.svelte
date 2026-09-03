<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let stats = $state({
    cpuUsage: 0,
    cpuTemperature: null,
    gpuUsage: 0,
    gpuTemperature: null,
    ramUsed: 0,
    ramTotal: 16,
    vramUsed: 0,
    vramTotal: 8
  });
  
  let timer: any;
  
  onMount(() => {
    timer = setInterval(async () => {
      try {
        const sysStats: any = await invoke('get_system_stats');
        stats.cpuUsage = sysStats.cpu_usage;
        stats.ramUsed = sysStats.ram_used;
        stats.ramTotal = sysStats.ram_total;
      } catch (e) { console.error('get_system_stats error:', e); }
      
      try {
        const hwStats: any = await invoke('get_hwinfo_stats');
        if (hwStats) {
          if (hwStats.cpu_temp) stats.cpuTemperature = hwStats.cpu_temp;
          if (hwStats.gpu_temp) stats.gpuTemperature = hwStats.gpu_temp;
          if (hwStats.gpu_usage) stats.gpuUsage = hwStats.gpu_usage;
          if (hwStats.vram_used) stats.vramUsed = hwStats.vram_used;
          if (hwStats.vram_total) stats.vramTotal = hwStats.vram_total;
        }
      } catch (e) { console.error('get_hwinfo_stats error:', e); }
      
    }, 2000);
  });
  
  onDestroy(() => {
    clearInterval(timer);
  });
</script>

<div class="glass-panel rounded-2xl p-6 h-full flex flex-col">
  <h3 class="text-lg font-bold mb-6 text-gaming-muted tracking-wide">SYSTEM MONITOR</h3>
  
  <div class="flex-1 flex flex-col justify-around gap-4">
    <!-- CPU -->
    <div class="flex items-center gap-4">
      <div class="w-16 font-mono text-gaming-muted">CPU</div>
      <div class="w-16 font-mono text-right">{stats.cpuTemperature ? `${stats.cpuTemperature}°C` : 'N/A'}</div>
      <div class="flex-1 h-3 bg-gaming-border rounded-full overflow-hidden">
        <div class="h-full bg-gaming-accent transition-all duration-500" style="width: {stats.cpuUsage}%"></div>
      </div>
    </div>
    
    <!-- GPU -->
    <div class="flex items-center gap-4">
      <div class="w-16 font-mono text-gaming-muted">GPU</div>
      <div class="w-16 font-mono text-right">{stats.gpuTemperature ? `${stats.gpuTemperature}°C` : 'N/A'}</div>
      <div class="flex-1 h-3 bg-gaming-border rounded-full overflow-hidden">
        <div class="h-full bg-green-500 transition-all duration-500" style="width: {stats.gpuUsage}%"></div>
      </div>
    </div>
    
    <!-- RAM -->
    <div class="flex items-center gap-4">
      <div class="w-16 font-mono text-gaming-muted">RAM</div>
      <div class="w-24 font-mono text-right text-sm">{stats.ramUsed.toFixed(1)}/{stats.ramTotal}GB</div>
      <div class="flex-1 h-3 bg-gaming-border rounded-full overflow-hidden">
        <div class="h-full bg-purple-500 transition-all duration-500" style="width: {(stats.ramUsed / stats.ramTotal) * 100}%"></div>
      </div>
    </div>
    
    <!-- VRAM -->
    <div class="flex items-center gap-4">
      <div class="w-16 font-mono text-gaming-muted">VRAM</div>
      <div class="w-24 font-mono text-right text-sm">{stats.vramUsed.toFixed(1)}/{stats.vramTotal}GB</div>
      <div class="flex-1 h-3 bg-gaming-border rounded-full overflow-hidden">
        <div class="h-full bg-yellow-500 transition-all duration-500" style="width: {(stats.vramUsed / stats.vramTotal) * 100}%"></div>
      </div>
    </div>
  </div>
</div>
