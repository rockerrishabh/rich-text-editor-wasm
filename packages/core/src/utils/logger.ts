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

class Logger {
  private config: Required<Omit<LoggingConfig, "logger">> & {
    logger?: CustomLogger;
  };
  private history: LogEntry[] = [];

  constructor() {
    this.config = {
      enabled: false,
      level: "warn",
      prefix: "[RTE]",
      maxHistorySize: 100,
    };
  }

  configure(config: LoggingConfig): void {
    this.config = { ...this.config, ...config };
  }

  debug(message: string, ...args: any[]): void {
    this.log("debug", message, ...args);
  }

  info(message: string, ...args: any[]): void {
    this.log("info", message, ...args);
  }

  warn(message: string, ...args: any[]): void {
    this.log("warn", message, ...args);
  }

  error(message: string, ...args: any[]): void {
    this.log("error", message, ...args);
  }

  private log(level: LogLevel, message: string, ...args: any[]): void {
    if (!this.config.enabled || level === "none") {
      return;
    }

    const levelPriority: Record<LogLevel, number> = {
      debug: 0,
      info: 1,
      warn: 2,
      error: 3,
      none: 4,
    };

    if (levelPriority[level] < levelPriority[this.config.level]) {
      return;
    }

    // Add to history
    this.history.push({
      level,
      timestamp: Date.now(),
      message,
      args,
    });

    // Trim history if needed
    if (this.history.length > this.config.maxHistorySize) {
      this.history.shift();
    }

    const prefix = this.config.prefix ? `${this.config.prefix} ` : "";
    const fullMessage = `${prefix}${message}`;

    // Use custom logger if provided
    if (this.config.logger) {
      const customFn = this.config.logger[level];
      if (customFn) {
        customFn(fullMessage, ...args);
        return;
      }
    }

    // Use console
    const consoleFn =
      level === "error"
        ? console.error
        : level === "warn"
        ? console.warn
        : level === "info"
        ? console.info
        : console.log;

    if (args.length > 0) {
      consoleFn(fullMessage, ...args);
    } else {
      consoleFn(fullMessage);
    }
  }

  getHistory(): LogEntry[] {
    return [...this.history];
  }

  clearHistory(): void {
    this.history = [];
  }

  exportLogs(): string {
    return JSON.stringify(this.history, null, 2);
  }
}

export const logger = new Logger();

export function createLogger(config?: LoggingConfig): Logger {
  const newLogger = new Logger();
  if (config) {
    newLogger.configure(config);
  }
  return newLogger;
}
