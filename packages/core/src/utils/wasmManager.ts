/**
 * WASM Module Manager
 *
 * Manages WASM module reference counting across multiple editor instances
 * to track memory usage and provide statistics.
 *
 * Note: The WASM module itself is shared automatically by the browser's module system
 * when imported. This manager tracks references for monitoring and debugging purposes.
 * Each editor instance creates its own WasmDocument, but they all share the same
 * underlying WASM module code, minimizing memory overhead.
 */

import { logger } from "./logger";

/**
 * WASM module state
 */
interface WasmModuleState {
  refCount: number;
  createdAt: number;
}

/**
 * Global WASM module manager
 */
class WasmModuleManager {
  private state: WasmModuleState = {
    refCount: 0,
    createdAt: Date.now(),
  };

  /**
   * Register a new editor instance using the WASM module
   */
  registerInstance(): void {
    this.state.refCount++;
    logger.debug(
      `WASM module instance registered, refCount: ${this.state.refCount}`
    );
  }

  /**
   * Unregister an editor instance
   */
  unregisterInstance(): void {
    if (this.state.refCount > 0) {
      this.state.refCount--;
      logger.debug(
        `WASM module instance unregistered, refCount: ${this.state.refCount}`
      );
    }
  }

  /**
   * Get the current reference count
   */
  getRefCount(): number {
    return this.state.refCount;
  }

  /**
   * Get uptime in milliseconds
   */
  getUptime(): number {
    return Date.now() - this.state.createdAt;
  }

  /**
   * Get memory statistics for the WASM module
   */
  getMemoryStats(): {
    refCount: number;
    uptimeMs: number;
    estimatedSharedModuleKB: number;
  } {
    // The WASM module itself is shared across all instances
    // Only the first instance pays the cost of loading it
    // Subsequent instances reuse the same module code
    const baseModuleSize = 100; // ~100KB for the shared WASM module code

    return {
      refCount: this.state.refCount,
      uptimeMs: this.getUptime(),
      estimatedSharedModuleKB: baseModuleSize,
    };
  }

  /**
   * Force reset the WASM module state (for testing purposes)
   * @internal
   */
  reset(): void {
    logger.warn("Resetting WASM module manager");
    this.state = {
      refCount: 0,
      createdAt: Date.now(),
    };
  }
}

/**
 * Global singleton instance
 */
export const wasmManager = new WasmModuleManager();

/**
 * Register a new editor instance
 * Call this when creating a new RichTextEditor
 */
export function registerWasmInstance(): void {
  wasmManager.registerInstance();
}

/**
 * Unregister an editor instance
 * Call this when destroying a RichTextEditor
 */
export function unregisterWasmInstance(): void {
  wasmManager.unregisterInstance();
}

/**
 * Get WASM module memory statistics
 */
export function getWasmMemoryStats() {
  return wasmManager.getMemoryStats();
}

/**
 * Get the number of active editor instances
 */
export function getActiveInstanceCount(): number {
  return wasmManager.getRefCount();
}
