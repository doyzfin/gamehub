export class GamepadManager {
  private animationFrame: number | null = null;
  private prevButtons: boolean[] = [];
  private prevAxes: number[] = [];
  
  // Standard Gamepad API mapping
  // 0: A/Cross, 1: B/Circle, 12: D-Up, 13: D-Down, 14: D-Left, 15: D-Right
  
  private lastInputTime = 0;
  private readonly THROTTLE_MS = 150; // Prevent rapid-fire inputs from analog sticks
  
  start() {
    this.loop();
  }
  
  stop() {
    if (this.animationFrame !== null) {
      cancelAnimationFrame(this.animationFrame);
    }
  }
  
  private loop = () => {
    const gamepads = navigator.getGamepads ? navigator.getGamepads() : [];
    
    // Use the first connected gamepad
    const gp = gamepads.find(pad => pad !== null);
    
    if (gp) {
      const now = performance.now();
      const canTrigger = now - this.lastInputTime > this.THROTTLE_MS;
      
      // Buttons
      gp.buttons.forEach((button, i) => {
        const pressed = typeof button === 'object' ? button.pressed : button === 1.0;
        const justPressed = pressed && !this.prevButtons[i];
        
        if (justPressed) {
          if (i === 0) this.dispatchKey('Enter'); // A / Cross
          if (i === 1) this.dispatchKey('Escape'); // B / Circle
          
          if (i === 12) this.dispatchKey('ArrowUp'); // D-Pad Up
          if (i === 13) this.dispatchKey('ArrowDown'); // D-Pad Down
          if (i === 14) this.dispatchKey('ArrowLeft'); // D-Pad Left
          if (i === 15) this.dispatchKey('ArrowRight'); // D-Pad Right
        }
        this.prevButtons[i] = pressed;
      });
      
      // Analog Sticks (Left Stick typically axes 0 and 1)
      if (canTrigger) {
        let axisTriggered = false;
        
        // X-Axis
        if (gp.axes[0] > 0.5) { this.dispatchKey('ArrowRight'); axisTriggered = true; }
        else if (gp.axes[0] < -0.5) { this.dispatchKey('ArrowLeft'); axisTriggered = true; }
        
        // Y-Axis
        if (gp.axes[1] > 0.5) { this.dispatchKey('ArrowDown'); axisTriggered = true; }
        else if (gp.axes[1] < -0.5) { this.dispatchKey('ArrowUp'); axisTriggered = true; }
        
        if (axisTriggered) {
          this.lastInputTime = now;
        }
      }
    }
    
    this.animationFrame = requestAnimationFrame(this.loop);
  };
  
  private dispatchKey(key: string) {
    const event = new KeyboardEvent('keydown', {
      key,
      code: key,
      bubbles: true,
      cancelable: true
    });
    
    // Special handling for ExitOverlay which binds to window directly in some contexts
    window.dispatchEvent(event);
    
    // Also dispatch to active element for standard UI interactions (like buttons)
    if (key === 'Enter' && document.activeElement && document.activeElement instanceof HTMLElement) {
      document.activeElement.click();
    }
  }
}

export const gamepadManager = new GamepadManager();
