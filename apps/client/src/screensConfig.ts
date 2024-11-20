// src/screensConfig.ts

interface Tool {
    icon: string;
    onClick: () => void;
  }
  
  interface ScreenConfig {
    tools: Tool[];
    rightPanel: string;
  }
  
  export const screensConfig: Record<string, ScreenConfig> = {
    dashboard: {
      tools: [
        { icon: "📋", onClick: () => console.log("Dashboard Tool 1") },
        { icon: "📊", onClick: () => console.log("Dashboard Tool 2") },
      ],
      rightPanel: "DashboardRightPanel",
    },
    settings: {
      tools: [
        { icon: "⚙️", onClick: () => console.log("Settings Tool 1") },
        { icon: "🔧", onClick: () => console.log("Settings Tool 2") },
      ],
      rightPanel: "SettingsRightPanel",
    },
    reports: {
      tools: [
        { icon: "📈", onClick: () => console.log("Reports Tool 1") },
        { icon: "📉", onClick: () => console.log("Reports Tool 2") },
      ],
      rightPanel: "ReportsRightPanel",
    },
  };
  