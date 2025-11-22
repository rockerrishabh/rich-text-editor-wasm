/**
 * Logging utilities for debugging and error tracking
 */
export type LogLevel = "debug" | "info" | "warn" | "error" | "none";
export interface LoggingConfig {
    enabled?: boolean;
    level?: LogLevel;
    prefix?: string;
    logger?: CustomLogger;
    maxHistorySize?: number;
}
export interface CustomLogger {
    debug?: (...args: any[]) => void;
    info?: (...args: any[]) => void;
    warn?: (...args: any[]) => void;
    error?: (...args: any[]) => void;
}
interface LogEntry {
    level: LogLevel;
    timestamp: number;
    message: string;
    args: any[];
}
declare class Logger {
    private config;
    private history;
    constructor();
    configure(config: LoggingConfig): void;
    debug(message: string, ...args: any[]): void;
    info(message: string, ...args: any[]): void;
    warn(message: string, ...args: any[]): void;
    error(message: string, ...args: any[]): void;
    private log;
    getHistory(): LogEntry[];
    clearHistory(): void;
    exportLogs(): string;
}
export declare const logger: Logger;
export declare function createLogger(config?: LoggingConfig): Logger;
export {};
//# sourceMappingURL=logger.d.ts.map