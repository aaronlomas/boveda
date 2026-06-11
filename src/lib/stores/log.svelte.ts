export type LogCategory = 
  | "INIT" 
  | "KDF" 
  | "MEM" 
  | "DECRYPT" 
  | "CIPHER" 
  | "SUCCESS" 
  | "ERROR" 
  | "WARN"
  | "NETWORK"
  | "AUTH"
  | "SYSTEM"
  | "EXPORT"
  | "IMPORT"
  | "WRITE";

export interface LogEntry {
  id: string;
  timestamp: string;
  category: LogCategory;
  text: string;
}

export class LogState {
  entries = $state<LogEntry[]>([]);
  maxEntries = 100;

  add(category: LogCategory, text: string) {
    const now = new Date();
    const timeString = now.toLocaleTimeString('en-US', { hour12: false });
    
    this.entries.push({
      id: crypto.randomUUID(),
      timestamp: timeString,
      category,
      text
    });

    if (this.entries.length > this.maxEntries) {
      this.entries.shift();
    }
  }

  clear() {
    this.entries = [];
  }
}

export const logStore = new LogState();

