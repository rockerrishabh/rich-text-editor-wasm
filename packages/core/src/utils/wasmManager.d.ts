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
/**
 * Global WASM module manager
 */
declare class WasmModuleManager {
    private state;
    /**
     * Register a new editor instance using the WASM module
     */
    registerInstance(): void;
    /**
     * Unregister an editor instance
     */
    unregisterInstance(): void;
    /**
     * Get the current reference count
     */
    getRefCount(): number;
    /**
     * Get uptime in milliseconds
     */
    getUptime(): number;
    /**
     * Get memory statistics for the WASM module
     */
    getMemoryStats(): {
        refCount: number;
        uptimeMs: number;
        estimatedSharedModuleKB: number;
    };
    /**
     * Force reset the WASM module state (for testing purposes)
     * @internal
     */
    reset(): void;
}
/**
 * Global singleton instance
 */
export declare const wasmManager: WasmModuleManager;
/**
 * Register a new editor instance
 * Call this when creating a new RichTextEditor
 */
export declare function registerWasmInstance(): void;
/**
 * Unregister an editor instance
 * Call this when destroying a RichTextEditor
 */
export declare function unregisterWasmInstance(): void;
/**
 * Get WASM module memory statistics
 */
export declare function getWasmMemoryStats(): {
    refCount: number;
    uptimeMs: number;
    estimatedSharedModuleKB: number;
};
/**
 * Get the number of active editor instances
 */
export declare function getActiveInstanceCount(): number;
export {};
//# sourceMappingURL=wasmManager.d.ts.map